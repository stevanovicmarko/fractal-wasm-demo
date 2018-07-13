# WASM based fractal generator

A Simple fractal generator that uses [rust programming language](https://www.rust-lang.org) and [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for web assembly code generation.

# Installation and running

- install webpack server and typescript via yarn or npm using: `yarn install` or `npm install` command
- (re)compile ts to js by running yarn or npm: `yarn run compile:ts`
- run the webpack server with yarn or npm: `yarn run serve`
- navigate to http://localhost:8080

Optionally you can modify and recompile wasm(rust) code if have rustup and cargo installed by following [this](https://rustwasm.github.io/wasm-bindgen/basic-usage.html) guide.
You can run `yarn run compile:wasm` after wasm-bindgen setup.
