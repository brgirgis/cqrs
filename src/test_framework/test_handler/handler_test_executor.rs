use std::marker::PhantomData;

use crate::{
    commands::{
        ICommand,
        ICommandHandler,
    },
    events::{
        IEvent,
        IEventHandler,
    },
};

use super::handler_result_validator::HandlerResultValidator;

/// Holds the initial event state and accepts a command
pub struct HandlerResultExecutor<
    C: ICommand,
    E: IEvent,
    A: Default + ICommandHandler<C, E> + IEventHandler<E>,
> {
    pub events: Vec<E>,
    _phantom: PhantomData<(C, A)>,
}

impl<
        C: ICommand,
        E: IEvent,
        A: Default + ICommandHandler<C, E> + IEventHandler<E>,
    > HandlerResultExecutor<C, E, A>
{
    pub fn new(events: Vec<E>) -> Self {
        Self {
            events,
            _phantom: PhantomData,
        }
    }

    /// Consumes a command and using the state details previously
    /// passed provides a validator object to test against
    pub fn when(
        self,
        command: C,
    ) -> HandlerResultValidator<E> {
        let mut handler = A::default();

        for event in self.events {
            handler.apply(&event);
        }

        let result = handler.handle(command);

        HandlerResultValidator::new(result)
    }
}
