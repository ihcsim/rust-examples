# mandelbrot

A Rust program to plot the
[Mendelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set)
. It explores the usage of crates to:

* Work with complex number arithmetic
* Write image to PNG files
* Perform multi-thread computation

```sh
cargo build --release

target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1.0,0.20
```
