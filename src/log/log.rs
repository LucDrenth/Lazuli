const COLOR_RED: &str = "\x1B[31m";
const COLOR_YELLOW: &str = "\x1B[33m";
const COLOR_RESET: &str = "\x1B[0m";

const SOURCE_ENGINE: &str = "ENGINE";
const SOURCE_APP: &str = "APP";

pub fn engine_info(msg: impl Into<String>) { log("", SOURCE_ENGINE, msg); }
pub fn engine_warn(msg: impl Into<String>) { log(COLOR_YELLOW, SOURCE_ENGINE, msg); }
pub fn engine_err(msg: impl Into<String>) { log(COLOR_RED, SOURCE_ENGINE, msg); }

pub fn info(msg: impl Into<String>) { log("", SOURCE_APP, msg); }
pub fn warn(msg: impl Into<String>) { log(COLOR_YELLOW, SOURCE_APP, msg); }
pub fn err(msg: impl Into<String>) { log(COLOR_RED, SOURCE_APP, msg); }

fn log(color: &str, source: &str, msg: impl Into<String>) {
    let message = msg.into();
    let time = chrono::Local::now().format("%H:%M:%S");
    let message_to_log = format!("{color}[{time}] {source}: {message}{COLOR_RESET}");

    println!("{message_to_log}");
}
