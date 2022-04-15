
extern "C" { 
    pub fn zlibVersion() -> *const std::os::raw::c_char;
}

fn zlib_version_safe() -> String {
    unsafe {
        std::ffi::CStr::from_ptr(zlibVersion()).to_string_lossy().into_owned()
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", zlib_version_safe())
}
