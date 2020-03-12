mod applet;
mod seeds;

fn main() {
    let sds = seeds::Seeds::from_file().unwrap_or(seeds::Seeds::new());
    let app = applet::Applet::new(sds);
    app.run()
}
