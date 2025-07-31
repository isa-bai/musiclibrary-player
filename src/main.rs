#![windows_subsystem = "windows"]

mod ui;
mod musiclib;
mod discord;
mod config;
mod websocket;


fn main() {
    let _ = ui::app::run();
}

