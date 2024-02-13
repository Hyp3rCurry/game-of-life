
use piston::window::WindowSettings;
//use piston::event_loop;
use piston::input::*;
use glutin_window::GlutinWindow;
use graphics::rectangle;
use graphics::types::{ColorComponent};//, Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::EventLoop;

const NSQ: usize = 200;
const SIZE: f64 = 3.0;
const WIN_SIZE: f64 = 600.0;
const WHITE: [ColorComponent; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [ColorComponent; 4] = [0.0, 0.0, 0.0, 1.0];

struct Game {
    gl: GlGraphics,
}
#[derive(Debug)]
struct Rect {
    x: f64,
    y: f64,
    color: [ColorComponent; 4],
}

impl Game {
    fn render (&mut self, arg: &RenderArgs, rects : &Vec<Vec<Rect>>) {

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);

            let transform = _c
                .transform;

            for row in rects.iter() {
                for item in row.iter() {
                    let square = rectangle::square(
                        item.x*SIZE,
                        item.y*SIZE,
                        SIZE);

                    rectangle(item.color, square, transform, gl);
                }
            }
        })
    }

}

fn generate_rects() -> Vec<Vec<Rect>>{
    let mut i = 0;
    let mut j = 0;
    let mut rects: Vec<Vec<Rect>> = vec![];

    while i < NSQ {
        let mut row: Vec<Rect> = Vec::new();
        while j < NSQ {
            let x: f64 = j as f64;
            let y: f64 = i as f64;

            let color= BLACK;
            row.push( Rect{x,y, color});
            j += 1;

        }

        rects.push(row);
        i += 1;
        j = 0;
    }
    assert!(rects.len() == 200 && rects[0].len() == 200);
    rects
}

fn update_rects(rects: &mut Vec<Vec<Rect>>) {
    let mut i = 1;
    let mut j = 1;

    while i < NSQ-1 {
        while j < NSQ-1 {
            let neighbours = count_neighbours(i, j, &rects);
            let rect = &mut rects[i][j];
            change_state(neighbours,rect);


            j += 1;
        }
        i += 1;
        j = 1;
    }
}

fn count_neighbours(x: usize, y: usize, rects: &Vec<Vec<Rect>>) -> i32{
    let mut count = 0;
    //edge cases: x=0,y=0, x= NSQ-1, y= NSQ-1
    let neighbours= vec![
            (x-1,y-1), (x,y-1), (x+1,y-1),
            (x-1,y),            (x+1,y),
            (x-1,y+1), (x,y+1), (x+1,y+1)];

    for pos in neighbours {
        let i = pos.0;
        let j = pos.1;
        if rects[i][j].color == WHITE {
            count += 1;
        }
    }

    count
}

fn change_state(neighbours:i32, rect: &mut Rect) {
    if rect.color == WHITE &&
        (neighbours < 2 || neighbours > 3){
        rect.color = BLACK;
    }
    if rect.color == BLACK && neighbours == 3 {
        rect.color = WHITE;
    }
}

fn paint(pos:[f64; 2], rects: &mut Vec<Vec<Rect>>) {
    let x = (pos[1]/SIZE) as usize;
    let y = (pos[0]/SIZE) as usize;

    rects[x][y].color = WHITE;
    rects[x+1][y].color = WHITE;
    rects[x][y+1].color = WHITE;
    rects[x+1][y+1].color = WHITE;
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

    let mut rects: Vec<Vec<Rect>> = generate_rects();


    let mut events = Events::new(EventSettings::new());
    let mut cursor: [f64;2] = [0.0,0.0];
    let mut last_cursor = cursor;
    events.set_max_fps(10);
    while let Some(e) = events.next(&mut window) {
        //mouse input

        cursor = e.mouse_cursor(|pos| { pos }).unwrap_or_else(|| last_cursor);
        last_cursor = cursor;

        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(_) => paint(cursor, &mut rects),
                _ => {}
            }
        }


        if let Some(args) = e.render_args() {

            update_rects(&mut rects);

            game.render(&args, &rects);
        }
    }
}