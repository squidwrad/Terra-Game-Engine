use std::{collections::HashMap, vec};
use minifb::{Key, Window, WindowOptions};
use crate::*;
pub fn transform(point: &Point, player: &PlayerComp) -> Point {
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
pub fn project(tp: &Point, player: &PlayerComp) -> Point {
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
pub fn pixeldraw(){
    
}
pub fn edgemath(point1:&Point,point2: &Point,x:&i64,y:&i64)->i64{
    let mut abx=(point2.x as i64)-(point1.x as i64);
    let mut aby=(point2.y as i64)-(point1.y as i64);
    let mut apx=x-(point1.x as i64);
    let mut apy=y-(point1.y as i64);
    return (abx*apy)-(aby*apx);
}
pub fn pixeliter(projected:&HashMap<String, Object>,frame_buffer:&mut Vec<u32>,player: &PlayerComp){
    for (obj_name, obj) in projected.iter(){
        for tri in &obj.triangles {
            let mut point_array=[
                &tri.p1,&tri.p2,&tri.p3,
            ];
            point_array.sort_by(|a,b|a.x.partial_cmp(&b.y).unwrap());
            let mut miny=(point_array[0].y).floor();
            if miny<0.0{
                miny=0.0;
            }
            let mut maxy=(point_array[2].y).ceil();
            if maxy>(player.window.Height){
                maxy=player.window.Height;
            }
            point_array.sort_by(|a,b|a.x.partial_cmp(&b.x).unwrap());
            let mut minx=(point_array[0].x).floor();
            if minx<0.0{
                minx=0.0;
            }
            let mut maxx=(point_array[2].x).ceil();
            if maxx>(player.window.Width){
                maxx=player.window.Width;
            }
            println!("{},{},{},{}",minx,maxx,miny,maxy);
            for x in minx as i64..maxx as i64{
                //println!("{}",x);
                let mut yrange=[miny as i64,maxy as i64];
                yrange.sort();
                for y in yrange[0]..yrange[1]{
                    //println!("{}",y);
                    let mut edge1;
                    let mut edge2;
                    let mut edge3;
                    if point_array[1].y>=point_array[0].y{
                        edge1=edgemath(&point_array[0],&point_array[2],&x,&y);
                        edge2=edgemath(&point_array[2],&point_array[1],&x,&y);
                        edge3=edgemath(&point_array[1],&point_array[0],&x,&y);
                    }
                    else{
                        edge1=edgemath(&point_array[0],&point_array[1],&x,&y);
                        edge2=edgemath(&point_array[1],&point_array[2],&x,&y);
                        edge3=edgemath(&point_array[2],&point_array[0],&x,&y);
                    }
                    if (edge1>=0) && (edge2>=0) && (edge3>=0){
                        //println!("{},{}",x,y);
                        frame_buffer[y as usize * player.window.Width as usize + x as usize]=0xFF0000;
                    }
                    //println!("{},{},{}",edge1,edge2,edge3);
                }
            }

        }
    }
}

pub fn render(world: &HashMap<String, Object>, player: &PlayerComp,frame_buffer:&mut Vec<u32>) {
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
    pixeliter(&projected,frame_buffer,&player);
}
