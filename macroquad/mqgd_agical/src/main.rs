use macroquad::{prelude::*, rand::ChooseRandom};


struct Shape { size: f32, speed: f32, x: f32, y: f32, color: Color, }
impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        let rect = other.rect();

        let radius = self.size / 2.0;
        
        let close_x = clamp(self.x + radius, rect.x, rect.x + rect.w);
        let close_y = clamp(self.y + radius, rect.y, rect.y + rect.h);

        let dist_x = self.x + radius - close_x;
        let dist_y = self.y + radius - close_y;

        (dist_x * dist_x + dist_y * dist_y) < (radius * radius)
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("My game")]

async fn main() {

    const MOVEMENT_SPEED: f32 = 200.0;
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];
    let color = vec![RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
    let mut circle = Shape {
        size: 32.0, speed: MOVEMENT_SPEED, x: screen_width()/2.0, y: screen_height()/2.0, color: WHITE
    };

    let mut gameover = false;
    loop {
        clear_background(DARKPURPLE);
        let frame_time = get_frame_time();

        if !gameover {
            if is_key_down(KeyCode::Right) { circle.x += MOVEMENT_SPEED * frame_time; }
            if is_key_down(KeyCode::Left) { circle.x -= MOVEMENT_SPEED * frame_time; }
            if is_key_down(KeyCode::Down) { circle.y += MOVEMENT_SPEED * frame_time; }
            if is_key_down(KeyCode::Up) { circle.y -= MOVEMENT_SPEED * frame_time; }
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
        circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size, speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size/2.0, screen_width()-size/2.0),
                y: -size, color: *color.choose().unwrap()
            });
        }

        for square in &mut squares { square.y += square.speed * frame_time; }
        squares.retain(|square| square.y < screen_height() + square.size);

        draw_circle(circle.x, circle.y, circle.size, circle.color);

        for square in &squares {
            draw_rectangle(square.x - square.size / 2.0, square.y - square.size / 2.0, square.size, square.size, square.color);
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text, 
                screen_width() / 2.0 - text_dimensions.width / 2.0, 
                screen_height() / 2.0, 
                50.0, RED,);
        }

        next_frame().await
    }
}