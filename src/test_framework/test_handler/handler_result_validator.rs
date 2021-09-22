use crate::{
    errors::Error,
    events::IEvent,
};

/// Validation object for the `HandlerTester`
pub struct HandlerResultValidator<E: IEvent> {
    result: Result<Vec<E>, Error>,
}

impl<E: IEvent> HandlerResultValidator<E> {
    pub fn new(result: Result<Vec<E>, Error>) -> Self {
        Self { result }
    }

    /// Verifies that the expected events have been produced by the
    /// command handler
    pub fn then_expect(
        self,
        expected: Vec<E>,
    ) {
        let events = match self.result {
            Ok(x) => x,
            Err(e) => {
                panic!(
                    "expected success, received error: '{}'",
                    e
                );
            },
        };
        assert_eq!(&events[..], &expected[..]);
    }

    /// Verifies that an `Error` with the expected message is
    /// produced from the command handler
    pub fn then_expect_error(
        self,
        error_message: &str,
    ) {
        let err = match self.result {
            Ok(events) => {
                panic!(
                    "expected error, received events: '{:?}'",
                    events
                );
            },
            Err(e) => e,
        };

        match err {
            Error::TechnicalError(e) => {
                panic!(
                    "expected user error but found technical error: \
                     {}",
                    e
                )
            },
            Error::UserError(e) => {
                assert_eq!(
                    e.message,
                    Some(error_message.to_string())
                );
            },
        }
    }
}
