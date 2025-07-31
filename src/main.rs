#![windows_subsystem = "windows"]

use std::ops::Deref;

mod ui;
mod musiclib;
mod discord;
mod config;
mod websocket;


fn main() {
    let cfg = config::PROGRAM_CFG.deref();
    println!("{:?}", cfg);
    //return;

    let _ = ui::app::run();

}

