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
        // You can use HJKL or arrow keys to move
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
            x -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            x += MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::K) {
            y -= MOVE_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::J) {
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
