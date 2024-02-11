
use piston::window::WindowSettings;
use piston::event_loop;
use piston::input::*;
use glutin_window::GlutinWindow;
use graphics::rectangle;
use graphics::types::{ColorComponent, Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};

const NSQ: i32 = 200;
const SIZE: f64 = 3.0;
const WIN_SIZE: f64 = 600.0;
const WHITE: [ColorComponent; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [ColorComponent; 4] = [0.0, 0.0, 0.0, 1.0];

struct Game {
    gl: GlGraphics,
}

#[derive(Default)]
struct Rect {
    x: f64,
    y: f64,
    color: [ColorComponent; 4],
    square: Rectangle,
}

impl Game {
    fn render (&mut self, arg: &RenderArgs, rects : &Vec<Vec<Rect>>) {
        use graphics;

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);

            let transform = _c
                .transform;
            for row in rects.iter() {
                for item in row.iter() {
                    rectangle(item.color, item.square, transform, gl);
                }
            }
        })
    }

}

fn generate_rects() -> Vec<Vec<Rect>>{
    let mut i: i32 = 0;
    let mut j:i32 = 0;
    let mut rects: Vec<Vec<Rect>> = vec![vec![]];

    while i < NSQ {
        let mut row: Vec<Rect> = Vec::new();
        while j < NSQ {
            let x: f64 = f64::from(j) * SIZE;
            let y: f64 = f64::from(i) * SIZE;

            let color;
            if j % 2 == 0 && i %2 == 0 {
                color = WHITE;
            }
            else {
                color = BLACK;
            }
            let square = rectangle::square(x, y, SIZE);

            row.push(Rect{x,y, color, square});
            j += 1;
        }
        rects.push(row);
        i += 1;
        j = 0;
    }
    rects
}

fn main() {
    let opengl = OpenGL::V3_3;

    let mut window: GlutinWindow = WindowSettings::new(
        "GameOfLife",
        [WIN_SIZE, WIN_SIZE]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
    };

    let rects: Vec<Vec<Rect>> = generate_rects();


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args, &rects);
        }
    }
}