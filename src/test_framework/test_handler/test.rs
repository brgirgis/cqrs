use crate::example_impl::*;

use super::handler_tester::HandlerTester;

type ThisTester =
    HandlerTester<CustomerCommand, CustomerEvent, Customer>;

#[test]
fn test_handler_tester() {
    let test_name = "test A";
    let test_framework = ThisTester::default();

    test_framework
        .given(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: "test_id_A".to_string(),
            },
        )])
        .when(CustomerCommand::UpdateEmail(
            UpdateEmail {
                new_email: test_name.to_string(),
            },
        ))
        .then_expect(vec![CustomerEvent::EmailUpdated(
            EmailUpdated {
                new_email: test_name.to_string(),
            },
        )]);

    test_framework
        .given(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: test_name.to_string(),
            },
        )])
        .when(CustomerCommand::AddCustomerName(
            AddCustomerName {
                changed_name: test_name.to_string(),
            },
        ))
        .then_expect_error(
            "a name has already been added for this customer",
        )
}

#[test]
#[should_panic]
fn test_handler_tester_failure_test_a() {
    let test_name = "test A";
    let test_framework = ThisTester::default();

    test_framework
        .given(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: test_name.to_string(),
            },
        )])
        .when(CustomerCommand::AddCustomerName(
            AddCustomerName {
                changed_name: test_name.to_string(),
            },
        ))
        .then_expect(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: test_name.to_string(),
            },
        )]);
}

#[test]
#[should_panic]
fn test_handler_tester_failure_test_b() {
    let test_name = "test A";
    let test_framework = ThisTester::default();

    test_framework
        .given(vec![CustomerEvent::NameAdded(
            NameAdded {
                changed_name: "test_id_A".to_string(),
            },
        )])
        .when(CustomerCommand::UpdateEmail(
            UpdateEmail {
                new_email: test_name.to_string(),
            },
        ))
        .then_expect_error("some error message")
}
