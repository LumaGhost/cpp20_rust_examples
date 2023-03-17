
extern {
    fn hello_from_cpp();
}

fn main() {
    unsafe { hello_from_cpp() };
}
