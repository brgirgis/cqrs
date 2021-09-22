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

use super::handler_test_executor::HandlerResultExecutor;

/// `HandlerTester` provides a consistent way to test aggregate
/// implementations
///
/// # Examples
/// ```rust
/// use cqrs_es2::{
///     example_impl::{
///         AddCustomerName,
///         Customer,
///         CustomerCommand,
///         CustomerEvent,
///         NameAdded,
///     },
///     HandlerTester,
/// };
///
/// type CustomTester =
///     HandlerTester<CustomerCommand, CustomerEvent, Customer>;
///
/// CustomTester::default()
///     .given_no_previous_events()
///     .when(CustomerCommand::AddCustomerName(
///         AddCustomerName {
///             changed_name: "John Doe".to_string(),
///         },
///     ))
///     .then_expect(vec![CustomerEvent::NameAdded(
///         NameAdded {
///             changed_name: "John Doe".to_string(),
///         },
///     )]);
///
/// CustomTester::default()
///     .given(vec![CustomerEvent::NameAdded(
///         NameAdded {
///             changed_name: "John Doe".to_string(),
///         },
///     )])
///     .when(CustomerCommand::AddCustomerName(
///         AddCustomerName {
///             changed_name: "John Doe".to_string(),
///         },
///     ))
///     .then_expect_error(
///         "a name has already been added for this customer",
///     )
/// ```
pub struct HandlerTester<
    C: ICommand,
    E: IEvent,
    A: Default + ICommandHandler<C, E> + IEventHandler<E>,
> {
    _phantom: PhantomData<(C, E, A)>,
}

impl<
        C: ICommand,
        E: IEvent,
        A: Default + ICommandHandler<C, E> + IEventHandler<E>,
    > HandlerTester<C, E, A>
{
    /// Initiates a handler test with no previous events
    #[must_use]
    pub fn given_no_previous_events(
        &self
    ) -> HandlerResultExecutor<C, E, A> {
        HandlerResultExecutor::new(Vec::new())
    }

    /// Initiates a handler test with a collection of previous events
    #[must_use]
    pub fn given(
        &self,
        events: Vec<E>,
    ) -> HandlerResultExecutor<C, E, A> {
        HandlerResultExecutor::new(events)
    }
}

impl<
        C: ICommand,
        E: IEvent,
        A: Default + ICommandHandler<C, E> + IEventHandler<E>,
    > Default for HandlerTester<C, E, A>
{
    fn default() -> Self {
        HandlerTester {
            _phantom: PhantomData,
        }
    }
}
