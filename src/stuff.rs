use std::f64;

use graphics::{Context, Transformed, rectangle };

use opengl_graphics::{GlGraphics };

use rusttype::{Font};

use std::collections::{HashMap};

pub struct Controller {
    pub motion: [bool;4],
    pub weight: f64,
}

impl Controller {
    pub fn new() -> Controller {
        Controller{motion:[false,false,false,false],weight:1.0}
    }
    pub fn get_speed_0(&mut self) -> f64 {
        let mut a: f64 = 0.0;
        if self.motion[2] {a-=self.weight;}; // left
        if self.motion[3] {a+=self.weight;}; // right
        a
    }
    pub fn get_speed_1(&mut self) -> f64 {
        let mut a: f64 = 0.0;
        if self.motion[0] {a-=self.weight;}; // up
        if self.motion[1] {a+=self.weight;}; // down
        a
    }

}

/*
use graphics::rectangle::square;
use piston::{UpdateArgs };

pub struct Square {
    pub pos: (f64,f64),
    pub size: f64,
    pub rotation: f64,
    pub color: [f32; 4],
}

impl Square {
    pub fn new(pos:(f64,f64),size:f64,color:[f32;4]) -> Square {
        Square{pos:pos,size:size,rotation:0.0,color:color}
    }
    pub fn draw(&self,c: Context,gl:&mut GlGraphics) {
        let transform = c
            .transform
            .trans(self.pos.0,self.pos.1)
            .rot_rad(self.rotation)
            .trans(-self.size/2.0,-self.size/2.0);
        rectangle(self.color,square(0.0,0.0,self.size),transform,gl);
    }
    pub fn update(&mut self,variables:&mut Vec<f64>,args:&UpdateArgs) {
        self.rotation+=(variables[0]/self.size) * args.dt;
    }
}
*/

pub trait SimpleDraw {
    fn pos(&self) -> (f64,f64);
    fn size(&self) -> (f64,f64);
    fn color(&self) -> [f32;4];
    fn draw(&self,c: Context,gl:&mut GlGraphics) {
        let transform = c
            .transform
            .trans(self.pos().0,self.pos().1);
        rectangle(self.color(),rectangle::rectangle_by_corners(0.0,0.0,self.size().0,self.size().1),transform,gl);
    }
}

pub struct Collider {
    pub pos: (f64,f64),
    pub size: (f64,f64),
    pub platform: bool,
}

impl Collider {
    pub fn new(pos:(f64,f64),size:(f64,f64)) -> Collider {
        Collider{pos:pos,size:size,platform:false}
    }
    pub fn set_platform(mut self,v:bool) -> Self {
        self.platform=v;
        self
    }
    // TODO: FIX ME!
    pub fn collides(&self,physics:&Physics) -> bool {
        ((self.pos.0>=physics.pos.0 && self.pos.0<=physics.pos.0+physics.size.0) || 
         (physics.pos.0>=self.pos.0 && physics.pos.0<=self.pos.0+self.size.0)) 
        && 
        ((self.pos.1>=physics.pos.1 && self.pos.1<=physics.pos.1+physics.size.1) || 
         (physics.pos.1>=self.pos.1 && physics.pos.1<=self.pos.1+self.size.1)) 
    }
}

impl SimpleDraw for Collider {
    fn pos(&self) -> (f64,f64) { self.pos }
    fn size(&self) -> (f64,f64) { self.size }
    fn color(&self) -> [f32;4] { [1.0,0.0,0.0,1.0] }
}

pub struct Physics {
    pub pos: (f64,f64),
    pub size: (f64,f64),
    pub weight: f64,
    pub speed: (f64,f64),
    pub controller: Controller,
    pub has_jumped: bool,
    pub collisions: (bool,bool,bool,bool),
}

pub fn clamp(v:f64,u:f64,d:f64) -> f64 {
    if v>u {u} else if v<d {d} else {v}
}
pub fn degrade(v:f64,b:f64,d:f64) -> f64 {
    if v.abs()>b {0.0} else {v/d}
}

pub fn get_font() -> Font<'static> {
    let font_data: &[u8] = include_bytes!("Nexa-Heavy.ttf");
    let owned_font_data: Vec<u8> = font_data.to_vec();
    let from_owned_font: Font<'static> = Font::try_from_vec(owned_font_data).expect("");
    from_owned_font
}

#[allow(dead_code)]
pub fn distance(a:(f64,f64),b:(f64,f64)) -> f64 {
    ((a.0-b.0).powi(2)+(a.1-b.1).powi(2)).sqrt()
}

pub fn set_at_index(d:&mut (bool,bool,bool,bool),k:i8,v:bool){
    //! Set value (d.k) to v    
    match k {
        0 => { d.0=v; },
        1 => { d.1=v; },
        2 => { d.2=v; },
        3 => { d.3=v; },
        _ => {},
    }
}

impl Physics {
    pub fn new(pos:(f64,f64),size:(f64,f64),weight:f64) -> Physics {
        Physics{pos:pos,size:size,weight:weight,speed:(0.0,0.0),controller:Controller::new(),has_jumped:false,collisions:(false,false,false,false)}
    }
    pub fn update(&mut self,colliders:&Vec<Collider>) {
        if self.collisions.0 {}else{self.speed.1+=0.5};
        self.speed.0 += self.controller.get_speed_0();
        self.speed.1 += self.controller.get_speed_1();
        self.pos.0 += self.speed.0;
        self.pos.1 += self.speed.1;
        self.speed.0 = degrade(self.speed.0, 0.125,1.1);
        self.speed.1 = degrade(self.speed.1, 0.125,1.1);
        self.speed.0 = clamp(self.speed.0, 5.0, -5.0);
        self.speed.1 = clamp(self.speed.1, 5.0, -5.0);
        self.collisions.0=false;self.collisions.1=false;self.collisions.2=false;self.collisions.3=false;
        for i in colliders {
            if i.collides(&self) {
                let a = self.collision(i);
                set_at_index(&mut self.collisions,a,true);
            };
        }
    }
    pub fn collision(&mut self,collider:&Collider) -> i8{
        /*
        let mut d: HashMap<i8,f64> = HashMap::new();
        d.insert(0, (self.pos.1+self.size.1-collider.pos.1).abs()); // above
        d.insert(1, (self.pos.1-collider.size.1-collider.pos.1).abs()); // below
        d.insert(2, (self.pos.0-collider.size.0-collider.pos.0).abs()); // left
        d.insert(3, (self.pos.0+self.size.0-collider.pos.0).abs()); // right
        */
        let mut d: (f64,f64,f64,f64) = (0.0,0.0,0.0,0.0);
        d.0=(self.pos.1+self.size.1-collider.pos.1).abs();
        d.1=(self.pos.1-collider.size.1-collider.pos.1).abs();
        d.2=(self.pos.0-collider.size.0-collider.pos.0).abs();
        d.3=(self.pos.0+self.size.0-collider.pos.0).abs();
        let mut mk = 32;
        let mut mv = 1000000.0;
        if d.0<mv {mk=0;mv=d.0}else{};
        if d.1<mv {mk=1;mv=d.1}else{};
        if d.2<mv {mk=2;mv=d.2}else{};
        if d.3<mv {mk=3;mv=d.3}else{};
        match mk {
            0 => { // coming from above
                self.pos.1=collider.pos.1-self.size.1;
                if self.speed.1>0.0 { self.speed.1=0.0 }
            },
            1 => { // coming from below
                if collider.platform {}else{
                self.pos.1=collider.pos.1+collider.size.1;
                if self.speed.1<0.0 { self.speed.1=0.0 }
                }
            },
            2 => { // coming from left
                self.pos.0=collider.pos.0+collider.size.0;
                if self.speed.0<0.0 { self.speed.0=0.0 }
            },
            3 => { // coming from right
                self.pos.0=collider.pos.0-self.size.0;
                if self.speed.0>0.0 { self.speed.0=0.0 }
            },
            _ => {println!("At Physics.collision, somehow impossible collision type key was matched...")},
        }
        mk
    }
}

impl SimpleDraw for Physics {
    fn pos(&self) -> (f64,f64) { self.pos }
    fn size(&self) -> (f64,f64) { self.size }
    fn color(&self) -> [f32;4] { [0.0,1.0,0.0,1.0] }
}
