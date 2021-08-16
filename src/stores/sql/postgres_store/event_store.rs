use std::{
    collections::HashMap,
    marker::PhantomData,
};

use postgres::Client;

use crate::{
    AggregateContext,
    AggregateError,
    EventContext,
    IAggregate,
    IEventStore,
};

use super::constants::{
    INSERT_EVENT,
    INSERT_SNAPSHOT,
    SELECT_EVENTS,
    SELECT_EVENTS_WITH_METADATA,
    SELECT_SNAPSHOT,
    UPDATE_SNAPSHOT,
};

/// Storage engine using an Postgres backing and relying on a
/// serialization of the aggregate rather than individual events. This
/// is similar to the "snapshot strategy" seen in many CQRS
/// frameworks.
pub struct EventStore<A: IAggregate> {
    conn: Client,
    with_snapshots: bool,
    _phantom: PhantomData<A>,
}

impl<A: IAggregate> EventStore<A> {
    /// Creates a new `EventStore` from the provided
    /// database connection.
    pub fn new(
        conn: Client,
        with_snapshots: bool,
    ) -> Self {
        EventStore {
            conn,
            with_snapshots,
            _phantom: PhantomData,
        }
    }

    fn load_aggregate_from_snapshot(
        &mut self,
        aggregate_id: &str,
    ) -> Result<AggregateContext<A>, AggregateError> {
        let agg_type = A::aggregate_type();
        let id = aggregate_id.to_string();

        let rows = match self
            .conn
            .query(SELECT_SNAPSHOT, &[&agg_type, &id])
        {
            Ok(x) => x,
            Err(_e) => {
                return Ok(AggregateContext {
                    aggregate_id: id,
                    aggregate: A::default(),
                    current_sequence: 0,
                });
            },
        };

        let row = match rows.iter().next() {
            None => {
                return Ok(AggregateContext {
                    aggregate_id: id,
                    aggregate: A::default(),
                    current_sequence: 0,
                });
            },
            Some(x) => x,
        };

        let s: i64 = row.get(0);
        let val = row.get(1);

        let aggregate = match serde_json::from_value(val) {
            Ok(x) => x,
            Err(e) => {
                return Err(AggregateError::new(
                    format!(
                        "bad payload found in events table for \
                         aggregate id {} with error: {}",
                        &id, e
                    )
                    .as_str(),
                ));
            },
        };

        Ok(AggregateContext {
            aggregate_id: id,
            aggregate: aggregate,
            current_sequence: s as usize,
        })
    }

    fn load_aggregate_from_events(
        &mut self,
        aggregate_id: &str,
    ) -> Result<AggregateContext<A>, AggregateError> {
        let id = aggregate_id.to_string();

        let events = match self.load_events(&id, false) {
            Ok(x) => x,
            Err(e) => {
                return Err(e);
            },
        };

        if events.len() == 0 {
            return Ok(AggregateContext {
                aggregate_id: id,
                aggregate: A::default(),
                current_sequence: 0,
            });
        }

        let mut aggregate = A::default();

        events
            .iter()
            .map(|x| &x.payload)
            .for_each(|x| aggregate.apply(&x));

        Ok(AggregateContext {
            aggregate_id: id,
            aggregate,
            current_sequence: events.last().unwrap().sequence,
        })
    }

    fn commit_with_snapshots(
        &mut self,
        events: Vec<A::Event>,
        context: AggregateContext<A>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventContext<A>>, AggregateError> {
        let mut updated_aggregate = context.aggregate.clone();

        let agg_type = A::aggregate_type().to_string();
        let aggregate_id = context.aggregate_id.as_str();
        let current_sequence = context.current_sequence;

        let wrapped_events = self.wrap_events(
            aggregate_id,
            current_sequence,
            events,
            metadata,
        );

        let mut trans = match self.conn.transaction() {
            Ok(x) => x,
            Err(e) => {
                return Err(AggregateError::TechnicalError(
                    e.to_string(),
                ));
            },
        };

        let mut last_sequence = current_sequence as i64;

        for event in wrapped_events.clone() {
            let id = context.aggregate_id.clone();
            let sequence = event.sequence as i64;
            last_sequence = sequence;

            let payload = match serde_json::to_value(&event.payload) {
                Ok(x) => x,
                Err(e) => {
                    panic!(
                        "bad payload found in events table for \
                         aggregate id {} with error: {}",
                        &id, e
                    );
                },
            };

            let metadata = match serde_json::to_value(&event.metadata)
            {
                Ok(x) => x,
                Err(e) => {
                    panic!(
                        "bad metadata found in events table for \
                         aggregate id {} with error: {}",
                        &id, e
                    );
                },
            };

            match trans.execute(
                INSERT_EVENT,
                &[
                    &agg_type, &id, &sequence, &payload, &metadata,
                ],
            ) {
                Ok(_) => {},
                Err(e) => {
                    match e.code() {
                        None => {},
                        Some(state) => {
                            if state.code() == "23505" {
                                return Err(
                                    AggregateError::TechnicalError(
                                        "optimistic lock error"
                                            .to_string(),
                                    ),
                                );
                            }
                        },
                    }
                    panic!(
                        "unable to insert event table for aggregate \
                         id {} with error: {}\n  and payload: {}",
                        &id, e, &payload
                    );
                },
            };

            updated_aggregate.apply(&event.payload);
        }

        let aggregate_payload =
            match serde_json::to_value(updated_aggregate) {
                Ok(x) => x,
                Err(e) => {
                    panic!(
                        "bad metadata found in events table for \
                         aggregate id {} with error: {}",
                        &aggregate_id, e
                    );
                },
            };

        if context.current_sequence == 0 {
            match trans.execute(
                INSERT_SNAPSHOT,
                &[
                    &agg_type,
                    &aggregate_id,
                    &last_sequence,
                    &aggregate_payload,
                ],
            ) {
                Ok(_) => {},
                Err(e) => {
                    panic!(
                        "unable to insert snapshot for aggregate id \
                         {} with error: {}\n  and payload: {}",
                        &aggregate_id, e, &aggregate_payload
                    );
                },
            };
        }
        else {
            match trans.execute(
                UPDATE_SNAPSHOT,
                &[
                    &agg_type,
                    &aggregate_id,
                    &last_sequence,
                    &aggregate_payload,
                ],
            ) {
                Ok(_) => {},
                Err(e) => {
                    panic!(
                        "unable to update snapshot for aggregate id \
                         {} with error: {}\n  and payload: {}",
                        &aggregate_id, e, &aggregate_payload
                    );
                },
            };
        }

        match trans.commit() {
            Ok(_) => Ok(wrapped_events),
            Err(e) => {
                Err(AggregateError::TechnicalError(
                    e.to_string(),
                ))
            },
        }
    }

    fn commit_events_only(
        &mut self,
        events: Vec<A::Event>,
        context: AggregateContext<A>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventContext<A>>, AggregateError> {
        let agg_type = A::aggregate_type().to_string();
        let id = context.aggregate_id.as_str();
        let current_sequence = context.current_sequence;

        let events =
            self.wrap_events(&id, current_sequence, events, metadata);

        let mut trans = match self.conn.transaction() {
            Ok(x) => x,
            Err(err) => {
                return Err(AggregateError::TechnicalError(
                    err.to_string(),
                ));
            },
        };

        for event in &events {
            let sequence = event.sequence as i64;

            let payload = match serde_json::to_value(&event.payload) {
                Ok(x) => x,
                Err(err) => {
                    return Err(AggregateError::new(
                        format!(
                            "Could not serialize the event payload \
                             for aggregate id {} with error: {}",
                            &id, err
                        )
                        .as_str(),
                    ));
                },
            };

            let metadata = match serde_json::to_value(&event.metadata)
            {
                Ok(x) => x,
                Err(err) => {
                    return Err(AggregateError::new(
                        format!(
                            "could not serialize the event metadata \
                             for aggregate id {} with error: {}",
                            &id, err
                        )
                        .as_str(),
                    ));
                },
            };

            match trans.execute(
                INSERT_EVENT,
                &[
                    &agg_type, &id, &sequence, &payload, &metadata,
                ],
            ) {
                Ok(_) => {},
                Err(err) => {
                    match err.code() {
                        None => {},
                        Some(state) => {
                            if state.code() == "23505" {
                                return Err(
                                    AggregateError::TechnicalError(
                                        "optimistic lock error"
                                            .to_string(),
                                    ),
                                );
                            }
                        },
                    }
                    return Err(AggregateError::TechnicalError(
                        format!(
                            "unable to insert event table for \
                             aggregate id {} with error: {}\n  and \
                             payload: {}",
                            &id, err, &payload
                        ),
                    ));
                },
            };
        }

        match trans.commit() {
            Ok(_) => Ok(events),
            Err(err) => {
                Err(AggregateError::TechnicalError(
                    err.to_string(),
                ))
            },
        }
    }
}

impl<A: IAggregate> IEventStore<A> for EventStore<A> {
    fn load_events(
        &mut self,
        aggregate_id: &str,
        with_metadata: bool,
    ) -> Result<Vec<EventContext<A>>, AggregateError> {
        let agg_type = A::aggregate_type();

        let sql = match with_metadata {
            true => SELECT_EVENTS_WITH_METADATA,
            false => SELECT_EVENTS,
        };

        let rows = match self
            .conn
            .query(sql, &[&agg_type, &aggregate_id])
        {
            Ok(x) => x,
            Err(_e) => {
                return Ok(Vec::new());
            },
        };

        let mut result = Vec::new();

        for row in rows.iter() {
            let sequence: i64 = row.get(0);

            let payload = match serde_json::from_value(row.get(1)) {
                Ok(x) => x,
                Err(e) => {
                    return Err(AggregateError::new(
                        format!(
                            "bad payload found in events table for \
                             aggregate id {} with error: {}",
                            &aggregate_id, e
                        )
                        .as_str(),
                    ));
                },
            };

            let metadata = match with_metadata {
                true => {
                    match serde_json::from_value(row.get(2)) {
                        Ok(x) => x,
                        Err(err) => {
                            return Err(AggregateError::new(
                                format!(
                                    "bad metadata found in events \
                                     table for aggregate id {} with \
                                     error: {}",
                                    &aggregate_id, err
                                )
                                .as_str(),
                            ));
                        },
                    }
                },
                false => Default::default(),
            };

            let event = EventContext {
                aggregate_id: aggregate_id.to_string(),
                sequence: sequence as usize,
                payload,
                metadata,
            };

            result.push(event);
        }

        Ok(result)
    }

    fn load_aggregate(
        &mut self,
        aggregate_id: &str,
    ) -> Result<AggregateContext<A>, AggregateError> {
        match self.with_snapshots {
            true => self.load_aggregate_from_snapshot(aggregate_id),
            false => self.load_aggregate_from_events(aggregate_id),
        }
    }

    fn commit(
        &mut self,
        events: Vec<A::Event>,
        context: AggregateContext<A>,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventContext<A>>, AggregateError> {
        match self.with_snapshots {
            true => {
                self.commit_with_snapshots(events, context, metadata)
            },
            false => {
                self.commit_events_only(events, context, metadata)
            },
        }
    }
}