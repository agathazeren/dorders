# DOrders
Trustless and GMless order submission system for Diplomacy

## Building/Running
To build the client side code,
```bash
cd client
cargo web deploy
```
To run the server,
```bash
cd serve
cargo run
```


## Algorithm
All parties release the hash of their orders.
Once all parties have done so, all parties release their orders. These can then be checked against the hashes

