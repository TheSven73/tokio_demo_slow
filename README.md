This crate demonstrates a possible performance issue with tokio,
compared to synchronous code, and to `async-std`.

All three benchmarks do the same thing: read a file in a thread,
send the file data chunks through a channel to another thread,
where an md5 hash is computed.

On a dual Intel i5-3340M, Linux 4.15 (Mint):
```console
$ cargo bench
md5/Synchronous         time:   [142.67 ms 142.97 ms 143.29 ms]
md5/async-std           time:   [140.32 ms 140.85 ms 141.46 ms]
md5/tokio               time:   [157.68 ms 159.05 ms 160.45 ms]
md5/smol                time:   [193.95 ms 196.44 ms 198.67 ms]
```
