use gtk::{self, prelude::*};

use relm::{Relm, Update, Widget};

use super::super::seeds::Seed;

#[derive(Msg)]
pub enum Msg {
}

pub struct Model {
    seed: Seed,
}

pub struct SeedEditor {
    hbox: gtk::Box,
}

impl Update for SeedEditor {
    type Model = Model;
    type ModelParam = Seed;
    type Msg = Msg;

    fn model(_: &Relm<Self>, seed: Seed) -> Model {
        Model { seed: seed }
    }

    fn update(&mut self, event: Msg) {
        match event {}
    }
}

impl Widget for SeedEditor {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.hbox.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        let label = gtk::Label::new("Name:");
        hbox.add(&label);

        let entry = gtk::Entry::new();
        entry.set_text(model.seed.name());
        entry.set_can_focus(true);
        hbox.add(&entry);

        let label = gtk::Label::new("Seed:");
        hbox.add(&label);

        let entry = gtk::Entry::new();
        entry.set_text(model.seed.seed());
        entry.set_can_focus(true);
        hbox.add(&entry);

        let btn = gtk::Button::new_with_label("Remove");
        hbox.add(&btn);

        hbox.show_all();
        SeedEditor { hbox: hbox }
    }
}
