use std::backtrace::Backtrace;

pub fn gl_check_errors() {
    unsafe {
        let mut err = gl::GetError();

        while err != gl::NO_ERROR {
            let trace = Backtrace::capture();
            match trace.status() {
                std::backtrace::BacktraceStatus::Captured => {
                    println!("gl error: {}\n{}", err, trace);
                },
                _ => {
                    println!("gl error: {}", err);
                }
            }

            err = gl::GetError();
        }
    }
}
