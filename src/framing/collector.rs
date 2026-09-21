#[derive(Debug)]
pub enum CollectorError {
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Collecting,
    Discarding,
}

#[derive(Debug)]
pub struct FrameCollector<const N: usize> {
    buffer: [u8; N],
    length: usize,
    state: State,
}

impl<const N: usize> FrameCollector<N> {
    pub const fn new() -> Self {
        const {
            assert!(N > 0, "Capacity must be positive");
        }
        Self {
            buffer: [0; N],
            length: 0,
            state: State::Collecting,
        }
    }

    pub fn push(&mut self, byte: u8) -> Result<Option<&[u8]>, CollectorError> {
        if byte == 0 {
            match self.state {
                State::Discarding => {
                    self.state = State::Collecting;
                    self.length = 0;
                    return Ok(None);
                }
                State::Collecting => {
                    if self.length == 0 {
                        return Ok(None);
                    }
                    let frame_size = self.length;
                    self.length = 0;
                    return Ok(Some(&self.buffer[..frame_size]));
                }
            }
        }
        match self.state {
            State::Discarding => Ok(None),
            State::Collecting => {
                if self.length == N {
                    self.length = 0;
                    self.state = State::Discarding;
                    return Err(CollectorError::Overflow);
                }
                self.buffer[self.length] = byte;
                self.length += 1;
                Ok(None)
            }
        }
    }
}

impl<const N: usize> Default for FrameCollector<N> {
    fn default() -> Self {
        Self::new()
    }
}
