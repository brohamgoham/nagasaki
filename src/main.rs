mod asset;
mod database;
mod tui;
mod listener;

fn main() {
    listener::listener();
    tui::start_terminal()
        .expect("Error trying to start the terminal");
}
