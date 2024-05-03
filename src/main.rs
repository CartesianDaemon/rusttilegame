use macroquad::prelude::*;

use std::collections::LinkedList;

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

struct Angel {
    // Number of squares on each side of map.
    squares: i16,
    // Size of grid squares. Set proportional to window size at start of current frame.
    sq_size: f32,
    // Coordinates of fruit (soon to be character).
    fruit: Point,
}

// Draw a tile's texture given the object's window coordinates.
fn draw_sq(
    a: &Angel,
    tex: &Texture2D,
    x: f32,
    y: f32,
) {
    draw_texture_ex(
        &tex,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(a.sq_size, a.sq_size)),
            ..Default::default()
        },
    );
}

#[macroquad::main("Snake")]
async fn main() {
    let tex_crab: Texture2D = load_texture("imgs/ferris.png").await.unwrap();

    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
        body: LinkedList::new(),
    };
    let mut a = Angel { squares: 16, sq_size: 32.0, fruit: (0,0) };

    a.fruit = (rand::gen_range(0, a.squares), rand::gen_range(0, a.squares));

    let mut score = 0;
    let mut speed = 0.3;
    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut game_over = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    let mut last_key_pressed : Option<KeyCode> = None;

    loop {
        if !game_over {
            /*
            if is_key_down(KeyCode::Right) && snake.dir != left && !navigation_lock {
                snake.dir = right;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Left) && snake.dir != right && !navigation_lock {
                snake.dir = left;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Up) && snake.dir != down && !navigation_lock {
                snake.dir = up;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Down) && snake.dir != up && !navigation_lock {
                snake.dir = down;
                navigation_lock = true;
            }
            */

            // TODO: Way of expressing assign and test on same line?
            let key = get_last_key_pressed();
            if key.is_some() {
                last_key_pressed = key;
            }

            // Update grid with all beings moving.
            if get_time() - last_update > speed {
                // Remember time we drew current frame, to know when to draw next frame.
                last_update = get_time();

                // Move snake

                // add old head to top of body (LISP thanks us :))
                snake.body.push_front(snake.head);

                // if snake on same row xor column as fruit, change dir to face fruit
                if (snake.head.0 == a.fruit.0) != (snake.head.1 == a.fruit.1) {
                    snake.dir = ((a.fruit.0 - snake.head.0).signum(),(a.fruit.1 - snake.head.1).signum())
                }

                // move head to new location
                snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
                if snake.head == a.fruit {
                    // If new head is on fruit, eat it. Body is already the right length.
                    a.fruit = (rand::gen_range(0, a.squares), rand::gen_range(0, a.squares));
                    score += 100;
                    speed *= 0.9;
                } else {
                    // If snake didn't eat anything, remove tip of tail.
                    snake.body.pop_back();
                }
                // die if head out of bounds
                if snake.head.0 < 0
                    || snake.head.1 < 0
                    || snake.head.0 >= a.squares
                    || snake.head.1 >= a.squares
                {
                    game_over = true;
                }
                // die if head intersects body
                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        game_over = true;
                    }
                }
                navigation_lock = false;

                // Move character
                if let Some(key) = last_key_pressed {
                    match key {
                        KeyCode::Left  => a.fruit.0 -= 1,
                        KeyCode::Right => a.fruit.0 += 1,
                        KeyCode::Up    => a.fruit.1 -= 1,
                        KeyCode::Down  => a.fruit.1 += 1,
                        _ => (),
                    }
                }
                last_key_pressed = None;
            }
        }
        if !game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            a.sq_size = (screen_height() - offset_y * 2.) / a.squares as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            for i in 1..a.squares {
                draw_line(
                    offset_x,
                    offset_y + a.sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + a.sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..a.squares {
                draw_line(
                    offset_x + a.sq_size * i as f32,
                    offset_y,
                    offset_x + a.sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            draw_rectangle(
                offset_x + snake.head.0 as f32 * a.sq_size,
                offset_y + snake.head.1 as f32 * a.sq_size,
                a.sq_size,
                a.sq_size,
                DARKGREEN,
            );

            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * a.sq_size,
                    offset_y + *y as f32 * a.sq_size,
                    a.sq_size,
                    a.sq_size,
                    LIME,
                );
            }

            draw_rectangle(
                offset_x + a.fruit.0 as f32 * a.sq_size,
                offset_y + a.fruit.1 as f32 * a.sq_size,
                a.sq_size,
                a.sq_size,
                GOLD,
            );

            draw_sq(
                &a,
                &tex_crab,
                offset_x + a.fruit.0 as f32 * a.sq_size,
                offset_y + a.fruit.1 as f32 * a.sq_size,
            );

            draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                a.fruit = (rand::gen_range(0, a.squares), rand::gen_range(0, a.squares));
                score = 0;
                speed = 0.3;
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await;
    }
}
