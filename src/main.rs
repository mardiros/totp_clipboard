#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate libreauth;
extern crate clipboard;
extern crate libappindicator;
extern crate serde_json;

extern crate gtk;
extern crate gtk_sys;


mod seeds;
mod applet;

fn main() {
    pretty_env_logger::init();
    info!("Starting totp-clipboard");
    let sds = seeds::Seeds::from_file().unwrap_or(seeds::Seeds::new());
    let app = applet::Applet::new(sds);
    app.run()
}
