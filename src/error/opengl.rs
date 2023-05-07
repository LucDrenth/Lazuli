pub fn gl_check_errors() {
    unsafe {
        let mut err = gl::GetError();

        while err != gl::NO_ERROR {
            print!("gl error: {}", err);
            err = gl::GetError();
        }
    }
}
