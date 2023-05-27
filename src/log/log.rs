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
        use chrono::Local;
        
        Local::now().format("%H:%M:%S")
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
