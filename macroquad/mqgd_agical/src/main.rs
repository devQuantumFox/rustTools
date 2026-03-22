use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut r = 16.0;
    const MOVEMENT_SPEED: f32 = 200.0;

    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Right) { x += MOVEMENT_SPEED * delta_time; }
        if is_key_down(KeyCode::Left) { x -= MOVEMENT_SPEED * delta_time; }
        if is_key_down(KeyCode::Down) { y += MOVEMENT_SPEED * delta_time; }
        if is_key_down(KeyCode::Up) { y -= MOVEMENT_SPEED * delta_time; }

        x = clamp(x, 0.0 + r, screen_width() - r);
        y = clamp(y, 0.0 + r, screen_height() - r);

        draw_circle(x, y, r, YELLOW);

        next_frame().await
    }
}