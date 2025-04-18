use macroquad::prelude::*;

struct Shape {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
}

impl Shape {
    fn rect(&self) -> Rect {
        Rect::new(
            self.x - self.size / 2.0,
            self.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }

    fn collides_with(&self, other: &Shape) -> bool {
        self.rect().overlaps(&other.rect())
    }
}

#[macroquad::main("🚀 Space Invaders")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    const MOVE_SPEED: f32 = 200.0;
    let mut squares = vec![];
    let mut circle = Shape {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: 32.0,
        speed: MOVE_SPEED,
    };
    let mut game_over = false;
    loop {
        clear_background(SKYBLUE);

        // Draw your game here
        draw_text(
            "Hello, Space Invaders! Use arrow keys to move, and space to restart.",
            0.0,
            20.0,
            30.0,
            WHITE,
        );

        if !game_over {
            // Handle input
            // You can use HJKL or arrow keys to move
            let delta_time = get_frame_time();
            if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
                circle.x -= MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
                circle.x += MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::K) {
                circle.y -= MOVE_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::J) {
                circle.y += MOVE_SPEED * delta_time;
            }

            // Keep the player within the screen bounds
            circle.x = circle.x.clamp(0.0, screen_width());

            circle.y = circle.y.clamp(0.0, screen_height());

            // Add enemy squares
            if rand::gen_range(0, 99) > 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                });
            }

            // Update enemy squares
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            // Remove squares that are off-screen
            squares.retain(|square| square.y < screen_height() + square.size);

            if squares.iter().any(|square| square.collides_with(&circle)) {
                game_over = true;
            }
        }

        // Draw a circle at the current position
        draw_circle(circle.x, circle.y, 16.0, YELLOW);

        // Draw the squares
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        // Draw game over text
        if game_over {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        // Reset the game if the player presses space
        if game_over && is_key_pressed(KeyCode::Space) {
            game_over = false;
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
        }
        next_frame().await;
    }
}
