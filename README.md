# Minimalistic OpenGL Rust Library
![Rust Workflow](https://github.com/steschu77/minigl-rs/actions/workflows/rust.yml/badge.svg)

## Project Goals

MiniGL-rs is a minimalistic and low-level OpenGL library that provides OpenGL bindings and enables applications to use OpenGL without the need to link against a large library like GLFW or SDL.

This comes with the drawback of limited functionality and features. If you require a more feature-rich alternative, you might want to consider the following options:

* Use [glutin](https://crates.io/crates/glutin), a more comprehensive library.
* Contribute to this project by opening a pull request (PR) to add the missing functionality.
* Fork this repository and implement the features you need.

## Features

* Windows support
  * Window creation and access to the OpenGL context and OpenGL functions
  * Depends only on [windows-rs](https://crates.io/crates/windows)
* Linux support
  * Window creation and access to the OpenGL context and OpenGL functions
  * Depends only on [x11](https://crates.io/crates/x11)
* MacOS support tbd.

## Usage

Check out the [examples](examples/) directory for some example code which can be run using `cargo run --example triangle`.

## License

This project is licensed under the MIT license.
