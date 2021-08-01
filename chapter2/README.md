### Cargo

cargo是rust包管理工具

cargo version

cargo init 初始化包结构

cargo new project_name

```
chapter2
├── Cargo.toml
└── src
    └── main.rs
```

Cargo.toml
```
[package]
name = "chapter2"      // 项目名
version = "0.1.0"      // 版本号
edition = "2018"       // rust版本

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]         // 依赖项

```

rust中代码的包称做crate


### 构建cargo项目

cargo build [--release]
cargo run
cargo check
cargo update