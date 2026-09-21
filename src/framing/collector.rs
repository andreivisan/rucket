#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventKind {
    Pending,
    FrameReady,
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Event {
    kind: EventKind,
    data: u8,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Collecting,
    Discarding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCollector {

}


