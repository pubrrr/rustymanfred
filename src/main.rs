mod component;

use crate::component::manfred::Manfred;
use crate::component::Position;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_startup_system(add_manf.system())
        .add_system(hello_system.system())
        .add_system(component.system())
        .run();
}

fn hello_system() {
    println!("hi");
}

fn add_manf(mut commands: Commands) {
    commands.spawn().insert(Manfred).insert(Position::new(0, 1));
}

fn component(query: Query<&Position, With<Manfred>>) {
    println!("query arrived: {}", query.iter().count());
    query.for_each(|position| println!("manfred at ({},{})", position.x, position.y));
}
