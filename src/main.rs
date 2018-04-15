#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate clipboard;
extern crate libappindicator;
extern crate libreauth;
extern crate serde_json;

extern crate gdk;
extern crate gtk;

#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod applet;
mod configui;
mod seeds;

use relm::Widget;

fn main() {
    pretty_env_logger::init();
    info!("Starting totp-clipboard");
    let sds = seeds::Seeds::from_file().unwrap_or(seeds::Seeds::new());
    applet::Applet::run(sds).unwrap()
}
