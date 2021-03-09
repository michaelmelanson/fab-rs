# fab-rs [![Rust](https://github.com/michaelmelanson/fab-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/michaelmelanson/fab-rs/actions/workflows/rust.yml)
The fabulous, aspirationally Make-compatible, fabricator of things.

# Status
This is really early days. Here's the checklist of what's supported and what's not right now:

- [x] Parsing Makefiles
- [x] Executing commands in rules
- [x] Dependency resolution
- [x] Environment variables passed into commands
- [x] Basic special variable substitution (`$@`, `$<`)
- [ ] Don't rebuild unmodified targets
- [ ] Pattern rules
- [ ] Variable definitions
- [ ] Standard pattern rule library
- [ ] Parallel builds (via https://github.com/alexcrichton/jobserver-rs)
- [ ] ...

# Usage

Fab reads Makefiles and executes the rules inside.

```
cargo install fab
cd /path/to/code
fab
```
