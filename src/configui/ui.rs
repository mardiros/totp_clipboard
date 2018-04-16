use std::collections::HashMap;

use gdk::WindowTypeHint;
use gtk::{self, prelude::*};

use relm::{Component, ContainerWidget, Relm, Update, Widget};

use super::super::seeds::{Seed, Seeds};
use super::seed_editor::SeedEditor;

#[derive(Msg)]
pub enum Msg {
    AddingSeed(Seed),
    Saving,
    Quitting,
}

pub struct Model {
    seeds: Seeds,
}

pub struct Popup {
    relm: Relm<Popup>,
    window: gtk::Window,
    seeds_box: gtk::Box,
    seeds_editor: HashMap<String, Component<SeedEditor>>,
    model: Model,
}

impl Update for Popup {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        let sds = Seeds::from_file().unwrap_or(Seeds::new());
        Model { seeds: sds }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddingSeed(seed) => {
                info!("Adding Seed {:?}", seed);
                let seed_name = seed.name().to_owned();
                if !self.seeds_editor.contains_key(&seed_name) {
                    let editor = self.seeds_box.add_widget::<SeedEditor>(seed);
                    self.seeds_editor.insert(seed_name, editor);
                }
            }
            Msg::Saving => {
                self.model.seeds.safe_sync();
                self.relm.stream().emit(Msg::Quitting);
            }
            Msg::Quitting => {
                info!("Closing configuration");
                self.window.close();
                gtk::main_quit();
            }
        }
    }
}

impl Widget for Popup {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        info!("Starting configuration view");
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_type_hint(WindowTypeHint::Dialog);
        window.set_title("Configure totp-clipboard");

        let mainbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        mainbox.set_margin_left(20);
        mainbox.set_margin_right(20);
        mainbox.set_margin_top(20);
        let seeds_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        seeds_box.set_margin_bottom(20);
        mainbox.add(&seeds_box);

        let btn_box = gtk::Box::new(gtk::Orientation::Horizontal, 50);
        btn_box.set_margin_bottom(20);
        let btn = gtk::Button::new_with_label("New seed");
        btn_box.add(&btn);

        let btn = gtk::Button::new_with_label("Save & Exit");

        connect!(relm, btn, connect_clicked(_), Msg::Saving);

        btn_box.add(&btn);

        let btn = gtk::Button::new_with_label("Quit without saving");
        btn_box.add(&btn);

        connect!(relm, btn, connect_clicked(_), Msg::Quitting);

        mainbox.add(&btn_box);
        window.add(&mainbox);

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Msg::Quitting, Inhibit(false))
        );

        for seed in model.seeds.get_seeds() {
            info!("Emitting Adding {:?}", seed);
            relm.stream().emit(Msg::AddingSeed(seed.clone()));
        }

        window.show_all();
        Popup {
            relm: relm.clone(),
            window: window,
            seeds_box: seeds_box,
            model: model,
            seeds_editor: HashMap::new(),
        }
    }
}
