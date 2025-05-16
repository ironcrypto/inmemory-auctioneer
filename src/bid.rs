use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::cmp::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: Uuid,
    pub value: u64,
    pub timestamp: u128,
    pub builder_id: String,
    pub nonce: u32,
}

impl Bid {
    pub fn new(value: u64, builder_id: String) -> Self {
        Bid {
            id: Uuid::new_v4(),
            value,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            builder_id,
            nonce: rand::random::<u32>(),
        }
    }

    pub fn compare_to(&self, other: &Bid) -> Ordering {
        match self.value.cmp(&other.value) {
            Ordering::Equal => match self.timestamp.cmp(&other.timestamp) {
                Ordering::Equal => self.builder_id.cmp(&other.builder_id),
                t => t,
            },
            v => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_bid_ordering() {
        let mut bid1 = Bid::new(1000, "builder_a".into());
        let mut bid2 = bid1.clone();
        bid2.value = 1200;

        assert_eq!(bid2.compare_to(&bid1), Ordering::Greater);
        assert_eq!(bid1.compare_to(&bid2), Ordering::Less);
    }
}