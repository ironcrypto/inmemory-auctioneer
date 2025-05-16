mod auctioneer;
mod bid;

use auctioneer::Auctioneer;
use bid::Bid;

fn main() {
    env_logger::init();
    let mut auctioneer = Auctioneer::new();

    let bid = Bid::new(4200, "builder_xyz".to_string());
    auctioneer.submit_bid(bid.clone());

    println!("Best bid: {:?}", auctioneer.get_best_bid());
}
