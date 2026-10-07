#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record<T> {
    pub sequence: u64,
    pub payload: T,
}

#[derive(Debug)]
pub struct MemoryLog<T> {
    records: Vec<Record<T>>,
    next_sequence: u64,
}

impl<T> Default for MemoryLog<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MemoryLog<T> {
    pub fn new() -> Self {
        MemoryLog {
            records: Vec::new(),
            next_sequence: 1,
        }
    }

    pub fn append(&mut self, payload: T) -> u64 {
        let sequence = self.next_sequence;
        self.records.push(Record { sequence, payload });
        self.next_sequence += 1;
        sequence
    }

    pub fn snapshot(&self) -> Vec<Record<T>>
    where
        T: Clone,
    {
        let records = &self.records;
        records.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_payload() {
        let mut mem_log = MemoryLog::<String>::new();
        assert_eq!(mem_log.snapshot().len(), 0);
        let id_1 = mem_log.append("first".to_owned());
        assert_eq!(id_1, 1);
        let id_2 = mem_log.append("second".to_owned());
        assert_eq!(id_2, 2);
        let records = mem_log.snapshot();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].sequence, 1);
        assert_eq!(records[0].payload, "first");
        assert_eq!(records[1].sequence, 2);
        assert_eq!(records[1].payload, "second");
        let id_3 = mem_log.append("third".to_owned());
        assert_eq!(id_3, 3);
        assert_eq!(records.len(), 2);
        let updated = mem_log.snapshot();
        assert_eq!(updated.len(), 3);
        assert_eq!(updated[2].sequence, 3);
        assert_eq!(updated[2].payload, "third");
    }
}
