extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;

use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::{ButtonEvent, ButtonState, Key, Button, EventLoop};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use glem::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game,
    glyphs: GlyphCache<'static>,
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32;4] = [0.0,0.0,0.0,1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            for i in &self.game.colliders {
                i.draw(c,gl);
            }
            for i in &self.game.physics {
                i.draw(c,gl);
            }
            let transform = c
            .transform
            .trans(10.0,10.0);
            Text{color:[1.0,1.0,0.0,1.0],font_size:16,round:false}.draw(
                self.game.colliders[0].collides(&self.game.physics[0]).to_string().as_str(), 
                &mut self.glyphs, 
                &c.draw_state, 
                transform, 
                gl).expect("Can't even write some letters...")
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        for i in &mut self.game.physics {
            i.update(&self.game.colliders);
        }

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let window_size = [640.0,480.0];
    let mut window: Window = WindowSettings::new("moving-squares", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    const F: () = ();
    let mut app = App {
        gl: GlGraphics::new(opengl),
        game: Game::new(),
        glyphs: GlyphCache::from_font(get_font(),F, TextureSettings::new()),
    };
    // VARIABLES
    app.game.variables.push(30.0); // speed
    app.game.physics.push(Physics::new((0.0,0.0),(30.0,30.0),20.0));
    //app.physics[0].controller.motion[3]=true;
    app.game.colliders.push(Collider::new((0.0,window_size[1]-20.0),(window_size[0],20.0))); // floor 
    app.game.colliders.push(Collider::new((60.0,120.0),(50.0,1.0))); // platform
    app.game.colliders.push(Collider::new((-10.0,0.0),(10.0,window_size[1]))); // left border
    app.game.colliders.push(Collider::new((window_size[0],0.0),(10.0,window_size[1]))); // right border
    let mut events = Events::new(EventSettings::new()).max_fps(30);
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
                    //Button::Keyboard(Key::W) => {app.physics[0].controller.motion[0]=true;},
                    //Button::Keyboard(Key::S) => {app.physics[0].controller.motion[1]=true;},
                    Button::Keyboard(Key::A) => {app.game.physics[0].controller.motion[2]=true;},
                    Button::Keyboard(Key::D) => {app.game.physics[0].controller.motion[3]=true;},
                    Button::Keyboard(Key::Space) => {app.game.physics[0].controller.jump=true;},
                    Button::Keyboard(Key::Escape) => {println!("Handled Keyboard(Escape) was pressed!");},
                    _ => {println!("Unhandled {:?} was pressed!",k.button)},
                }
            }
            if k.state == ButtonState::Release {
                match k.button {
                    //Button::Keyboard(Key::W) => {app.physics[0].controller.motion[0]=false;},
                    //Button::Keyboard(Key::S) => {app.physics[0].controller.motion[1]=false;},
                    Button::Keyboard(Key::A) => {app.game.physics[0].controller.motion[2]=false;},
                    Button::Keyboard(Key::D) => {app.game.physics[0].controller.motion[3]=false;},
                    Button::Keyboard(Key::Escape) => {},
                    _ => {},
                }
            }
        }
    }
}