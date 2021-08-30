use log::trace;
use std::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    commands::ICommand,
    events::IEvent,
};

use super::i_aggregate::IAggregate;

/// Returns the aggregate and context around it that is needed when
/// committing events in an event store implementation.
#[derive(Debug, PartialEq, Clone)]
pub struct AggregateContext<
    C: ICommand,
    E: IEvent,
    A: IAggregate<C, E>,
> {
    /// The aggregate ID of the aggregate instance that has been
    /// loaded.
    pub aggregate_id: String,

    /// The current version number for this aggregate instance.
    pub version: i64,

    /// The current state of the aggregate instance.
    pub payload: A,

    _phantom: PhantomData<(C, E)>,
}

impl<C: ICommand, E: IEvent, A: IAggregate<C, E>>
    AggregateContext<C, E, A>
{
    /// Constructor
    pub fn new(
        aggregate_id: String,
        version: i64,
        payload: A,
    ) -> Self {
        let x = Self {
            aggregate_id,
            version,
            payload,
            _phantom: PhantomData,
        };

        trace!("Created new {:?}", x,);

        x
    }
}
