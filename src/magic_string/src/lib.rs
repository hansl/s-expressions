mod chunk;
mod error;
mod magic_string;

pub use magic_string::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
