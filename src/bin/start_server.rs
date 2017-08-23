//! Binary for starting the actual web server.
//!
//! Since the main crate is a libary, we need this tiny executable crate.
extern crate luten;

pub fn main() {
    luten::start_server();
}
