use graphics::{Context, Transformed, rectangle};
use graphics::rectangle::square;
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

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