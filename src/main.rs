use bevy::prelude::*;

fn main() {
    App::build()
        .add_system(hello_system.system())
        .run();
}

fn hello_system() {
    println!("hi");
}