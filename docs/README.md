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
TODO: ^^ That's out of date.

# Serve web pages

Serve web pages (don't just open .html in browser). E.g:

 cargo install basic-http-server
 basic-http-server .

Works just the same on windows, going to http://127.0.0.1:4000/. Somehow!

TBD: If it uses any assets, it may need to be served from crate root.

# Misc problems

* Make sure that "img/" assets directory is in the directory we're serving http from.
When serving locally I used a symlink.
* Don't block threads. Wasm uses only one thread (?) and blocks completely (?)
* Macroquad already does some magic so assets are loaded using the same path they usually would, ie. from source root like "imgs/ferris.png". I am here adding a bit extra so everything can be served from docs dir like github pages expects.
* TBD: Any other false starts from my notes?

TBD: Check if symlink works in github pages.
TBD: If I'm compiling an existing levset do I want to compile the assets into the exe?

# To make a new release

Should be able to build release build, then run `scripts/release.sh` to copy release and assets to published folder.
Need to check workflow.

Should be able to then run `basic-http-server docs/` and check it works ok.

TBD: Have release.sh script do cargo build as well?

# Github pages

Configured to host from repo root. See `index.html`. Links to `wasm/tilegame.html`.

With that, can be played from web https://cartesiandaemon.github.io/rusttilegame/wasm/tilegame.html

And even from mobile web!
