use std::backtrace::Backtrace;

pub fn gl_check_errors() {
    unsafe {
        let mut err = gl::GetError();

        while err != gl::NO_ERROR {
            let err_string = error_to_string(err);
            let trace = Backtrace::capture();

            match trace.status() {
                std::backtrace::BacktraceStatus::Captured => {
                    println!("gl error: {}\n{}", err_string, trace);
                },
                _ => {
                    println!("gl error: {}", err_string);
                }
            }

            err = gl::GetError();
        }
    }
}

pub fn gl_clear_errors() {
    unsafe {
        while gl::GetError() != gl::NO_ERROR {}
    }
}

pub fn error_to_string(err: u32) -> String {
    match err {
        gl::INVALID_ENUM => String::from("INVALID_ENUM"),
        gl::INVALID_VALUE => String::from("INVALID_VALUE"),
        gl::INVALID_OPERATION => String::from("INVALID_OPERATION"),
        gl::INVALID_FRAMEBUFFER_OPERATION => String::from("INVALID_FRAMEBUFFER_OPERATION"),
        gl::INVALID_INDEX => String::from("INVALID_INDEX"),
        gl::STACK_OVERFLOW => String::from("STACK_OVERFLOW"),
        gl::STACK_UNDERFLOW => String::from("STACK_UNDERFLOW"),
        gl::OUT_OF_MEMORY => String::from("OUT_OF_MEMORY"),
        gl::CONTEXT_LOST => String::from("CONTEXT_LOST"),
        _ => String::from("UNKNOWN_ERROR")
    }
}
