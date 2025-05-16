#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use inmemory_auctioneer::bid::Bid;
    #[test]
    fn test_bid_ordering() {
        let mut bid1 = Bid::new(1000, "builder_a".into());
        let mut bid2 = bid1.clone();
        bid2.value = 1200;

        assert_eq!(bid2.compare_to(&bid1), Ordering::Greater);
        assert_eq!(bid1.compare_to(&bid2), Ordering::Less);
    }
}