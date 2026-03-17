use bevy::app::App;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, hello_wolrd)
        .run();
}

fn hello_wolrd() {
    println!("Hello, world!");
}