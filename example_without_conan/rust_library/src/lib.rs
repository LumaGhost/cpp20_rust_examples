
extern "C" { pub fn hello_from_cpp(); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("Hello, world!");
        unsafe { hello_from_cpp() };
    }
}
