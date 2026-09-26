use rucket::{
    encoding::{Encoder, cobs::Cobs},
    framing::collector::FrameCollector,
};

#[test]
fn packet_round_trip() {
    let payload: Vec<u8> = vec![0xAA, 0x00, 0xBB, 0xCC];
    let mut encoded: Vec<u8> = vec![0u8; 32];
    let encoded_size = Cobs::encode(&payload, &mut encoded).expect("Encoding error");
    let expected: Vec<u8> = vec![0x02, 0xAA, 0x03, 0xBB, 0xCC];
    assert_eq!(encoded_size, expected.len());
    assert_eq!(&encoded[..encoded_size], &expected[..]);

    let mut frame_collector: FrameCollector<32> = FrameCollector::new();
    for &byte in &encoded[..encoded_size] {
        let frame = frame_collector.push(byte).expect("Collector overflow");
        assert!(frame.is_none());
    }
    let frame = frame_collector
        .push(0x00)
        .expect("Collector overflow")
        .expect("Expected a completed frame");
    assert_eq!(frame, &expected[..]);

    let mut decoded: Vec<u8> = vec![0u8; 32];
    let decoded_size = Cobs::decode(frame, &mut decoded).expect("Decoding error");
    assert_eq!(decoded_size, payload.len());
    assert_eq!(&decoded[..decoded_size], &payload[..]);
}
