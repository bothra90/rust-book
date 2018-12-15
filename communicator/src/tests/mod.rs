#[test]
fn it_works() {
    // This will not work because paths are relative to the current crate by default, except in use
    // statements
    // client::connect;
    // All of the following work
    {
        use crate::client::connect; // Paths in "use" are relative to the crate root by default.
        connect();
    }
    crate::client::connect();
    super::client::connect();
    {
        use super::client;
        client::connect();
    }
}
