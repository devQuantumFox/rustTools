use macroquad::prelude::*;


struct Shape { size: f32, speed: f32, x: f32, y: f32, }

#[macroquad::main("My game")]

async fn main() {

    const MOVEMENT_SPEED: f32 = 200.0;
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0, speed: MOVEMENT_SPEED, x: screen_width()/2.0, y: screen_height()/2.0
    };

    loop {
        clear_background(DARKPURPLE);
        let frame_time = get_frame_time();
        let velocity = MOVEMENT_SPEED * frame_time;

        if is_key_down(KeyCode::Right) { circle.x += velocity; }
        if is_key_down(KeyCode::Left) { circle.x -= velocity; }
        if is_key_down(KeyCode::Down) { circle.y += velocity; }
        if is_key_down(KeyCode::Up) { circle.y -= velocity; }

        circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
        circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size, speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size/2.0, screen_width()-size/2.0),
                y: -size
            });
        }
        for square in &mut squares { square.y += square.speed * frame_time; }
        squares.retain(|square| square.y < screen_height() + square.size);

        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for square in &squares {
            draw_rectangle(square.x - square.size / 2.0, square.y - square.size / 2.0, square.size, square.size, GREEN);
        }

        next_frame().await
    }
}