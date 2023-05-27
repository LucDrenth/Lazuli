/*********************
 * Ansi colors codes *
 *********************
 *
 * "\x1B[31m"   RED
 * "\x1B[33m"   YELLOW
 * "\x1B[0m"    RESET
 */

#[macro_export]
macro_rules! current_time {
    () => {{
        use std::time::{SystemTime, UNIX_EPOCH};

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to obtain system time.")
            .as_secs();

        // TODO get this from the system timezone
        let timezone_offset = 2;

        let hours = (current_time / 3600) % 24;
        let minutes = (current_time / 60) % 60;
        let seconds = current_time % 60;

        format!("{:02}:{:02}:{:02}", hours + timezone_offset, minutes, seconds)
    }};
}


#[macro_export]
macro_rules! lz_core_info {
    ($($arg:tt)*) => {
        println!("[{}] ENGINE: {}", current_time!(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! lz_core_warn {
    ($($arg:tt)*) => {
        println!("{}[{}] ENGINE: {}{}",  "\x1B[33m", current_time!(), format!($($arg)*), "\x1B[0m");
    };
}

#[macro_export]
macro_rules! lz_core_err {
    ($($arg:tt)*) => {
        println!("{}[{}] ENGINE: {}{}", "\x1B[31m", current_time!(), format!($($arg)*), "\x1B[0m");
    };
}


#[macro_export]
macro_rules! lz_info {
    ($($arg:tt)*) => {
        println!("[{}] APP: {}", current_time!(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! lz_warn {
    ($($arg:tt)*) => {
        println!("{}[{}] APP: {}{}", "\x1B[33m", current_time!(), format!($($arg)*), "\x1B[0m");
    };
}

#[macro_export]
macro_rules! lz_err {
    ($($arg:tt)*) => {
        println!("{}[{}] APP: {}{}", "\x1B[31m", current_time!(), format!($($arg)*), "\x1B[0m");
    };
}
