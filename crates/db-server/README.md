# Stupid-Db (Server)

A really terrible "database" I plan on writing in Rust. Mostly inspired by this [blog series](https://kakku.org/writing-a-simple-database-part-1/) and [accompanying repo](https://github.com/amitt001/moodb/) which looks pretty awesome but unfortunately was originally written in Go *shudder*. **Luckily I'm here to correct that mistake.**

## Various Moving Parts:
- [ ] The DB will have a [client](../db-client/) and [server](./) architecture
- [ ] It will store key-value data
- [ ] DB will be completely in-memory (to start)
- [ ] Backed by Write-ahead logs(WAL)
- [ ] Fault tolerance and restore in case of failure using WAL
- [ ] The latest snapshot of the in-memory DB will be backed up time-to-time in the file system
