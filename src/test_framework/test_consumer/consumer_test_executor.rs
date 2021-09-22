use std::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    commands::ICommand,
    events::{
        EventContext,
        IEvent,
        IEventConsumer,
    },
};

use super::consumer_result_validator::ConsumerResultValidator;

/// Holds the initial state of a projection and accepts an event
pub struct ConsumerResultExecutor<
    C: ICommand,
    E: IEvent,
    Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
> {
    pub handler: Option<Q>,
    _phantom: PhantomData<(C, E)>,
}

impl<
        C: ICommand,
        E: IEvent,
        Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
    > Default for ConsumerResultExecutor<C, E, Q>
{
    fn default() -> Self {
        Self {
            handler: None,
            _phantom: PhantomData,
        }
    }
}

impl<
        C: ICommand,
        E: IEvent,
        Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
    > ConsumerResultExecutor<C, E, Q>
{
    pub fn new(handler: Q) -> Self {
        Self {
            handler: Some(handler),
            _phantom: PhantomData,
        }
    }

    /// Consumes an event using the state details previously
    /// passed and provides a validator object to test against
    pub fn when(
        self,
        event: &EventContext<C, E>,
    ) -> ConsumerResultValidator<C, E, Q> {
        let mut handler = self.handler.unwrap_or_default();

        handler.update(event);

        ConsumerResultValidator::new(handler)
    }
}
