
extern "C" { pub fn hello_from_cpp () ; }

fn main() {
    println!("Hello, world!");
    unsafe { hello_from_cpp() };
}
