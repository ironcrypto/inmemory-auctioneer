# Dev Plan: Week-by-Week Outline

## Phase 1 — Setup & Local Auctioneer
- Create project structure
- Implement Bid struct
- Implement in-memory Auctioneer with locking
- Write basic CLI or testbed in main.rs
- Benchmark local latency (e.g., using std::time::Instant)

## Phase 2 — Gossip Layer
- Implement UDP-based gossip (broadcast + receive)
- Spawn multiple instances as local async tasks
- Verify eventual convergence on best bid
- Add replay protection or ordering validation

## Phase 3 — Simulation & Benchmarks
- Simulate N relay nodes receiving builder bids
- Measure time to convergence (local + injected latency)
- Test memory pressure, edge cases (e.g. race conditions)
- Optionally: model slot boundary and expiry

## Phase 4 — Polish & Publish
- Finalize docs and README
- Add criterion benchmarks (or your own profiler)
- Push to GitHub with examples and test cases
- Optionally: record a quick demo/GIF