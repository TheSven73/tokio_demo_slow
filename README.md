This crate demonstrates a possible performance issue with tokio,
compared to synchronous code, and to `async-std`.

All three benchmarks do the same thing: read a file in a thread,
send the file data chunks through a channel to another thread,
where an md5 hash is computed.

On a dual Intel i5-3340M, Linux 4.15 (Mint):
```console
$ cargo bench
md5/Synchronous         time:   [148.33 ms 149.13 ms 150.20 ms]
md5/async-std           time:   [146.35 ms 146.81 ms 147.30 ms]
md5/tokio               time:   [1.2348  s 1.2609  s 1.2890  s]
md5/smol                time:   [192.35 ms 195.45 ms 198.22 ms]
```
