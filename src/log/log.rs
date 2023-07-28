const COLOR_RED: &str = "\x1B[31m";
const COLOR_YELLOW: &str = "\x1B[33m";
const COLOR_RESET: &str = "\x1B[0m";

const SOURCE_ENGINE: &str = "ENGINE";
const SOURCE_APP: &str = "APP";

pub fn engine_info(msg: String) { log("", SOURCE_ENGINE, msg); }
pub fn engine_warn(msg: String) { log(COLOR_YELLOW, SOURCE_ENGINE, msg); }
pub fn engine_err(msg: String) { log(COLOR_RED, SOURCE_ENGINE, msg); }

pub fn info(msg: String) { log("", SOURCE_APP, msg); }
pub fn warn(msg: String) { log(COLOR_YELLOW, SOURCE_APP, msg); }
pub fn err(msg: String) { log(COLOR_RED, SOURCE_APP, msg); }

fn log(color: &str, source: &str, msg: String) {
    let time = chrono::Local::now().format("%H:%M:%S");
    let message_to_log = format!("{color}[{time}] {source}: {msg}{COLOR_RESET}");

    println!("{message_to_log}");
}
