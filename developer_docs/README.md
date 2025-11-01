# Running linux build locally

Should just work:

```
cargo run
```

Sadly does not work in WSL yet, even though other graphical linux applications do.

# Running windows build locally

Sadly build from WSL does not run under Windows successfully yet.

Not yet tried building natively on Windows.

# Running wasm locally

## Build wasm:

If not already done, install wasm target:

```
rustup target add wasm32-unknown-unknown
```

Build the wasm:

```
cargo build --target wasm32-unknown-unknown # Build debug version
cargo build -r --target wasm32-unknown-unknown # Build release version
```

Release version is smaller but otherwise both work.

## .html pages

The `test_wasm/` directory contains a local version of the html used to
serve the game. It includes some magic javascript from the macroquad
homepage to get the javascript runtime for sdl working.

It also symlinks the imgs directory so the wasm can see the imgs/ directory
relative to its current directory.

## Serve web page locally

Most browsers won't serve wasm from .html files from the local file system. It
ought to work to override that setting. But it is typical to test the wasm in
a website using a minimal web server. E.g:

```
 # Install very simple web server
 cargo install basic-http-server

# Serve the web pages from the test_wasm directory.
 basic-http-server test_wasm/
```

You can then see the debug build in a web browser by going to:
http://127.0.0.1:4000/pushing_debug.html. You can see the release build by going to:
http://127.0.0.1:4000/pushing_release.html.

This works even when running the web browser in WSL and viewing the webpage from
a browser in Windows.

### Notes

index.html was copied from the example on the macroquad homepage.

Macroquad suggested two different versions of the javascript bundle. `mq_js_bundle.js`
`miniquad-samples/gl.js`. One was from the README on the homepage, the other from a
recent article. I originally thought this was the cause of problems I had, but those
turned out to be due to me using executors::block_on() to call texture loading
functions.

### Misc problems

* Don't block threads. Wasm uses only one thread (?) and blocks completely (?)
* Macroquad already does some magic so assets are loaded from the repository [Is that true or is it just current dir of website?]. I am ensure that the imgs/ directory is available in both the local test site and the release site. It seems GitHub pages servers everything in docs/ and nothing else.

Maybe: Check if symlink works in github pages.
Maybe: Can compile the assets into the exe?

# Making a wasm release

## Building release

Run `build_scripts/release.sh` to build a wasm release from the latest source, and copy the wasm
output and image assets to published folder.

You can check it works the same way as the local test site by running:

`basic-http-server docs/`

## Serving from Github pages

GitHub automatically publishes the 'docs/' directory as a GitHub Pages site whenever the repository
is pushed.

It can be played from web https://cartesiandaemon.github.io/rusttilegame/wasm/tilegame.html

And even from mobile web!

Maybe: Update top-level README with more up-to-date readme.
Maybe: Update test_wasm to have versions for release/debug for all three exes?
Maybe: Rename test_wasm to "latest_wasm_build".
Maybe: Make publishing a release more like copying some/all of test_wasm/ wholesale to docs/
Maybe Maybe: make it easier to build, make sure web server is running, and open html in browser..?
