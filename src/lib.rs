use std::fs::File;
use std::io::Read;

const BUF_SZ: usize = 5 * 1024 * 1024;

pub fn sync_benchmark(mut f: File) {
    use std::sync::mpsc::sync_channel;

    let mut md5 = md5::Context::new();
    let (tx, rx) = sync_channel(1);

    std::thread::spawn(move || {
        loop {
            let mut buf = vec![0u8; BUF_SZ];
            let bytes_read = f.read(&mut buf).unwrap();
            tx.send((buf, bytes_read)).unwrap();
            if bytes_read == 0 {
                break;
            }
        }
    });

    loop {
        let (buf, len) = rx.recv().unwrap();
        if len == 0 {
            break;
        }
        md5.consume(&buf[..len]);
    }
}

async fn async_std_benchmark_run(f: File) {
    use async_std::fs::File;
    use async_std::task;
    use async_std::io::ReadExt;
    use async_channel::bounded;

    let mut md5 = md5::Context::new();
    let mut f: File = f.into();
    let (tx, rx) = bounded(1);

    task::spawn(async move {
        loop {
            let mut buf = vec![0u8; BUF_SZ];
            let bytes_read = f.read(&mut buf).await.unwrap();
            tx.send((buf, bytes_read)).await.unwrap();
            if bytes_read == 0 {
                break;
            }
        }
    });

    loop {
        let (buf, len) = rx.recv().await.unwrap();
        if len == 0 {
            break;
        }
        // async-std no longer needs explicit call to spawn_blocking():
        // it detects blocking and automatically spawns a new executor
        // thread:
        // https://async.rs/blog/stop-worrying-about-blocking-the-new-async-std-runtime/
        md5.consume(&buf[..len]);
    }
}

pub fn async_std_benchmark(f: File) {
    use async_std::task;

    task::block_on(async_std_benchmark_run(f));
}

async fn tokio_benchmark_run(f: File) {
    use tokio::{
        fs::File,
        io::AsyncReadExt,
        sync::mpsc::channel,
    };

    let mut md5 = md5::Context::new();
    let mut f: File = f.into();
    let (tx, mut rx) = channel(1);

    tokio::spawn(async move {
        loop {
            let mut buf = vec![0u8; BUF_SZ];
            let bytes_read = f.read(&mut buf).await.unwrap();
            tx.send((buf, bytes_read)).await.unwrap();
            if bytes_read == 0 {
                break;
            }
        }
    });

    loop {
        let (buf, len) = rx.recv().await.unwrap();
        if len == 0 {
            break;
        }
        tokio::task::block_in_place(|| md5.consume(&buf[..len]));
    }
}

pub fn tokio_benchmark(f: File) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(tokio_benchmark_run(f));
}
