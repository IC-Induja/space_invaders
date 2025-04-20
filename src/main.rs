use macroquad::prelude::*;
use std::fs;

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");

const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

enum GameState {
    MainMenu,
    Paused,
    Running,
    GameOver,
}

struct Shape {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    collided: bool,
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
    let mut score = 0;
    let mut high_score = fs::read_to_string("high_score.dat")
        .map_or(Ok(0), |s| s.trim().parse::<u32>())
        .unwrap_or(0);
    const MOVE_SPEED: f32 = 200.0;
    let mut squares = vec![];
    let mut bullets = vec![];
    let mut circle = Shape {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: 32.0,
        speed: MOVE_SPEED,
        collided: false,
    };
    let mut game_state = GameState::MainMenu;
    let (material, mut direction_modifier, render_target) = match initialize_shader() {
        Some((material, direction_modifier, render_target)) => {
            (Some(material), direction_modifier, Some(render_target))
        }
        None => (None, 0.0, None),
    };

    loop {
        clear_background(SKYBLUE);
        if let (Some(material), Some(render_target)) = (&material, &render_target) {
            draw_shader(material, direction_modifier, render_target);
        }

        match game_state {
            GameState::MainMenu => {
                let text = "Press SPACE to start";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Running;
                }
            }
            GameState::Paused => {
                let title_text = "PAUSED";
                let title_dimensions = measure_text(title_text, None, 50, 1.0);
                let text = "Press SPACE to resume";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    title_text,
                    screen_width() / 2.0 - title_dimensions.width / 2.0,
                    screen_height() / 2.0 - 50.0,
                    50.0,
                    WHITE,
                );
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Running;
                }
            }
            GameState::Running => {
                // Handle input
                // Pause the game if the player presses Escape
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }
                // You can use HJKL or arrow keys to move
                let delta_time = get_frame_time();
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
                    circle.x -= MOVE_SPEED * delta_time;
                    direction_modifier += 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
                    circle.x += MOVE_SPEED * delta_time;
                    direction_modifier -= 0.05 * delta_time;
                }
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::K) {
                    circle.y -= MOVE_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::J) {
                    circle.y += MOVE_SPEED * delta_time;
                }

                // Enable shooting bullets
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: circle.x,
                        y: circle.y,
                        size: 5.0,
                        speed: circle.speed * 2.0,
                        collided: false,
                    });
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
                        collided: false,
                    });
                }

                // Update enemy squares
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                // Update bullets
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                // Remove off-screen shapes
                squares.retain(|square| square.y < screen_height() + square.size);
                bullets.retain(|bullet| bullet.y < screen_height() + bullet.size);

                // Remove collided shapes
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                // Check for collisions between player and squares
                if squares.iter().any(|square| square.collides_with(&circle)) {
                    if score == high_score {
                        fs::write("high_score.dat", score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }

                // Check for collisions between bullets and squares
                for bullet in &mut bullets {
                    for square in &mut squares {
                        if bullet.collides_with(square) {
                            square.collided = true;
                            bullet.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                        }
                    }
                }
                // Draw the score
                // Current score on the left and high score on the right
                let score_text = format!("Score: {}", score);
                let high_score_text = format!("High Score: {}", high_score);
                let high_score_dimensions = measure_text(&high_score_text, None, 20, 1.0);
                draw_text(&score_text, 10.0, 20.0, 20.0, WHITE);
                draw_text(
                    &high_score_text,
                    screen_width() - high_score_dimensions.width - 10.0,
                    20.0,
                    20.0,
                    WHITE,
                );

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

                // Draw the bullets
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
                }
            }
            GameState::GameOver => {
                // Reset the game if the player presses space
                if is_key_pressed(KeyCode::Space) {
                    squares.clear();
                    bullets.clear();
                    score = 0;
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    game_state = GameState::Running;
                }
                let text = "GAME OVER! Press SPACE to restart";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
        }
        next_frame().await;
    }
}

fn initialize_shader() -> Option<(Material, f32, RenderTarget)> {
    let direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .ok();

    match material {
        Some(mat) => Some((mat, direction_modifier, render_target)),
        None => {
            println!("Failed to load shader");
            None
        }
    }
}

fn draw_shader(material: &Material, direction_modifier: f32, render_target: &RenderTarget) {
    material.set_uniform("iResolution", (screen_width(), screen_height()));
    material.set_uniform("direction_modifier", direction_modifier);
    gl_use_material(material);
    draw_texture_ex(
        &render_target.texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    gl_use_default_material();
}
