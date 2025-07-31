#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ui;
mod musiclib;
mod discord;
mod config;
mod websocket;

fn main() {
    ui::app::run();
}