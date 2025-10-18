<img width="980" height="496" alt="image" src="https://github.com/user-attachments/assets/84ee0f99-c8cb-425a-8b47-4379fc8879e0" />

## Overview



## Project Structure

### 📊 DashBench
- benchmark read/write operations for different rust types

### Mollusk
- benchmarking functionality
- write mollusk test for magicblock example, see how it interacts with SVM api
- https://github.com/magicblock-labs/ephemeral-rollups-sdk/blob/main/rust/sdk/src/cpi.rs#L41


- Test different parts of SVM
- Add/Modify sysvar
- https://github.com/anza-xyz/mollusk/blob/main/harness/src/sysvar.rs
- https://github.com/anza-xyz/mollusk/blob/main/harness/src/lib.rs#L685
- https://github.com/anza-xyz/mollusk/blob/main/harness/src/lib.rs#L724
- https://github.com/anza-xyz/mollusk/blob/main/harness/src/lib.rs#L818

### 🦀 Native Rust Vault






## Getting Started



### Building Projects

#### Anchor Marketplace
```bash
cd anchor-marketplace
anchor build
```

#### Native Rust Vault
```bash
cd native-rust-vault
cargo build
```

#### DashBench
```bash
cd dashbench
cargo bench
```

## Testing

### Anchor Tests
```bash
cd anchor-marketplace
anchor test
```

### Rust Tests
```bash
# Run tests for specific project
cd [project-directory]
cargo test
```




