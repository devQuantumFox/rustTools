use macroquad::prelude::*;

#[macroquad::main("My First Game")]
async fn main() {
    loop {
        clear_background(RED);
        draw_text("HELLO MACROQUAD!", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
}