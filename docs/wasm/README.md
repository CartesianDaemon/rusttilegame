# Build wasm:

Build wasm:

```
rustup target add wasm32-unknown-unknown
cargo build -r --target wasm32-unknown-unknown
```

Release version is smaller but both release and debug seem to work.

# Make .html

Have tilegame.html and tilegame.wasm in the same directory.

tilegame.html copied from index.html https://github.com/not-fl3/macroquad
But currently working with `miniquad-samples/gl.js` as per recent article,
not with `mq_js_bundle.js` suggested by README.

TBD: Check errors in a developer console, were they more informative?
TBD: Check different versions of js bundle.

# Serve web pages

Serve web pages (don't just open .html in browser). E.g:

 cargo install basic-http-server
 basic-http-server .

Works just the same on windows, going to http://127.0.0.1:4000/. Somehow!

TBD: If it uses any assets, it may need to be served from crate root.

# Github pages

Configured to host from repo root. See `index.html`. Links to `wasm/tilegame.html`.

With that, can be played from web https://cartesiandaemon.github.io/rusttilegame/wasm/tilegame.html

And even from mobile web!

TBD: Although no mouse/touch controls that would work on phone yet.