use serde::{
    de::DeserializeOwned,
    Serialize,
};
use std::fmt::Debug;

use crate::{
    aggregates::Aggregate,
    events::EventEnvelope,
};

/// A `Query` is a read element in a CQRS system. As events are
/// emitted multiple downstream queries are updated to reflect the
/// current state of the system. A query may also be referred to as a
/// 'view', the concepts are identical but 'query' is used here to
/// conform with CQRS nomenclature.
///
/// Queries are generally serialized for persistence, usually in a
/// standard database, but a query could also utilize messaging
/// platform or other asynchronous, eventually-consistent systems.
pub trait Query<A: Aggregate>:
    Debug + Default + Serialize + DeserializeOwned {
    /// Each implemented query is responsible for updating its stated
    /// based on events passed via this method.
    fn update(
        &mut self,
        event: &EventEnvelope<A>,
    );
}