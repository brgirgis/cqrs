use crate::{
    example_impl::*,
    EventContext,
};

use super::consumer_tester::ConsumerTester;

type ThisTester = ConsumerTester<
    CustomerCommand,
    CustomerEvent,
    CustomerContactQuery,
>;

#[test]
fn test_consumer_tester() {
    ThisTester::default()
        .given_no_previous_state()
        .when(&EventContext::new(
            "".to_string(),
            0,
            CustomerEvent::NameAdded(NameAdded {
                changed_name: "John Doe".to_string(),
            }),
            Default::default(),
        ))
        .then_expect(CustomerContactQuery {
            name: "John Doe".to_string(),
            email: "".to_string(),
            latest_address: "".to_string(),
        });

    ThisTester::default()
        .given(CustomerContactQuery {
            name: "John Doe".to_string(),
            email: "".to_string(),
            latest_address: "".to_string(),
        })
        .when(&EventContext::new(
            "".to_string(),
            0,
            CustomerEvent::EmailUpdated(EmailUpdated {
                new_email: "j@d.com".to_string(),
            }),
            Default::default(),
        ))
        .then_expect(CustomerContactQuery {
            name: "John Doe".to_string(),
            email: "j@d.com".to_string(),
            latest_address: "".to_string(),
        });
}
