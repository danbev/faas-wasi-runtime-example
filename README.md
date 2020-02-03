# FaaS WASI Runtime Image example
The idea here is to enable WebAssembly modules, possibly using the WebAssembly
System Interface (WASI), to be executed as OpenShift Cloud Function. An end
user would have a WASM module they would like to expose as a function. This
WASM module could either be a module written and bundled with the users
project, or could be a WASM module in the Web Assembly Package Manager
([wapm](https://wapm.io)) or in any other package manager, for example Node.js
Package Manager ([npm](https://www.npmjs.com/)).

## Building

To build the image, run the following command:
```console
$ cargo build
```

### WASM test module
The .wasm module used is located in `module/add.wasm`, and looks like this:
```console
$ wasm2wat add.wasm
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    get_local 0
    get_local 1
    i32.add)
  (export "add" (func 0)))
```

## Running locally
```console
$ FUNCTION_NAME=add PORT=8080 MODULE_PATH=./module/add.wasm cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/faas-wasm-runtime-image`
WASI Runtime started. Port: 8080, Module path: ./module/add.wasm
```
And then from a second terminal you can call the service:
```console
$ curl 'http://localhost:8080/data?nr1=10&nr2=23'
module: ./module/add.wasm, function: add, returned 33: i32
```
