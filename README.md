# 威 wui

A mobile-first UI library for web assembly. The goal is to support both web and native mobile with a standard set of components similar to React Native and other hybrid UI technology.

This project uses the [`view!` macro](https://github.com/richardanaya/view)

## Hello World
```toml
[dependencies]
wui = "0.0.1"
```

```rust
use wui::*;

#[no_mangle]
pub fn main() -> () {
    globals::get::<Wui>().lock();
    wui.render(view!{
        Text("Hello World!")
    });
}
```

### Web
```html
<script src="https://cdn.jsdelivr.net/gh/richardanaya/js_ffi/js_ffi.js"></script>
<script>js_ffi.run("helloworld.wasm");</script>
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in wui by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
