pub fn gl_check_errors() {
    unsafe {
        let mut err = gl::GetError();

        while err != gl::NO_ERROR {
            println!("gl error: {}", err);
            err = gl::GetError();
        }
    }
}
