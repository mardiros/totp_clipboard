use gtk::{self, prelude::*};

use relm::{Relm, Update, Widget};

use super::super::seeds::Seed;

#[derive(Msg)]
pub enum Msg {
    SeedFocusLost,
    SeedUpdated(String, String),
}

pub struct Model {
    seed: Seed,
}

pub struct SeedEditor {
    relm: Relm<SeedEditor>,
    hbox: gtk::Box,
    seed_entry: gtk::Entry,
    model: Model,
}

impl Update for SeedEditor {
    type Model = Model;
    type ModelParam = Seed;
    type Msg = Msg;

    fn model(_: &Relm<Self>, seed: Seed) -> Model {
        Model { seed: seed }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SeedFocusLost => {
                let seed = self.seed_entry.get_text().unwrap();
                info!("Focus lost on {:?}", self.model.seed.name());
                if self.model.seed.seed() != seed {
                    info!("Seed updated {}", self.model.seed.name());
                    self.model.seed.set_seed(&seed);
                    self.relm.stream().emit(Msg::SeedUpdated(
                        self.model.seed.name().to_owned(),
                        seed.to_owned(),
                    ));
                }
            }
            _ => {}
        }
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

        let name_entry = gtk::Entry::new();
        name_entry.set_text(model.seed.name());
        name_entry.set_can_focus(true);
        hbox.add(&name_entry);

        let label = gtk::Label::new("Seed:");
        hbox.add(&label);

        let seed_entry = gtk::Entry::new();
        seed_entry.set_text(model.seed.seed());
        seed_entry.set_can_focus(true);
        connect!(
            relm,
            seed_entry,
            connect_focus_out_event(_, _),
            return (Msg::SeedFocusLost, Inhibit(false))
        );

        hbox.add(&seed_entry);

        let btn = gtk::Button::new_with_label("Remove");
        hbox.add(&btn);

        hbox.show_all();
        SeedEditor {
            relm: relm.clone(),
            hbox: hbox,
            seed_entry: seed_entry,
            model: model,
        }
    }
}
