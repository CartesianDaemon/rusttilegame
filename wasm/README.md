Build wasm:

 rustup target add wasm32-unknown-unknown
 cargo build -r --target wasm32-unknown-unknown

Release version is smaller but both release and debug seem to work.

Have tilegame.html and tilegame.wasm in the same directory.

tilegame.html copied from index.html https://github.com/not-fl3/macroquad
But currently working with miniquad-samples/gl.js as per recent article,
not with mq_js_bundle.js suggested by README.

TBD: Check errors in a developer console, were they more informative?
TBD: Check different versions of js bundle.

Server web pages (don't just open .html in browser). E.g:

 cargo install basic-http-server
 basic-http-server .

TBD: If any assets, may need to be in crate root.
