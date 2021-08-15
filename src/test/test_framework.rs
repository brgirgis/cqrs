use std::marker::PhantomData;

use crate::aggregates::IAggregate;

use super::aggregate_test_executor::AggregateTestExecutor;

/// A framework for rigorously testing the aggregate logic, one of the
/// ***most important*** parts of any CQRS system.
///
/// ```rust
/// use cqrs_es2::test::{
///     customers::{
///         AddCustomerName,
///         Customer,
///         CustomerCommand,
///         CustomerEvent,
///         NameAdded,
///     },
///     TestFramework,
/// };
/// type CustomerTestFramework = TestFramework<Customer>;
///
/// CustomerTestFramework::default()
///     .given_no_previous_events()
///     .when(CustomerCommand::AddCustomerName(
///         AddCustomerName {
///             changed_name: "John Doe".to_string(),
///         },
///     ))
///     .then_expect_events(vec![CustomerEvent::NameAdded(
///         NameAdded {
///             changed_name: "John Doe".to_string(),
///         },
///     )]);
///
/// CustomerTestFramework::default()
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
pub struct TestFramework<A: IAggregate> {
    _phantom: PhantomData<A>,
}

impl<A: IAggregate> TestFramework<A> {
    /// Initiates an aggregate test with no previous events.
    #[must_use]
    pub fn given_no_previous_events(
        &self
    ) -> AggregateTestExecutor<A> {
        AggregateTestExecutor { events: Vec::new() }
    }

    /// Initiates an aggregate test with a collection of previous
    /// events.
    #[must_use]
    pub fn given(
        &self,
        events: Vec<A::Event>,
    ) -> AggregateTestExecutor<A> {
        AggregateTestExecutor { events }
    }
}

impl<A: IAggregate> Default for TestFramework<A> {
    fn default() -> Self {
        TestFramework {
            _phantom: PhantomData,
        }
    }
}
