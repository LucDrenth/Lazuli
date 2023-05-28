/*********************
 * Ansi colors codes *
 *********************
 *
 * "\x1B[31m"   RED
 * "\x1B[33m"   YELLOW
 * "\x1B[0m"    RESET
 */

#[macro_export]
macro_rules! lz_core_info {
    ($($arg:tt)*) => {{
        use chrono::Local;
        let local = Local::now();

        println!("[{}] ENGINE: {}", local.format("%H:%M:%S"), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! lz_core_warn {
    ($($arg:tt)*) => {
        use chrono::Local;

        println!("{}[{}] ENGINE: {}{}",  "\x1B[33m", Local::now().format("%H:%M:%S"), format!($($arg)*), "\x1B[0m");
    };
}

#[macro_export]
macro_rules! lz_core_err {
    ($($arg:tt)*) => {
        use chrono::Local;

        println!("{}[{}] ENGINE: {}{}", "\x1B[31m", Local::now().format("%H:%M:%S"), format!($($arg)*), "\x1B[0m");
    };
}


#[macro_export]
macro_rules! lz_info {
    ($($arg:tt)*) => {
        use chrono::Local;

        println!("[{}] APP: {}", Local::now().format("%H:%M:%S"), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! lz_warn {
    ($($arg:tt)*) => {
        use chrono::Local;

        println!("{}[{}] APP: {}{}", "\x1B[33m", Local::now().format("%H:%M:%S"), format!($($arg)*), "\x1B[0m");
    };
}

#[macro_export]
macro_rules! lz_err {
    ($($arg:tt)*) => {
        use chrono::Local;
        
        println!("{}[{}] APP: {}{}", "\x1B[31m", Local::now().format("%H:%M:%S"), format!($($arg)*), "\x1B[0m");
    };
}
