use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: u32,
}

impl Ball {
    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // Bounce off the walls
        if self.x < 0.0 || self.x >= WIDTH as f32 {
            self.dx = -self.dx;
        }
        if self.y < 0.0 || self.y >= HEIGHT as f32 {
            self.dy = -self.dy;
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Rust GFX Demo - Moving Shapes",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rng = rand::thread_rng();

    // Create some balls
    let mut balls: Vec<Ball> = (0..10)
        .map(|_| Ball {
            x: rng.gen_range(0.0..WIDTH as f32),
            y: rng.gen_range(0.0..HEIGHT as f32),
            dx: rng.gen_range(-2.0..2.0),
            dy: rng.gen_range(-2.0..2.0),
            color: rng.gen::<u32>() | 0xFF000000, // Ensure alpha channel is set
        })
        .collect();

    // Rotation angle for the logo
    let mut angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0; // Black background
        }

        // Update and draw the balls
        for ball in balls.iter_mut() {
            ball.update();
            let x = ball.x as usize;
            let y = ball.y as usize;

            // Draw the ball as a pixel for simplicity
            if x < WIDTH && y < HEIGHT {
                buffer[y * WIDTH + x] = ball.color;
            }
        }

        // Draw connecting lines between balls for a dynamic effect
        for i in 0..balls.len() {
            for j in i + 1..balls.len() {
                draw_line(
                    &mut buffer,
                    (balls[i].x as usize, balls[i].y as usize),
                    (balls[j].x as usize, balls[j].y as usize),
                    rng.gen::<u32>() | 0xFF000000,
                );
            }
        }

        // Draw the rotating logo at the bottom right corner
        draw_logo(&mut buffer, WIDTH, HEIGHT, angle);
        angle += 0.05; // Increment the rotation angle

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// Function to draw a simple rotating "logo" as a circle pattern
fn draw_logo(buffer: &mut [u32], width: usize, height: usize, angle: f32) {
    let logo_color1 = 0xFF358997; // Color #358997
    let logo_color2 = 0xFF8ED3D4; // Color #8ED3D4
    let radius = 20.0;
    
    // Center position for the logo in the bottom right corner
    let center_x = width as f32 - 50.0;
    let center_y = height as f32 - 50.0;

    // Draw a rotating pattern of small circles
    for i in 0..8 {
        let theta = angle + i as f32 * std::f32::consts::PI / 4.0;
        let x = center_x + theta.cos() * radius;
        let y = center_y + theta.sin() * radius;
        
        draw_filled_circle(buffer, x as usize, y as usize, 5, if i % 2 == 0 { logo_color1 } else { logo_color2 });
    }
}

// Draw a filled circle at (cx, cy) with the given radius and color
fn draw_filled_circle(buffer: &mut [u32], cx: usize, cy: usize, radius: usize, color: u32) {
    let r2 = radius * radius;
    for y in 0..=radius {
        for x in 0..=radius {
            if x * x + y * y <= r2 {
                let px = cx as isize + x as isize;
                let py = cy as isize + y as isize;
                let nx = cx as isize - x as isize;
                let ny = cy as isize - y as isize;
                
                if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                    buffer[py as usize * WIDTH + px as usize] = color;
                    buffer[ny as usize * WIDTH + nx as usize] = color;
                }
                if nx >= 0 && nx < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                    buffer[py as usize * WIDTH + nx as usize] = color;
                    buffer[ny as usize * WIDTH + px as usize] = color;
                }
            }
        }
    }
}

// Draw a line between two points using Bresenham's line algorithm
fn draw_line(buffer: &mut [u32], start: (usize, usize), end: (usize, usize), color: u32) {
    let (x0, y0) = start;
    let (x1, y1) = end;
    let dx = (x1 as isize - x0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as isize - y0 as isize).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    while x != x1 as isize || y != y1 as isize {
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            buffer[y as usize * WIDTH + x as usize] = color;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
