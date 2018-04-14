use gtk::{self, MenuItemExt, MenuShellExt, WidgetExt};

use clipboard::{ClipboardContext, ClipboardProvider};
use libappindicator::{AppIndicator, AppIndicatorStatus};

use super::super::seeds::Seeds;

pub struct Applet {
    seeds: Seeds,
}

impl Applet {
    pub fn new(seeds: Seeds) -> Self {
        Applet { seeds: seeds }
    }

    pub fn run(&self) {
        gtk::init().unwrap();
        let mut indicator = AppIndicator::new("totp-clipboard", "");

        indicator.set_icon_full(
            "/usr/share/icons/Adwaita/48x48/status/dialog-password.png",
            "icon",
        );
        let mut m = gtk::Menu::new();

        for seed in self.seeds.get_seeds() {
            let mi = gtk::MenuItem::new_with_label(seed.name());

            mi.connect_activate(move |_| {
                let code = seed.code();
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                clipboard.set_contents(code).unwrap();
            });

            m.append(&mi);
        }

        indicator.set_menu(&mut m);
        m.show_all();
        indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        gtk::main();
    }
}
