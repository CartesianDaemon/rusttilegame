Build wasm:

 rustup target add wasm32-unknown-unknown
 cargo build -r --target wasm32-unknown-unknown

Release version is smaller but both release and debug seem to work.

Have tilegame.html and tilegame.wasm in the same directory.

tilegame.html copied from index.html https://github.com/not-fl3/macroquad
But gl.js not mq_js_bundle.js from macroquad github pages.

TBD: Check if history of .js is in macroquad crate.
TBD: Check if macroquad docs are out of date.
TBD: Check errors in a developer console, were they more informative?

Server web pages (don't just open .html in browser). E.g:

 cargo install basic-http-server
 basic-http-server .

TBD: If any assets, may need to be in crate root.
