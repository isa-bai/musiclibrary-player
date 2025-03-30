#![windows_subsystem = "windows"]

mod ui;
mod musiclib;
mod discord;


fn main() {

    let _ = ui::app::run();

}

