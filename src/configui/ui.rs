use gdk::WindowTypeHint;
use gtk::{self, prelude::*};

use relm::{ContainerWidget, Relm, Update, Widget};

use super::super::seeds::Seed;
use super::seed_editor::SeedEditor;

#[derive(Msg)]
pub enum Msg {
    AddingSeed(Seed),
}

pub struct Model {
    seeds: Vec<Seed>,
}

pub struct Popup {
    window: gtk::Window,
    seeds_box: gtk::Box,
}

impl Update for Popup {
    type Model = Model;
    type ModelParam = Vec<Seed>;
    type Msg = Msg;

    fn model(_: &Relm<Self>, seeds: Vec<Seed>) -> Model {
        Model { seeds: seeds }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddingSeed(seed) => {
                self.seeds_box.add_widget::<SeedEditor>(seed);
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
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_type_hint(WindowTypeHint::Dialog);
        window.set_title("Configure totp-clipboard");

        for seed in model.seeds {
            relm.stream().emit(Msg::AddingSeed(seed.clone()));
        }
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
        btn_box.add(&btn);

        let btn = gtk::Button::new_with_label("Quit without saving");
        btn_box.add(&btn);

        mainbox.add(&btn_box);
        window.add(&mainbox);
        window.show_all();
        Popup {
            window: window,
            seeds_box: seeds_box,
        }
    }
}
