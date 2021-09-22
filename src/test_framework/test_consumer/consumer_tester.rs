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

use super::consumer_test_executor::ConsumerResultExecutor;

/// `ConsumerTester` provides a consistent way to test query
/// implementations
///
/// # Examples
/// ```rust
/// use cqrs_es2::{
///     example_impl::{
///         CustomerCommand,
///         CustomerContactQuery,
///         CustomerEvent,
///         EmailUpdated,
///         NameAdded,
///     },
///     ConsumerTester,
///     EventContext,
/// };
///
/// type CustomTester = ConsumerTester<
///     CustomerCommand,
///     CustomerEvent,
///     CustomerContactQuery,
/// >;
///
/// CustomTester::default()
///     .given_no_previous_state()
///     .when(&EventContext::new(
///         "".to_string(),
///         0,
///         CustomerEvent::NameAdded(NameAdded {
///             changed_name: "John Doe".to_string(),
///         }),
///         Default::default(),
///     ))
///     .then_expect(CustomerContactQuery {
///         name: "John Doe".to_string(),
///         email: "".to_string(),
///         latest_address: "".to_string(),
///     });
///
/// CustomTester::default()
///     .given(CustomerContactQuery {
///         name: "John Doe".to_string(),
///         email: "".to_string(),
///         latest_address: "".to_string(),
///     })
///     .when(&EventContext::new(
///         "".to_string(),
///         0,
///         CustomerEvent::EmailUpdated(EmailUpdated {
///             new_email: "j@d.com".to_string(),
///         }),
///         Default::default(),
///     ))
///     .then_expect(CustomerContactQuery {
///         name: "John Doe".to_string(),
///         email: "j@d.com".to_string(),
///         latest_address: "".to_string(),
///     });
/// ```
pub struct ConsumerTester<
    C: ICommand,
    E: IEvent,
    Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
> {
    _phantom: PhantomData<(C, E, Q)>,
}

impl<
        C: ICommand,
        E: IEvent,
        Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
    > ConsumerTester<C, E, Q>
{
    /// Initiates a projection test with no previous state
    #[must_use]
    pub fn given_no_previous_state(
        &self
    ) -> ConsumerResultExecutor<C, E, Q> {
        ConsumerResultExecutor::default()
    }

    /// Initiates a projection test with a previous state
    #[must_use]
    pub fn given(
        &self,
        handler: Q,
    ) -> ConsumerResultExecutor<C, E, Q> {
        ConsumerResultExecutor::new(handler)
    }
}

impl<
        C: ICommand,
        E: IEvent,
        Q: Debug + Default + Clone + PartialEq + IEventConsumer<C, E>,
    > Default for ConsumerTester<C, E, Q>
{
    fn default() -> Self {
        ConsumerTester {
            _phantom: PhantomData,
        }
    }
}
