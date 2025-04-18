use macroquad::prelude::*;

#[macroquad::main("🚀 Space Invaders")]
async fn main() {
    const MOVE_SPEED: f32 = 200.0;
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    loop {
        clear_background(SKYBLUE);

        // Draw your game here
        draw_text("Hello, Space Invaders!", 0.0, 20.0, 30.0, WHITE);

        // Handle input
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Left) {
            x -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            x += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVE_SPEED * delta_time;
        }

        // Keep the player within the screen bounds
        x = x.clamp(0.0, screen_width());
        y = y.clamp(0.0, screen_height());

        // Draw a circle at the current position
        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await;
    }
}
