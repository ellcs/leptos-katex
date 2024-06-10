# Leptos KaTeX demo

This project is a simple experiment on how to render tex formulas on a website without using a single line of javascript. I used leptos for client site rendering and a KaTeX crate. These two projects combined offer a simple, but elegant way to render formulas.

## Setup

Assuming you have `rustup` and the `nightly` toolchain installed, you can just clone the repository and run it with `trunk` (`cargo install trunk`):

  - `git clone $repo`
  - `trunk serve --port 1337 --release --open`

You can omit the `--release` flag, but I like to see smaller artifacts and the project is not that big that you have long compile times.
