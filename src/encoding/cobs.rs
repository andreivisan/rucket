pub fn encode() {
    println!("Encode the COBS way");
    // Original message: 11 22 00 33 00 44 55
    // to delimit the messages we remove 00 and append a real 00
    // to do that we add counting instructions called code bytes
    // COBS Encoded: 03 11 22 02 33 03 44 55 00
    // where 03 and 02 are e positions after which zeros must be added
    // (after split at zeros)
}
