use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // get time
        let time = get_time();
        let circle_pos = Vec2::new(
            screen_width() / 2.0 + time.sin() as f32 * 100.0,
            screen_height() / 2.0 + time.cos() as f32 * 100.0,
        );
        draw_circle(circle_pos.x, circle_pos.y, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
