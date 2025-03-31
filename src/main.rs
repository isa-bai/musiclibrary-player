//#![windows_subsystem = "windows"]

mod ui;
mod musiclib;
mod discord;
mod config;


fn main() {
    let cfg = &*config::PROGRAM_CFG;
    println!("{:?}", cfg);
    //return;

    let _ = ui::app::run();

}

