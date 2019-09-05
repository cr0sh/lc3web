# lc3web: [lc3-rs](https://github.com/cr0sh/lc3-rs)\/[lc3asm](https://github.com/cr0sh/lc3asm) on web browser
This is a demo program about running LC-3 tools on web browser, using [WebAssembly](https://developer.mozilla.org/docs/WebAssembly) technology.

Technologies/Software used:
 - [Rust](http://www.rust-lang.org) (>90% of codes were written + some HTML/CSS)
 - [WebAssembly](https://developer.mozilla.org/docs/WebAssembly) (To run on browser)
 - [Yew](https://github.com/yewstack/yew) (frontend)
 
## Requirements
- Rust/Cargo
- [cargo-web](https://github.com/koute/cargo-web)
  - Install with `cargo install cargo-web`

Other dependencies will be downloaded during building.
 
## Running
 - `git clone` this repository
 - `cargo web run`
 - Go to http://localhost:8000