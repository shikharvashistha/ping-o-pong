extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::GlutinWindow;//allows to create opengl window
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics,//the opengl graphics
    left_score: i32,
    left_pos: i32,
    left_vel: i32,//velocity
    right_score: i32,
    right_pos: i32,
    right_vel: i32,
    ball_x: i32,
    ball_y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 50.0);//x, y, size
        let left_pos = self.left_pos as f64;
        let right = rectangle::square(0.0, 0.0, 50.0);
        let right_pos = self.right_pos as f64;

        let ball = rectangle::square(0.0, 0.0, 10.0);
        let ball_x = self.ball_x as f64;
        let ball_y = self.ball_y as f64;

        self.gl.draw(args.viewport(), |c, gl| {
            // c is context and gl being the opengl graphics render
            clear(BACKGROUND, gl);
            rectangle(FOREGROUND, left, c.transform.trans(-40.0, left_pos), gl);
            rectangle(
                FOREGROUND,
                right,
                c.transform.trans(args.width as f64 - 10.0, right_pos),
                gl,
            );
            rectangle(FOREGROUND, ball, c.transform.trans(ball_x, ball_y), gl);
        });//defining paddles as squares and then transforming them to rectangles
    }

    fn update(&mut self, _args: &UpdateArgs) {//game logic
        if (self.left_vel == 1 && self.left_pos < 291)
            || (self.left_vel == -1 && self.left_pos >= 1)
        {
            self.left_pos += self.left_vel;
        }//to check paddles are not out of bounds
        if (self.right_vel == 1 && self.right_pos < 291)
            || (self.right_vel == -1 && self.right_pos >= 1)
        {
            self.right_pos += self.right_vel;
        }//to check paddles are not out of bounds

        self.ball_x += self.vel_x;
        if self.ball_x > 502 {//right side of screen
            self.vel_x = -self.vel_x;//reverse x velocity
            if self.ball_y < self.right_pos || self.ball_y > self.right_pos + 50 {
                //if ball is not in the paddle
                self.left_score += 1;
                if self.left_score >= 5 {//left score >=5 left wins
                    println!("Left wins!");
                    process::exit(0);
                }
                //pass the right paddle then 
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }
        //This is for left paddle
        if self.ball_x < 1 {
            self.vel_x = -self.vel_x;
            if self.ball_y < self.left_pos || self.ball_y > self.left_pos + 50 {
                self.right_score += 1;
                if self.right_score >= 5 {
                    println!("Right wins!");
                    process::exit(0);
                }
                self.ball_x = 256;
                self.ball_y = 171;
            }
        }
        //This allows our ball to bounce off the top and bottom of the screen
        self.ball_y += self.vel_y;
        if self.ball_y > 332 || self.ball_y < 1 {
            //if the ball either hit top or bottom of the screen
            self.vel_y = -self.vel_y;//we reverse the velocity
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            //if let binding to destruct our args
            match key {
                Key::Up => {
                    self.right_vel = -1;
                }
                Key::Down => {
                    self.right_vel = 1;
                }
                Key::W => {
                    self.left_vel = -1;
                }
                Key::S => {
                    self.left_vel = 1;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        //if let binding on out args to
        //see if it eqals out button keyboard key
        if let &Button::Keyboard(key) = args {
            match key {//release the button to immeadiately stop movement
                Key::Up => {
                    self.right_vel = 0;
                }
                Key::Down => {
                    self.right_vel = 0;
                }
                Key::W => {
                    self.left_vel = 0;
                }
                Key::S => {
                    self.left_vel = 0;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pong", [512, 342])//new window
        /* .opengl(opengl) */
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {//instantiate our app structure
        gl: GlGraphics::new(opengl),//bound to opengl buffer
        left_score: 0,
        left_pos: 1,
        left_vel: 0,
        right_score: 0,
        right_pos: 1,
        right_vel: 0,
        ball_x: 0,
        ball_y: 0,
        vel_x: 1,
        vel_y: 1,
    };

    let mut events = Events::new(EventSettings::new());
    //bounds to events new 
    while let Some(e) = events.next(&mut window) {
        //while let binding to see if events.next is equal to pattern of some e
        //basically this is our game loop
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}