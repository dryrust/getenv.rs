# Getenv.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.81%2B-blue)](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0/)
[![Package](https://img.shields.io/crates/v/getenv)](https://crates.io/crates/getenv)
[![Documentation](https://docs.rs/getenv/badge.svg)](https://docs.rs/getenv/)

🚧 _We are building in public. This is presently under heavy construction._

## ✨ Features

- Implements a comprehensive set of environment variable getters for
  various platforms, programming languages, and tools.
- Provides a wide range of varsets (variable sets), including for:
  AWS, Cargo, Conda, CUDA, Docker, Git, Go, Homebrew, Java,
  Locale, macOS, NEAR, Node.js, OpenSSL, POSIX, Proxy,
  Python, Ruby, Rust, SSH, Windows, and XDG.
- Protects sensitive environment variables (such as API keys) by preventing
  their accidental copying, logging, or other exposure, and securely wiping
  them from memory when dropped.
- Treats empty environment variables the same as missing ones.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.81+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add getenv
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
getenv = "0.1"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
getenv = { version = "0.1", default-features = false, features = ["posix"] }
```

## 👉 Examples

### Importing the Library

```rust
use getenv::*;
```

### Reading environment variables

```rust
use getenv::aws::*;

println!("AWS_ACCESS_KEY_ID={:?}", aws_access_key_id());
println!("AWS_SECRET_ACCESS_KEY={:?}", aws_secret_access_key());
```

## 📚 Reference

Varset | Feature | Reference
:----- | :------ | :--------
aws | `aws` | [`use getenv::aws::*;`](https://docs.rs/getenv/latest/getenv/varsets/aws/index.html)
cargo | `cargo` | [`use getenv::cargo::*;`](https://docs.rs/getenv/latest/getenv/varsets/cargo/index.html)
conda | `conda` | [`use getenv::conda::*;`](https://docs.rs/getenv/latest/getenv/varsets/conda/index.html)
cuda | `cuda` | [`use getenv::cuda::*;`](https://docs.rs/getenv/latest/getenv/varsets/cuda/index.html)
docker | `docker` | [`use getenv::docker::*;`](https://docs.rs/getenv/latest/getenv/varsets/docker/index.html)
git | `git` | [`use getenv::git::*;`](https://docs.rs/getenv/latest/getenv/varsets/git/index.html)
go | `go` | [`use getenv::go::*;`](https://docs.rs/getenv/latest/getenv/varsets/go/index.html)
homebrew | `homebrew` | [`use getenv::homebrew::*;`](https://docs.rs/getenv/latest/getenv/varsets/homebrew/index.html)
java | `java` | [`use getenv::java::*;`](https://docs.rs/getenv/latest/getenv/varsets/java/index.html)
locale | `locale` | [`use getenv::locale::*;`](https://docs.rs/getenv/latest/getenv/varsets/locale/index.html)
macos | `macos` | [`use getenv::macos::*;`](https://docs.rs/getenv/latest/getenv/varsets/macos/index.html)
near | `near` | [`use getenv::near::*;`](https://docs.rs/getenv/latest/getenv/varsets/near/index.html)
node | `node` | [`use getenv::node::*;`](https://docs.rs/getenv/latest/getenv/varsets/node/index.html)
openssl | `openssl` | [`use getenv::openssl::*;`](https://docs.rs/getenv/latest/getenv/varsets/openssl/index.html)
posix | `posix` | [`use getenv::posix::*;`](https://docs.rs/getenv/latest/getenv/varsets/posix/index.html)
proxy | `proxy` | [`use getenv::proxy::*;`](https://docs.rs/getenv/latest/getenv/varsets/proxy/index.html)
python | `python` | [`use getenv::python::*;`](https://docs.rs/getenv/latest/getenv/varsets/python/index.html)
ruby | `ruby` | [`use getenv::ruby::*;`](https://docs.rs/getenv/latest/getenv/varsets/ruby/index.html)
rust | `rust` | [`use getenv::rust::*;`](https://docs.rs/getenv/latest/getenv/varsets/rust/index.html)
ssh | `ssh` | [`use getenv::ssh::*;`](https://docs.rs/getenv/latest/getenv/varsets/ssh/index.html)
windows | `windows` | [`use getenv::windows::*;`](https://docs.rs/getenv/latest/getenv/varsets/windows/index.html)
xdg | `xdg` | [`use getenv::xdg::*;`](https://docs.rs/getenv/latest/getenv/varsets/xdg/index.html)

### Integrations

Crate (Feature) | Version | Usage | Summary
:--- | :--- | :--- | :---
[miette] &nbsp;<sub>(`"miette"`)</sub> | 7.5 | [![miette](https://docs.rs/miette/badge.svg)](https://docs.rs/miette/) | Enhances `dogma::{IriError, UriError}`
[serde] &nbsp;<sub>(`"serde"`)</sub> | 1 | [![serde](https://docs.rs/serde/badge.svg)](https://docs.rs/serde/) | Derives `serde::{Serialize, Deserialize}`
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/getenv.rs.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/getenv.rs&text=Getenv.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/getenv.rs&title=Getenv.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/getenv.rs&t=Getenv.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/getenv.rs)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/getenv.rs)

[feature flags]: https://github.com/dryrust/getenv.rs/blob/master/lib/getenv/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[miette]: https://crates.io/crates/miette
[serde]: https://crates.io/crates/serde
