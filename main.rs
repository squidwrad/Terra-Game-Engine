use std::{collections::HashMap, vec};
use minifb::{Key, Window, WindowOptions};
pub struct window{
    Height: f64,
    Width: f64,
}
#[derive(Debug)]
struct Object {
    name: String,
    triangles: Vec<Triangle>,
}
#[derive(Debug)]
struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}
struct PlayerComp {
    playerx: f64,
    playery: f64,
    playerz: f64,
    playera: f64,
    playerfov: f64,
    window: window,

}

// Transform function for points in 3D space
fn transform(point: &Point, player: &PlayerComp) -> Point {
    let x = point.x;
    let y = point.y;
    let z = point.z;
    let newx = x - player.playerx;
    let newy = y - player.playery;
    let pi = std::f64::consts::PI;
    let cos = player.playera / (180.0 * pi);
    let sin = player.playera / (180.0 * pi);
    let tx = (newx * cos.cos()) - (newy * sin.sin());
    let ty = (newy * cos.cos()) + (newx * sin.sin());
    let tz = z - player.playerz;
    Point { x: tx, y: ty, z: tz }
}

// Project function for points in 3D space
fn project(tp: &Point, player: &PlayerComp) -> Point {
    let ox = tp.x;
    let mut oy = tp.y;
    let oz=tp.z;
    if oy == 0.0 {
        oy = 1.0;
    }
    let px = ox * player.playerfov / (oy) + player.window.Width / 2.0;
    let py = oz * player.playerfov / (-oy) + player.window.Height / 2.0;
    //println!("{},{},{}",px,py,oz);
    Point {x:px,y:py,z:oz}
}
fn draw(projected:&HashMap<String, Object>,frame_buffer:&mut Vec<u32>,player: &PlayerComp){
    for (obj_name, obj) in projected.iter(){
        let mut singleslope:f64=0.0;
        let mut slope1:f64=0.0;
        let mut slope2:f64=0.0;
        for tri in &obj.triangles {
            let mut point_array=[
                &tri.p1,&tri.p2,&tri.p3,
            ];
            point_array.sort_by(|a,b|a.x.partial_cmp(&b.x).unwrap());
            let mut startx=point_array[0].x;
            let mut starty=point_array[0].y;
            let mut endy=point_array[0].y;
            let mut endx=point_array[2].x;
            let mut midy=point_array[1].y;
            let mut midx=point_array[1].x;
            println!("{:?}",point_array);
            if startx<0.0{
                startx=0.0;
            }
            if startx>player.window.Width{
                startx=1920.0;
            }
            if endx>player.window.Width{
                endx=1920.0;
            }
            if endx<0.0{
                endx=0.0;
            }
            if endx-midx==0.0{
                slope2=0.0;
            }
            else{
                slope2=(endy-midy)/(endx-midx);
            }
            if midx-startx==0.0{
                slope1=0.0;
            }
            else{
                slope1=(midy-starty)/(midx-startx);
            }
            if endx-startx==0.0{
                singleslope=0.0;
            }
            else{
                singleslope=(endy-starty)/(endx-startx);
            }
            println!("{},{},{}",singleslope,slope1,slope2);
            println!("{},{},{},{},{},{}",startx as usize,endx as usize,starty as usize, endy as usize,midx as usize,midy as usize);
            for x in (startx)as usize..(endx+1.0)as usize{
                if starty<0.0{
                    starty=0.0;
                }
                if starty>player.window.Height{
                    starty=1080.0;
                }
                if endy>player.window.Height{
                    endy=1080.0;
                }
                if endy<0.0{
                    endy=0.0;
                }
                let mut yrange=[(starty)as usize,(endy)as usize];
                yrange.sort();
                //println!("x={}",x);
                for y in yrange[0]..yrange[1]+1{
                    //println!("y={},{}",yrange[0],yrange[1]);
                    let mut index=y*1920+x;
                    frame_buffer[index]=0xFF0000;
                }
                if x as f64<=midx{
                    starty+=slope1;
                }
                else{
                    midy+=slope2;
                    starty=midy;
                }
                endy=endy+singleslope;
                
            } 
            //println!("{:?}",point_array);
            //println!("{},{},{}",slope1,slope2,singleslope);
        }
    }
}
// Function to create a world HashMap with objects
fn world_array() -> HashMap<String, Object> {
    let mut world = HashMap::new();
    let point1 = Point { x: 5.0, y: 10.0, z: 0.0 };
    let point2 = Point { x: 10.0, y: 10.0, z: 0.0 };
    let point3 = Point { x: 20.0, y: 10.0, z: 20.0 };
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

fn main_update() {
    // Update game logic here
}

// Render function to iterate through the world HashMap and apply transformations
fn render(world: &HashMap<String, Object>, player: &PlayerComp,frame_buffer:&mut Vec<u32>) {
    let mut translated: HashMap<String, Object> = HashMap::new();
    let mut projected: HashMap<String, Object> = HashMap::new();
    for (obj_name, obj) in world.iter() {
        let trans_obj = Object {
            name: obj_name.to_string(),
            triangles: vec![],
        };
        let proj_obj = Object {
            name: obj_name.to_string(),
            triangles: vec![],
        };
        translated.insert(obj_name.to_string(), trans_obj);
        projected.insert(obj_name.to_string(), proj_obj);

        for tri in &obj.triangles {
            let p1 = &tri.p1;
            let tp1 = transform(&p1, &player);
            let pp1 = project(&tp1, &player);
            //println!("{:?}",pp1);
            let p2 = &tri.p2;
            let tp2 = transform(&p2, &player);
            let pp2 = project(&tp2, &player);
            //println!("{:?}",pp2);
            let p3 = &tri.p3;
            let tp3 = transform(&p3, &player);
            let pp3 = project(&tp3, &player);
            //println!("{:?}",pp3);
            let t_triangle = Triangle {
                p1: tp1,
                p2: tp2,
                p3: tp3,
            };

            let p_triangle= Triangle{
                p1:pp1,
                p2:pp2,
                p3:pp3,
            };
            translated
                .get_mut(obj_name)
                .unwrap()
                .triangles
                .push(t_triangle);
            projected
                .get_mut(obj_name)
                .unwrap()
                .triangles
                .push(p_triangle);
        }
    }
    draw(&projected,frame_buffer,&player);
}

fn main() {
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
        "Test - Esc to Exit",
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