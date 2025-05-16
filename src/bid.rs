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