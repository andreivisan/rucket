use super::{Encoder, EncodingError};

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
                let byte_en = encoded
                    .get_mut(write_index)
                    .ok_or(EncodingError::OutputBufferTooSmall)?;
                *byte_en = *byte_in;
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

    fn decode(input: &[u8], decoded: &mut [u8]) -> Result<usize, EncodingError> {
        println!("Decode the COBS way");
        Ok(0 as usize)
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
