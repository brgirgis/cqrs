use crate::HandlerTester;

use super::{
    aggregate::Customer,
    commands::*,
    events::*,
};

type ThisHandlerTester =
    HandlerTester<CustomerCommand, CustomerEvent, Customer>;

#[test]
fn test_change_name() {
    ThisHandlerTester::default()
        .given_no_previous_events()
        .when(CustomerCommand::AddCustomerName(
            AddCustomerName {
                changed_name: "John Doe".to_string(),
            },
        ))
        .then_expect(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: "John Doe".to_string(),
            },
        )]);
}

#[test]
fn test_change_name_again() {
    ThisHandlerTester::default()
        .given(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: "John Doe".to_string(),
            },
        )])
        .when(CustomerCommand::AddCustomerName(
            AddCustomerName {
                changed_name: "John Doe".to_string(),
            },
        ))
        .then_expect_error(
            "a name has already been added for this customer",
        );
}
