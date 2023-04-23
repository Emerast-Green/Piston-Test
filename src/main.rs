extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonEvent, ButtonState, Key, Button};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod stuff;
use stuff::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    squares: Vec<Square>,
    variables: Vec<f64>,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32;4] = [0.0,0.0,0.0,1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            for i in &self.squares {
                i.draw(c,gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        for i in &mut self.squares {
            i.update(&mut self.variables,args);
        }

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-squares", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        squares: Vec::new(),
        variables: Vec::new(),
    };
    // VARIABLES
    app.variables.push(30.0); // speed
    let pos = (100.0,100.0);
    app.squares.push(
        Square::new(pos,50.0,[1.0,1.0,1.0,1.0])
    );
    app.squares.push(
        Square::new(pos,25.0,[0.0,1.0,0.0,0.5])
    );
    app.squares.push(
        Square::new(pos,10.0,[1.0,0.0,0.0,0.5])
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::W) => {app.variables[0]=app.variables[0]+5.0;},
                    Button::Keyboard(Key::S) => {app.variables[0]=app.variables[0]-5.0;},
                    Button::Keyboard(Key::Escape) => {println!("Handled Keyboard(Escape) was pressed!");},
                    _ => {println!("Unhandled {:?} was pressed!",k.button)},
                }
            }
        }
    }
}