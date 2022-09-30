use zoon::*;

mod app;
mod ui;

fn main() {
    start_app("app", app::view::root);
}
