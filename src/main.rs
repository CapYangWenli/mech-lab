// Crudish plan for the program
// 1. Make a approximate grid where objects could be drawn
// 2. Draw simple shapes mimicking basic objects of the sim (sticks, joints, ground)
// 3. Make a entity tracking system that stores position, type, and properties of the object so they will be easier to draw
// 4. Draw a crude graph of basic functions including linear, polynomial, trigonometric and exponential
// 5. do simple calculation of physical properties of the object and display them in a graph
// 6. Analyze results and refine unil the goal is met

use macroquad::prelude::*;

const SPACING: f32 = 40.0;

struct Rectangle {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Color,
}

impl Rectangle {
    fn draw_box(&self) {
        draw_rectangle(
            self.x1,
            self.y1,
            self.x2 - self.x1,
            self.y2 - self.y1,
            self.color,
        );
    }
}
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLUE);

        let obj = Rectangle {
            x1: 40.0,
            y1: 40.0,
            x2: 40.0 + SPACING,
            y2: 40.0 + SPACING,
            color: WHITE,
        };

        draw_2d_grid(SPACING, 1.0, LIGHTGRAY);

        obj.draw_box();
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, WHITE);

        next_frame().await
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
