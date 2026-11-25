#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod claude;
mod config;
mod gui;
mod presence;
mod service;

fn main() {
    let isService = std::env::args().any(|argument| argument == service::Launcher::serviceFlag);
    if isService {
        service::Worker::run();
    } else {
        gui::Launcher::run();
    }
}
