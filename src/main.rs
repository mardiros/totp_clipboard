#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate libreauth;
extern crate clipboard;
extern crate libappindicator;
extern crate serde_json;

mod seeds;

fn main() {
    pretty_env_logger::init();
    info!("Starting totp-clipboard");
    let sds = seeds::Seeds::from_file().unwrap_or(seeds::Seeds::new());
    debug!("{:?}", sds);
    debug!("{:?}", sds.get_names());
    debug!("{:?}", sds.get_code("github.com"));
}
