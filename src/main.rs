mod RenderFIle;
mod structs;
use crate::structs::*;
use crate::RenderFIle::*;
use std::{collections::HashMap, vec};
use minifb::{Key, Window, WindowOptions};
// Transform function for points in 3D space
// Function to create a world HashMap with objects
pub fn world_array() -> HashMap<String, Object> {
    let mut world = HashMap::new();
    let point1 = Point { x: 5.0, y: 10.0, z: 0.0 };
    let point2 = Point { x: 10.0, y: 10.0, z: 0.0 };
    let point3 = Point { x: 7.0, y: 10.0, z: 20.0 };
    let triangle = Triangle {
        p1: point1,
        p2: point2,
        p3: point3,
    };
    let obj1 = Object {
        name: String::from("wall1"),
        triangles: vec![triangle],
    };
    world.insert(obj1.name.clone(), obj1);
    world
}

pub fn main_update() {
    // Update game logic here
}

pub fn main() {
    let world = world_array();
    let mut pwindow=window{
        Height:1080.0,
        Width:1920.0,
    };
    let player = PlayerComp {
        playerx: 0.0,
        playery: 0.0,
        playerz: 20.0,
        playera: 0.0,
        playerfov: 200.0,
        window: pwindow,
    };  

    let mut window = Window::new(
        "TerraGameEngine",
        player.window.Width as usize,
        player.window.Height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    let mut framebuffer = vec![0; player.window.Height as usize * player.window.Width as usize];

    while window.is_open() {
        main_update();
        render(&world, &player,&mut framebuffer);
        window.update_with_buffer(&framebuffer, player.window.Width as usize, player.window.Height as usize).unwrap();
    }
}