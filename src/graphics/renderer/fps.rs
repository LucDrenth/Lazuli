use std::time::{SystemTime, UNIX_EPOCH};

pub struct Fps {
    current_fps_count: u16,
    last_fps_measure_time: u128,
    print: bool,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            current_fps_count: 0,
            last_fps_measure_time: now(),
            print: false,
        }
    }

    pub fn reset_last_fps_measure_time(mut self) {
        self.last_fps_measure_time = now();
    }

    pub fn update_fps_count(&mut self) {
        let now = now();

        self.current_fps_count += 1;

        if now - self.last_fps_measure_time > 1_000 {
            if self.print {
                println!("fps: {}", self.current_fps_count);
            }

            self.last_fps_measure_time = now;
            self.current_fps_count = 0;
        }
    }
}

fn now() -> u128 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
}
