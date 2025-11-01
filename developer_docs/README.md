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

It also symlinks the imgs directory so the images are at a sensible path
relative to the directory the html is served from.

## Serve web page locally

Install a minimal web server. E.g:

```
 cargo install basic-http-server
```

Serve the web pages from the test_wasm directory. E.g:

```
 basic-http-server test_wasm/
```

You can see the debug build in a web browser by going to: http://127.0.0.1:4000/
or http://127.0.0.1:4000/index.html.

You can see the release build by going to: http://127.0.0.1:4000/index_release.html.

This works even when running the web browser in WSL and viewing the webpage from
a browser in Windows!

### Notes

It doesn't work if you just open the html in a browser directly from the file
system, rather than serving if through a web server.

index.html was copied from the example on the macroquad homepage.

Macroquad suggested two different versions of the javascript bundle. `mq_js_bundle.js`
`miniquad-samples/gl.js`. One was from the README on the homepage, the other from a
recent article. I originally thought this was the cause of problems I had, but those
turned out to be due to me using executors::block_on() to call texture loading
functions.

I still need to check if the image assets need to be served from crate root or from
this directory. [ETA: For the demo version, not the test version?]

### Misc problems [Triage this]

* Make sure that "img/" assets directory is in the directory we're serving http from.
When serving locally I used a symlink.
* Don't block threads. Wasm uses only one thread (?) and blocks completely (?)
* Macroquad already does some magic so assets are loaded using the same path they usually would, ie. from source root like "imgs/ferris.png". I am here adding a bit extra so everything can be served from docs dir like github pages expects.
* TBD: Any other false starts from my notes?

TBD: Check if symlink works in github pages.
TBD: If I'm compiling an existing levset do I want to compile the assets into the exe?

# Making a wasm release

## Building release

Should be able to build release build, then run `build_scripts/release.sh` to copy release and assets to published folder.

Should be able to then run `basic-http-server docs/` and check it works ok.

## Serving from Github pages

The github repository is configured to serve the html from the 'docs/' directory.

I can't remember if github still needs it to be compiled, or if it automatically serves the
current version from the source tree.

Configured to host from repo root. See `index.html`. Links to `wasm/tilegame.html`. [Out of date?]

With that, can be played from web https://cartesiandaemon.github.io/rusttilegame/wasm/tilegame.html

And even from mobile web!
