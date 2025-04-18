use macroquad::prelude::*;

#[macroquad::main("🚀 Space Invaders")]
async fn main() {
    loop {
        clear_background(SKYBLUE);

        // Draw your game here
        draw_text("Hello, Space Invaders!", 0.0, 20.0, 30.0, WHITE);

        next_frame().await;
    }
}
