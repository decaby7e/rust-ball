use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use term_size;

const WIDTH: usize = 128;
const HEIGHT: usize = 32;
//const GRAVITY: f32 = 0.03;
const GRAVITY: f32 = 0.00;

type Display = [[u8; WIDTH]; HEIGHT];

#[derive(Debug)]
struct V2f {
    x: f32,
    y: f32,
}

struct Circle {
    radius: i32,
    center: V2f,
    velocity: V2f,
}

impl Circle {
    fn draw(&self, display: &mut Display) {
        for x in 0..display.len() {
            for y in 0..display[x].len() {
                let dx = x as f32 + 0.5;
                let dy = y as f32 + 0.5;
                let d = ((dx as f32 - self.center.x - 0.5).powf(2.0)
                    + (dy as f32 - self.center.y - 0.5).powf(2.0)) as f32;
                let r_2 = self.radius.pow(2);
                if d <= r_2 as f32 {
                    display[x][y] = '@' as u8;
                } else {
                    display[x][y] = ' ' as u8;
                }
            }
        }
    }

    fn tick_physics(&mut self) {
        if self.center.x + self.radius as f32 >= HEIGHT as f32 {
            self.velocity = V2f {
                x: self.velocity.x * -0.98,
                y: self.velocity.y,
            };
            self.center.x = HEIGHT as f32 - self.radius as f32;
            // Frictional force
            // if self.velocity.y > 0.0 {
            //     self.velocity.y *= 0.98;
            // } else {
            //     self.velocity.y = 0.0;
            // }
        } else {
            self.velocity.x += GRAVITY;
        }

        if self.center.y + self.radius as f32 >= WIDTH as f32 {
            self.velocity = V2f {
                x: self.velocity.x,
                y: self.velocity.y * -1.0,
            };
            self.center.y = WIDTH as f32 - self.radius as f32;
        }

        if self.center.y - self.radius as f32 <= 0 as f32 {
            self.velocity = V2f {
                x: self.velocity.x,
                y: self.velocity.y * -1.0,
            };
            self.center.y = self.radius as f32;
        }

        self.center.x += self.velocity.x;
        self.center.y += self.velocity.y;
    }
}

fn update_display(arr: &Display) {
    print!("{}c", 27 as char); // clear the screen
    for row in arr {
        io::stdout().write_all(row);
        io::stdout().write(&['\n' as u8]);
    }
}

fn main() {
    let mut display: Display = [[' ' as u8; WIDTH]; HEIGHT];

    let wait_time: Duration = Duration::from_millis(14); // ~ 30tps

    let mut circle = Circle {
        radius: 5,
        center: V2f { x: 18.0, y: 15.0 },
        velocity: V2f { x: 0.0, y: 0.0 },
    };

    update_display(&display);
    loop {
        circle.tick_physics();
        circle.draw(&mut display);
        update_display(&display);
        sleep(wait_time);
    }
}
