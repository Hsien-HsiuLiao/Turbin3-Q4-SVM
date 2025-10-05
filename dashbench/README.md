# dashbench
Dashmap benching, performance comparison between `Arc<Mutex<>> / RwLock<>` and Dashmap, 
using rayon threads and the [criterion](https://github.com/bheisler/criterion.rs) benchmarking framework.

## Usage

Run all benchmarks:
```bash
cargo bench
```

Run specific benchmark groups:
```bash
cargo bench hashmap    # HashMap benchmarks
cargo bench dashmap    # DashMap benchmarks  
cargo bench rayon      # Threaded benchmarks
```

Results are available in `target/criterion/report/index.html`
