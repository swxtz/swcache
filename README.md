# Rust Cache Library

![Rust](https://img.shields.io/badge/rust-1.0+-blue.svg) [![Crates.io Version](https://img.shields.io/crates/v/swcache?style=flat-square&color=blue)](https://crates.io/crates/swcache)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/swcache?style=flat-square&color=orange)](https://crates.io/crates/swcache)
![Crates.io License](https://img.shields.io/crates/l/swcache) ![Crates.io Size](https://img.shields.io/crates/size/swcache)



## Overview

The Rust Cache Library is a lightweight and high-performance caching solution designed for applications that require fast data retrieval with minimal overhead. It is perfect for developers who need an efficient caching mechanism without the bloat.

## Features

- **Lightweight**: Designed to minimize memory usage and increase performance.
- **High Performance**: Optimized for speed, allowing for quick read and write operations.
- **Thread-safe**: Built with concurrency in mind, ensuring safe access in multi-threaded environments.
- **Customizable eviction policies**: Supports various cache eviction strategies.
- **Easy to use**: Simple API for seamless integration into your projects.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
swcache = "0.1"
