### Cargo

替换cargo源

```
vim ~/.cargo/config

[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'tuna'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

https://cargo.budshome.com/index.html

如果cargo go 发生waiting for file lock on package cache

rm -rf ~/.cargo/.package-cache