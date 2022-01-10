# quickreplace

A search-and-replace tool that replaces some targeted strings in an input text
file, and write the result to another output text file. This tool uses the
`text-colorizer` crate to produce colored output.

```sh
cargo run "world" "Rust" test.txt test-modified.txt
```
