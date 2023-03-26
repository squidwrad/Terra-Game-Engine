
use std::{collections::HashMap, vec};
use minifb::{Key, Window, WindowOptions};
pub struct window{
    pub Height: f64,
    pub Width: f64,
}
#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub triangles: Vec<Triangle>,
}
#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub struct PlayerComp {
    pub playerx: f64,
    pub playery: f64,
    pub playerz: f64,
    pub playera: f64,
    pub playerfov: f64,
    pub window: window,

}
