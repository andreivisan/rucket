pub mod encoding;

use encoding::cobs;

pub fn test_my_shit() {
    cobs::encode();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_my_shit();
    }
}
