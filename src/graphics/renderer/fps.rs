use crate::{time, log};

pub struct Fps {
    current_fps_count: u16,
    last_fps_measure_time: u128,
    print: bool,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            current_fps_count: 0,
            last_fps_measure_time: time::now_millis(),
            print: false,
        }
    }

    pub fn update_fps_count(&mut self) {
        let now = time::now_millis();

        self.current_fps_count += 1;

        if now - self.last_fps_measure_time > 1_000 {
            if self.print {
                log::engine_info(format!("fps: {}", self.current_fps_count));
            }

            self.last_fps_measure_time = now;
            self.current_fps_count = 0;
        }
    }
}
