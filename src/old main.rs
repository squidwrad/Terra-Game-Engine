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
fn transform(x:&f64,y:&f64,z:&f64,player:&playercomp)->(f64,f64,f64)
{
    let mut newx=x-player.playerx;
    let mut newy=y-player.playery;
    let mut pi=std::f64::consts::PI;
    let mut cos=player.playera/(180.0*pi);
    let mut sin=player.playera/(180.0*pi);
    let mut tx=(newx*cos.cos())-(newy*sin.sin());
    let mut ty=(newx*cos.cos())+(newy*sin.sin());
    let mut tz=z-player.playerz;
    return (tx,ty,tz);


}
fn project(x:&f64,mut y:&f64,z:&f64,player:&playercomp)->(f64,f64)
{   
    if y.clone()==0.0
    {
        y=&1.0;
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
    let mut new_obj=object{name:objname.to_string(),Triangles:vec!()};
    translated.insert(objname.to_string(),new_obj); 
    projected.insert(objname.to_string(),new_obj);
    for (tri) in &obj.Triangles
    {  
       let mut p1=&tri.p1;
       let mut tp1=transform(&p1.x,&p1.y,&p1.z,&player);
       let mut sp1=point{x:tp1.0,y:tp1.1,z:tp1.2};
       let mut pp1=project(tp1.0,tp1.1,tp1.2);
       let mut p2=&tri.p2;
       let mut tp2=transform(&p2.x,&p2.y,&p2.z,&player);
       let mut sp2=point{x:tp2.0,y:tp2.1,z:tp2.2};
       let mut pp2=project(tp2.0,tp2.1,tp2.2);
       let mut p3=&tri.p3;
       let mut tp3=transform(&p3.x,&p3.y,&p3.z,&player);
       let mut sp3=point{x:tp3.0,y:tp3.1,z:tp3.2};
       let mut pp3=project(tp3.0,tp3.1,tp3.2);
       let mut Ttriangle=triangle{p1:sp1,p2:sp2,p3:sp3};
       let mut Ptriangle=triangle{};
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
