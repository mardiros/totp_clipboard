#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate clipboard;
extern crate libappindicator;
extern crate libreauth;
extern crate serde_json;

extern crate gtk;

mod applet;
mod seeds;

fn main() {
    pretty_env_logger::init();
    info!("Starting totp-clipboard");
    let sds = seeds::Seeds::from_file().unwrap_or(seeds::Seeds::new());
    let app = applet::Applet::new(sds);
    app.run()
}
