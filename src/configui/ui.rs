use std::collections::HashMap;

use gdk::WindowTypeHint;
use gtk::{self, prelude::*};

use relm::{Component, ContainerWidget, Relm, Update, Widget};

use super::super::seeds::{Seed, SeedMap, SeedMapIO, Seeds};
use super::seed_editor::{Msg as MsgSeedEditor, SeedEditor};

#[derive(Msg)]
pub enum Msg {
    AddingSeed(Seed),
    SeedUpdated(String, String),
    Saving,
    Quitting,
}

pub struct Model {
    seedmap: SeedMap,
}

impl Model {
    pub fn set_name(&mut self, oldname: &str, newname: &str) {
        let seed = self.seedmap.remove(oldname).unwrap(); // crash if ?
        self.seedmap.insert(newname.to_owned(), seed);
    }

    pub fn set_seed(&mut self, name: &str, newseed: &str) {
        if let Some(seed) = self.seedmap.get_mut(name) {
            *seed = newseed.to_owned();
        }
        else {
            error!("Not seed {} in the seedmap", name);
        }

    }

    pub fn to_seeds(&self) -> Vec<Seed> {
        Seeds::from_seedmap(&self.seedmap).get_seeds().to_vec()
    }

    pub fn sync(&self) {
        error!("Implement saving seeds to file: {:?}", self.seedmap);
        self.seedmap.to_file().unwrap_or_else(|err| {
            error!{"Seeds not synchronized: {}", err}
        });
    }
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
        let map = SeedMap::from_file().unwrap(); // crash if the file does not exists
        Model { seedmap: map }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddingSeed(seed) => {
                info!("Adding Seed {:?}", seed);
                let seed_name = seed.name().to_owned();
                if !self.seeds_editor.contains_key(&seed_name) {
                    let editor = self.seeds_box.add_widget::<SeedEditor>(seed);
                    connect!(
                        editor@MsgSeedEditor::SeedUpdated(ref name, ref seed),
                        self.relm,
                        Msg::SeedUpdated(name.to_owned(), seed.to_owned())
                    );

                    self.seeds_editor.insert(seed_name, editor);
                }
            }
            Msg::Saving => {
                self.model.sync();
                self.relm.stream().emit(Msg::Quitting);
            }
            Msg::Quitting => {
                info!("Closing configuration");
                self.window.close();
                gtk::main_quit();
            }
            Msg::SeedUpdated(name, seed) => {
                error!("Updating seed of {}", name);
                self.model.set_seed(name.as_str(), seed.as_str());
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

        for seed in model.to_seeds() {
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
