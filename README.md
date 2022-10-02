# Vigenère Cipher
> Based on Rust framework [MoonZoon](http://moonzoon.rs/)

Live demo: [moonzoon-vigenere.netlify.app](https://moonzoon-vigenere.netlify.app/)

![Demo](./demo.gif)

Inspiration: [Building a Real-Time Web Cipher with Rust, Sycamore and Trunk](https://rsdlt.github.io/posts/rust-sycamore-trunk-wasm-iterators-vigenere-cipher/)

---

Version without global variables: [branch no_globals](https://github.com/MoonZoon/vigenere-cipher/tree/no_globals)

---

How to Run:

1. Install the latest stable [Rust](https://www.rust-lang.org/tools/install). (Or upgrade with `rustup update stable`.)
1. Install the web assembly target `rustup target add wasm32-unknown-unknown`
1. `cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --locked`
1. `mzoon start --open`

Deploy to Netlify:

1. `mzoon build --release --frontend-dist netlify`
1. Drag & drop the `frontend_dist` directory to [Netlify](https://www.netlify.com/).
