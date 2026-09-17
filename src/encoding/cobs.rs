use super::{DecodingError, Encoder, EncodingError};

pub struct Cobs;

impl Encoder for Cobs {
    fn encode(input: &[u8], encoded: &mut [u8]) -> Result<usize, EncodingError> {
        if encoded.len() <= input.len() {
            return Err(EncodingError::OutputBufferTooSmall);
        }

        let mut write_index: usize = 1;
        let mut code_index: usize = 0;
        let mut code: u8 = 1;

        for byte_in in input {
            if *byte_in == 0 {
                write_code(encoded, &mut code, &mut code_index, &mut write_index)?;
            } else {
                let write_slot = encoded
                    .get_mut(write_index)
                    .ok_or(EncodingError::OutputBufferTooSmall)?;
                *write_slot = *byte_in;
                write_index += 1;
                code += 1;
                if code == 0xFF {
                    write_code(encoded, &mut code, &mut code_index, &mut write_index)?;
                }
            }
        }

        let final_code = encoded
            .get_mut(code_index)
            .ok_or(EncodingError::OutputBufferTooSmall)?;
        *final_code = code;
        Ok(write_index)
    }

    fn decode(input: &[u8], decoded: &mut [u8]) -> Result<usize, DecodingError> {
        if input.is_empty() {
            return Err(DecodingError::InputBufferEmpty);
        }
        let mut read_index: usize = 0;
        let mut write_index: usize = 0;
        let in_size: usize = input.len();

        while read_index < in_size {
            let code = input[read_index];
            if code == 0 || read_index + usize::from(code) > in_size { 
                return Err(DecodingError::MalformedInput);
            }
            read_index += 1;
            for _ in 1..code {
                let byte = input[read_index];
                if byte == 0 {
                    return Err(DecodingError::MalformedInput);
                } 
                let write_slot = decoded
                    .get_mut(write_index)
                    .ok_or(DecodingError::OutputBufferTooSmall)?;
                *write_slot = input[read_index];
                write_index += 1;
                read_index += 1;
            }
            if code != 0xFF && read_index != in_size {
                let write_slot = decoded
                    .get_mut(write_index)
                    .ok_or(DecodingError::OutputBufferTooSmall)?;
                *write_slot = b'\0';
                write_index += 1;
            }
        }
        Ok(write_index)
    }
}

fn write_code(
    encoded: &mut [u8],
    code: &mut u8,
    code_index: &mut usize,
    write_index: &mut usize,
) -> Result<(), EncodingError> {
    let code_slot = encoded
        .get_mut(*code_index)
        .ok_or(EncodingError::OutputBufferTooSmall)?;
    *code_slot = *code;
    *code = 1;
    *code_index = *write_index;
    *write_index += 1;
    Ok(())
}
