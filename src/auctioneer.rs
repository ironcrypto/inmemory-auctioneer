use crate::bid::Bid;
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct Auctioneer {
    best_bid: Option<Bid>,
    history: Vec<Bid>,
}

impl Auctioneer {
    pub fn new() -> Self {
        Self { best_bid: None, history: vec![] }
    }

    pub fn submit_bid(&mut self, bid: Bid) -> bool {
        if self.best_bid.as_ref().map_or(true, |b| bid.compare_to(b) == Ordering::Greater) {
            self.best_bid = Some(bid.clone());
            self.history.push(bid);
            true
        } else {
            false
        }
    }

    pub fn get_best_bid(&self) -> Option<Bid> {
        self.best_bid.clone()
    }

    pub fn history(&self) -> &[Bid] {
        &self.history
    }
}