
extern {
    fn hello_from_cpp();
}

pub fn hello_rust_cpp() {
    unsafe { hello_from_cpp() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        hello_rust_cpp();
    }
}

