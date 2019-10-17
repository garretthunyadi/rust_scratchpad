use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

pub fn main() {
    println!("specs.here");
    // use specs::{Builder, World};
    use specs::World;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
}

use specs::System;

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {}
}
