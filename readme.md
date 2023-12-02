# `2^11` (`=2048``) written in Rust

Use the arrow-keys to slide the tiles around.
This line in `main.rs` defines the board size:
```rust
let mut board = Board::new(4, 4);
```

Uses `termion` for terminal ui and keyboard events.

```text
Score: [000360]
########################
##                    ##
##  8                 ##
##                    ##
##  4         2       ##
##                    ##
## 16    8            ##
##                    ##
## 64    2    2       ##
##                    ##
########################
```