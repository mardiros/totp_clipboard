#[macro_use]
extern crate log;
extern crate pretty_env_logger;


fn main() {
    pretty_env_logger::init();
    info!("Starting totp-clipboard");

}
