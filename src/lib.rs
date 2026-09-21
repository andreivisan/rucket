pub mod encoding;
pub mod framing;

pub fn test_my_shit() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_my_shit();
    }
}
