use postgres::{
    Client,
    NoTls,
};

pub fn db_connection() -> Client {
    Client::connect(
        "postgresql://test_user:test_pass@localhost:9084/test",
        NoTls,
    )
    .unwrap()
}
