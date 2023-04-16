# Simple multithreading in Rust 🦀

Basic ThreadPool implementation in Rust following the guide from The Rust Book with added error handling!

# Features

- 100% Rust! 🦀🦀🦀
- Can easily execute any piece of code concurrently
- Uses safe Rust code (Memory safe, Hacker safe)
- Workers automatically wait for new jobs and total jobs are stored in a queue

# Basic usage

Remember to not create too many workers or your CPU won't be able to keep up, `4` is a good place to start.
```rs
use multithreading::ThreadPool;

fn main() {
    let num_workers = 4;
    let pool = ThreadPool::new(num_workers);

    for _ in 0..10 {
        pool.execute(|| {
            // your very complicated job
        });
    }
}
```

# Error handling

Designed to not panic when encountering errors.
The `execute` method returns a `Result<(), String>` that you can check for errors like such:

```rs
fn main() {
    let num_workers = 4;
    let pool = ThreadPool::new(num_workers);

    for _ in 0..10 {
        pool.execute(|| {
            // your very complicated job
        }).unwrap_or_else(|err| {
            // handle the error here
        });
    }
}
