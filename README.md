# arp-track


### Build
```
cargo build --release
```

### Run

You'll need to run as `sudo` so [arplib](https://github.com/drbh/arplib) can directly access the network interface and send packets.

```bash
sudo ./target/release/arp-track
```

### What's stored

Each time a ARP scan is run, the program will attempt to save the results in a SQLite + JSON databased managed by [hotpot-db](https://github.com/drbh/hotpot-db). 

It will automatically create the database if it doesn't already exist. On each write the console will also printout the last timestamp

```
Last update: 1582827565745
Last update: 1582827686817
Last update: 1582827807761
Last update: 1582827928725
```
