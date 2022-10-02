use zoon::*;

mod app;
mod cipher;
mod ui;

fn main() {
    start_app("app", || app::App::new().root());
}
