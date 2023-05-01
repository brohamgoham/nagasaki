mod asset;
mod database;
mod listener;
mod tui;

fn main() {
    listener::listener();
    tui::start_terminal().expect("Error trying to start the terminal");
}


