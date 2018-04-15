use gtk::{self, MenuItemExt, MenuShellExt, WidgetExt};

use clipboard::{ClipboardContext, ClipboardProvider};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use relm::{Relm, Update, Widget};

use super::super::seeds::{Seed, Seeds};


#[derive(Msg)]
pub enum Msg {
    SettingClipboardContent(Seed),
}


pub struct Model {
    seeds: Seeds,
}


pub struct Applet {
    menu: gtk::Menu
}


impl Update for Applet {
    type Model = Model;
    type ModelParam = Seeds;
    type Msg = Msg;

    fn model(_: &Relm<Self>, seeds: Seeds) -> Model {
        Model { seeds: seeds }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SettingClipboardContent(seed) => {
                let code = seed.code();
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                clipboard.set_contents(code).unwrap();
            }
        }
    }
}


impl Widget for Applet {

    type Root = gtk::Menu;

    fn root(&self) -> Self::Root {
        self.menu.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {

        let mut indicator = AppIndicator::new("totp-clipboard", "");

        indicator.set_icon_full(
            "/usr/share/icons/Adwaita/48x48/status/dialog-password.png",
            "icon",
        );
        let mut m = gtk::Menu::new();

        for seed in model.seeds.get_seeds() {
            let mi = gtk::MenuItem::new_with_label(seed.name());

            connect!(
                relm,
                mi,
                connect_activate(_),
                Msg::SettingClipboardContent(seed.clone())
            );

            m.append(&mi);
        }

        indicator.set_menu(&mut m);
        m.show_all();
        indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);
        Applet {menu: m}
    }
}
