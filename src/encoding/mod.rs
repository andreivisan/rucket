pub mod cobs;

#[derive(Debug)]
pub enum EncodingError {
    OutputBufferTooSmall,
}

pub trait Encoder {
    const MARKER: u8 = 0;

    fn encode(input: &[u8], encoded: &mut [u8]) -> Result<usize, EncodingError>;

    fn decode(input: &[u8], decoded: &mut [u8]) -> Result<usize, EncodingError>;
}
