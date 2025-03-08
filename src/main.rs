use macroquad::prelude::*;
use std::time::{Duration, Instant};

fn window_conf() -> Conf {
    Conf {
        window_title: "Reaction speed".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[derive(Eq, PartialEq)]
enum State {
    Paused,
    Waiting,
    Go,
    Failed,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut text_enabled = true;
    let mut state = State::Paused;

    let mut start_time = Instant::now();
    let mut duration = Duration::from_secs(0);

    let mut scores: Vec<Duration> = Vec::new();

    loop {
        if state == State::Waiting {
            if Instant::now() >= start_time + duration {
                state = State::Go;
            }
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_key_pressed(KeyCode::T) {
            text_enabled = !text_enabled;
        }

        if is_key_pressed(KeyCode::R) {
            state = State::Paused;
            scores.clear();
        }

        if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
            match state {
                State::Paused | State::Failed => {
                    start_time = Instant::now();
                    duration = Duration::from_millis(rand::gen_range(1500, 5000));
                    state = State::Waiting;
                }
                State::Waiting => {
                    state = State::Failed;
                }
                State::Go => {
                    state = State::Paused;
                    scores.push(Instant::now().duration_since(start_time + duration));
                }
            }
        }

        match state {
            State::Paused => {
                let last_5 = if scores.len() == 0 { "none".to_string() } else {
                    let mut scores_str = String::new();

                    for score in scores.iter().rev().take(5).rev() {
                        scores_str.push_str(&format!("{}ms ", score.as_millis()));
                    }

                    scores_str
                };

                let last_5_average = {
                    let sum: u128 = scores.iter().rev().take(5).rev().fold(Duration::from_millis(0), |acc, &x| acc + x).as_millis();
                    let len = if scores.len() > 5 { 5 } else { scores.len() } as u128;

                    if len == 0 {
                        "N/A".to_owned()
                    } else {
                        (sum / len).to_string() + "ms"
                    }
                };

                let total_average = {
                    let sum: u128 = scores.iter().fold(Duration::from_millis(0), |acc, &x| acc + x).as_millis();
                    let len = scores.len() as u128;

                    if len == 0 {
                        "N/A".to_owned()
                    } else {
                        (sum / len).to_string() + "ms"
                    }
                };

                clear_background(WHITE);
                if text_enabled {
                    draw_text(&format!("Click to start"), 20.0, 20.0, 30.0, BLACK);
                    draw_text(&format!("Last 5: {}", last_5), 20.0, 50.0, 30.0, BLACK);
                    draw_text(&format!("Average of last 5: {}", last_5_average), 20.0, 80.0, 30.0, BLACK);
                    draw_text(&format!("Total average: {}", total_average), 20.0, 110.0, 30.0, BLACK);
                }
            }
            State::Waiting => {
                clear_background(BLUE);
                if text_enabled {
                    draw_text("Wait for green", 20.0, 20.0, 30.0, BLACK);
                }
            }
            State::Go => {
                clear_background(GREEN);

                if text_enabled {
                    draw_text("Click", 20.0, 20.0, 30.0, BLACK);
                }
            }
            State::Failed => {
                clear_background(RED);

                if text_enabled {
                    draw_text("Too early", 20.0, 20.0, 30.0, BLACK);
                }
            }
        }

        next_frame().await
    }
}
