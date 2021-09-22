use std::{
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    commands::ICommand,
    events::{
        IEvent,
        IEventConsumer,
    },
};

/// Validation object for the `ConsumerTester`
pub struct ConsumerResultValidator<
    C: ICommand,
    E: IEvent,
    Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
> {
    result: Q,
    _phantom: PhantomData<(C, E)>,
}

impl<
        C: ICommand,
        E: IEvent,
        Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
    > ConsumerResultValidator<C, E, Q>
{
    pub fn new(result: Q) -> Self {
        Self {
            result,
            _phantom: PhantomData,
        }
    }

    /// Verifies that the expected projection has been produced by
    /// consumer handler
    pub fn then_expect(
        self,
        expected: Q,
    ) {
        assert_eq!(&self.result, &expected);
    }
}
