extern "C" { pub fn c_api(); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe { c_api() };
    }
}
