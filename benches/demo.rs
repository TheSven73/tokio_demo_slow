use criterion::{criterion_group, criterion_main, Criterion};
use tempfile::NamedTempFile;
use std::io::Write;
use std::fs::File;
use rand::Rng;

use tokio_slow_demo::{
    sync_benchmark,
    async_std_benchmark,
    tokio_benchmark,
    smol_benchmark,
};

fn make_random_tmpfile(size_mb: usize) -> NamedTempFile {
    let mut buf = [0u8; 1024 * 1024];
    let mut rng = rand::thread_rng();
    let mut tmp = NamedTempFile::new().unwrap();

    for _ in 0..size_mb {
        rng.fill(&mut buf[..]);
        tmp.write_all(&buf).unwrap();
    }
    tmp
}

fn bench_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("md5");

    let tmp = make_random_tmpfile(50);

    let tests = vec![
        ("Synchronous", sync_benchmark as fn(File)),
        ("async-std",   async_std_benchmark),
        ("tokio",       tokio_benchmark),
        ("smol",        smol_benchmark),
    ];

    for (name, function) in tests {
        group.bench_function(name, |b| b.iter(|| {
                let f = tmp.reopen().unwrap();
                function(f);
            })
        );
    }

    group.finish();
}
criterion_group!(benches, bench_simple);
criterion_main!(benches);
