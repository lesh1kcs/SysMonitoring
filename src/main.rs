#![allow(unused)]

mod app;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use sysinfo::{System};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory : {} KB", sys.used_memory());

}

