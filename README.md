This crate demonstrates a possible performance issue with tokio,
compared to synchronous code, and to `async-std`.

All three benchmarks do the same thing: read a file in a thread,
send the file data chunks through a channel to another thread,
where an md5 hash is computed.

On a dual Intel i5-3340M, Linux 4.15 (Mint):
```console
$ cargo bench
md5/Synchronous         time:   [145.19 ms 145.45 ms 145.72 ms]
md5/async-std           time:   [143.16 ms 143.36 ms 143.59 ms]
md5/tokio               time:   [728.15 ms 730.27 ms 732.42 ms]
md5/smol                time:   [165.09 ms 167.82 ms 170.67 ms]
```
