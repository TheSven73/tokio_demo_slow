This crate demonstrates a possible performance issue with tokio,
compared to synchronous code, and to `async-std`.

All three benchmarks do the same thing: read a file in a thread,
send the file data chunks through a channel to another thread,
where an md5 hash is computed.

On a dual Intel i5-3340M, Linux 4.15 (Mint):
```console
$ cargo bench
md5/Synchronous         time:   [141.49 ms 141.77 ms 142.05 ms]
md5/async-std           time:   [140.05 ms 140.20 ms 140.36 ms]
md5/tokio               time:   [155.29 ms 156.67 ms 158.10 ms]
md5/smol                time:   [200.92 ms 201.73 ms 202.53 ms]
```
