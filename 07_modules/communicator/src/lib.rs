
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {

    // This one is the preferred way:
    //use super::client;

    #[test]
    fn it_works() {
        // Leading :: to let Rust know to start from the root and list the
        // whole path.
        ::client::connect();
        // Super is to move up one module in the hierarchy.
        super::client::connect();
    }
}
