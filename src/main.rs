// Crudish plan for the program
// 1. Make a approximate grid where objects could be drawn
// 2. Draw simple shapes mimicking basic objects of the sim (sticks, joints, ground)
// 3. Make a entity tracking system that stores position, type, and properties of the object so they will be easier to draw
// 4. Draw a crude graph of basic functions including linear, polynomial, trigonometric and exponential
// 5. do simple calculation of physical properties of the object and display them in a graph
// 6. Analyze results and refine unil the goal is met

use macroquad::{experimental::scene::Node, prelude::*};

const SPACING: f32 = 40.0;
const MICROCELL_SIZE: f32 = 10.0;
const CELL_SIZE: f32 = MICROCELL_SIZE * 4.0;
const LINE_THICKNESS: f32 = 2.0;

trait Drawable {
    fn draw(&self);
}
enum Orientation {
    Left,
    Right,
    Up,
    Down,
}
struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    thck: f32,
    color: Color,
    color_bkcg: Color,
}

struct Ground {
    x: i32,
    y: i32,
    len: i32,
    ornt: Orientation,
    color: Color,
}

struct Stick {
    x: i32,
    y: i32,
    len: i32,
    width: f32,
    ornt: Orientation,
    color: Color,
}

struct Arrow {
    x: i32,
    y: i32,
    len: i32,
    arrow_size: f32,
    ornt: Orientation,
    color: Color,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        draw_rectangle(
            self.x as f32 * SPACING,
            self.y as f32 * SPACING,
            self.w as f32 * SPACING,
            self.h as f32 * SPACING,
            self.color_bkcg,
        );
        draw_rectangle_lines(
            self.x as f32 * SPACING,
            self.y as f32 * SPACING,
            self.w as f32 * SPACING,
            self.h as f32 * SPACING,
            self.thck,
            self.color,
        );
    }
}
impl Drawable for Stick {
    fn draw(&self) {
        match self.ornt {
            Orientation::Up | Orientation::Down => draw_rectangle(
                (self.x as f32 - self.width * 0.5) * SPACING,
                self.y as f32 * SPACING,
                self.width * SPACING,
                self.len as f32 * SPACING,
                self.color,
            ),
            Orientation::Right | Orientation::Left => draw_rectangle(
                self.x as f32 * SPACING,
                (self.y as f32 - self.width * 0.5) * SPACING,
                self.len as f32 * SPACING,
                self.width * SPACING,
                self.color,
            ),
        }
    }
}

impl Drawable for Arrow {
    fn draw(&self) {
        match self.ornt {
            Orientation::Up => {
                draw_line(
                    self.x as f32 * SPACING,
                    self.y as f32 * SPACING,
                    self.x as f32 * SPACING,
                    (self.y + self.len) as f32 * SPACING,
                    LINE_THICKNESS,
                    self.color,
                );
            }
            _ => todo!(),
        }
    }
}

impl Drawable for Ground {
    fn draw(&self) {
        match self.ornt {
            Orientation::Up => {
                draw_line(
                    self.x as f32 * SPACING,
                    self.y as f32 * SPACING,
                    (self.x + self.len) as f32 * SPACING,
                    self.y as f32 * SPACING,
                    LINE_THICKNESS,
                    self.color,
                );

                for i in 1..=(self.len * 2) {
                    draw_line(
                        (self.x as f32 + i as f32 * 0.5) * SPACING,
                        self.y as f32 * SPACING,
                        ((self.x as f32 + i as f32 * 0.5) - 0.5) * SPACING,
                        (self.y as f32 + 0.5) * SPACING,
                        LINE_THICKNESS * 0.5,
                        self.color,
                    );
                }
            }

            Orientation::Down => {
                draw_line(
                    self.x as f32 * SPACING,
                    self.y as f32 * SPACING,
                    (self.x + self.len) as f32 * SPACING,
                    self.y as f32 * SPACING,
                    LINE_THICKNESS,
                    self.color,
                );

                for i in 0..(self.len * 2) {
                    draw_line(
                        (self.x as f32 + i as f32 * 0.5) * SPACING,
                        self.y as f32 * SPACING,
                        ((self.x as f32 + i as f32 * 0.5) + 0.5) * SPACING,
                        (self.y as f32 - 0.5) * SPACING,
                        LINE_THICKNESS * 0.5,
                        self.color,
                    );
                }
            }

            Orientation::Right => {
                draw_line(
                    self.x as f32 * SPACING,
                    self.y as f32 * SPACING,
                    self.x as f32 * SPACING,
                    (self.y + self.len) as f32 * SPACING,
                    LINE_THICKNESS,
                    self.color,
                );

                for i in 0..(self.len * 2) {
                    draw_line(
                        self.x as f32 * SPACING,
                        (self.y as f32 + i as f32 * 0.5) * SPACING,
                        (self.x as f32 - 0.5) * SPACING,
                        ((self.y as f32 + i as f32 * 0.5) + 0.5) * SPACING,
                        LINE_THICKNESS * 0.5,
                        self.color,
                    );
                }
            }

            Orientation::Left => {
                draw_line(
                    self.x as f32 * SPACING,
                    self.y as f32 * SPACING,
                    self.x as f32 * SPACING,
                    (self.y + self.len) as f32 * SPACING,
                    LINE_THICKNESS,
                    self.color,
                );

                for i in 1..=(self.len * 2) {
                    draw_line(
                        self.x as f32 * SPACING,
                        (self.y as f32 + i as f32 * 0.5) * SPACING,
                        (self.x as f32 + 0.5) * SPACING,
                        ((self.y as f32 + i as f32 * 0.5) - 0.5) * SPACING,
                        LINE_THICKNESS * 0.5,
                        self.color,
                    );
                }
            }
            _ => todo!(),
        }
    }
}

fn draw_2d_grid(spacing: f32, thickness: f32, color: Color) {
    for i in 0..=(screen_width() / spacing) as i32 {
        draw_line(
            spacing * i as f32,
            0.0,
            spacing * i as f32,
            screen_height(),
            thickness,
            color,
        )
    }

    for j in 0..=(screen_height() / spacing) as i32 {
        draw_line(
            0.0,
            spacing * j as f32,
            screen_width(),
            spacing * j as f32,
            thickness,
            color,
        );
    }
}

fn draw_all_objs(vec: Vec<Box<dyn Drawable>>) {
    for obj in vec {
        obj.draw();
    }
}
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let blueprint_clr: Color = Color::from_hex(0x228be6);
        clear_background(blueprint_clr);
        let mut drawable_objects: Vec<Box<dyn Drawable>> = vec![];

        let rec_obj = Rectangle {
            x: 3,
            y: 3,
            w: 6,
            h: 3,
            thck: 6.0,
            color: WHITE,
            color_bkcg: blueprint_clr,
        };
        drawable_objects.push(Box::new(rec_obj));

        let gnd_obj = Ground {
            x: 12,
            y: 3,
            len: 1,
            ornt: Orientation::Up,
            color: WHITE,
        };
        drawable_objects.push(Box::new(gnd_obj));

        drawable_objects.push(Box::new(Ground {
            x: 17,
            y: 3,
            len: 2,
            ornt: Orientation::Down,
            color: WHITE,
        }));
        drawable_objects.push(Box::new(Ground {
            x: 2,
            y: 9,
            len: 5,
            ornt: Orientation::Right,
            color: WHITE,
        }));
        drawable_objects.push(Box::new(Ground {
            x: 15,
            y: 9,
            len: 5,
            ornt: Orientation::Left,
            color: WHITE,
        }));

        drawable_objects.push(Box::new(Stick {
            x: 18,
            y: 3,
            len: 9,
            width: 0.5,
            ornt: Orientation::Up,
            color: WHITE,
        }));
        drawable_objects.push(Box::new(Stick {
            x: 2,
            y: 10,
            len: 4,
            width: 0.5,
            ornt: Orientation::Right,
            color: WHITE,
        }));

        draw_2d_grid(SPACING, 1., LIGHTGRAY);

        draw_all_objs(drawable_objects);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
