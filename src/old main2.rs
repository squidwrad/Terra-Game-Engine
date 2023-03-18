use std::{collections::HashMap, hash::Hash, vec};
use minifb::{Key,Window,WindowOptions};
struct object 
{
    name: String,
    Triangles: Vec<triangle>,
}
struct triangle
{
    p1: point,
    p2: point,
    p3: point,
}
struct point
{
    x:f64,
    y:f64,
    z:f64,
}
struct playercomp
{
    playerx:f64,
    playery:f64,
    playerz:f64,
    playera:f64,
    playerfov:f64,
}
fn transform(point:&point,player:&playercomp)->point
{
    let mut x=point.x;
    let mut y=point.y;
    let mut z=point.z;
    let mut newx=x-player.playerx;
    let mut newy=y-player.playery;
    let mut pi=std::f64::consts::PI;
    let mut cos=player.playera/(180.0*pi);
    let mut sin=player.playera/(180.0*pi);
    let mut tx=(newx*cos.cos())-(newy*sin.sin());
    let mut ty=(newx*cos.cos())+(newy*sin.sin());
    let mut tz=z-player.playerz;
    let mut Tpoint=point{x:tx,y:ty,z:tz};
    return Tpoint;


}
fn project(point:&point,player:&playercomp)->(f64,f64)
{   
    let mut x=point.x;
    let mut y=point.y;
    if y.clone()==0.0
    {
        y=1.0;
    }
    let mut px=(x*player.playerfov)/y+(1920.0/2.0);
    let mut py=(y*player.playerfov)/(-y)+(1080.0/2.0);
    return (px,py);
}
fn worldarray()-> HashMap<String,object>
{
    let mut Worldc=HashMap::new();
    let mut point1=point{x:5.0,y:10.0,z:0.0};
    let mut point2=point{x:10.0,y:10.0,z:0.0};
    let mut point3=point{x:5.0,y:10.0,z:20.0};
    let mut triangle=triangle{ p1:point1,p2:point2,p3:point3};
    let mut obj1=object{name: String::from("wall1"),Triangles:vec!(triangle)};
    Worldc.insert(obj1.name.clone(),obj1);
    return Worldc;
}

fn MainUpdate()
{
    
}
fn render(Worldc: &HashMap<String,object>,player:&playercomp)
{
   let mut translated: HashMap<String,object>=HashMap::new(); 
   let mut projected:HashMap<String,object>=HashMap::new();
   for (objname,obj) in Worldc.iter()
   {
    let mut trans_obj=object{name:objname.to_string(),Triangles:vec!()};
    let mut proj_obj=object{name:objname.to_string(),Triangles:vec!()};
    translated.insert(objname.to_string(),trans_obj); 
    projected.insert(objname.to_string(),proj_obj);
    for (tri) in &obj.Triangles
    {  
        let mut p1=&tri.p1;
        let mut tp1=transform(&p1,&player);
        let mut pp1=project(&tp1,&player);
        let mut p2=&tri.p2;
        let mut tp2=transform(&p1,&player);
        let mut pp2=project(&tp1,&player);
        let mut p3=&tri.p3;
        let mut tp3=transform(&p1,&player);
        let mut pp3=project(&tp1,&player);
        let mut Ttriangle=triangle{p1:tp1,p2:tp2,p3:tp3};
        //let mut Ptriangle=triangle{};
        translated.get_mut(objname).unwrap().Triangles.push(Ttriangle);

    }
   }
}
fn main() //this is were the program starts
{   
    let mut Worldc=worldarray();//this calls a function to create and store the main world object.
    let mut player=playercomp{playerx:0.0,playery:0.0,playerz:20.0,playera:0.0,playerfov:200.0};
    let Height= 1080;
    let Width = 1920;
    let mut Window=Window::new( "test-EscToExit",Width,Height,WindowOptions::default(),).unwrap();
    let mut fbuffer=vec![0;Height*Width];

    while Window.is_open()
    {   
        MainUpdate();
        render(&Worldc,&player);
        Window.update_with_buffer(&fbuffer,Width,Height);
    }
}
