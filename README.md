# key-counter

A small for-fun program that counts how many times each key and mouse button is pressed.
Runs silently in the background without a window.

It stores only the total number of keys pressed and which one of them into your desktop as .txt file. 
Never stores or sends any text or whatever possible malicious activities one might assume this does. 


## Build & run

Requires [Rust](https://www.rust-lang.org/).

```
cargo build --release
```

Then run `target\release\key-counter.exe`.

Stop with Ctrl+Q.

