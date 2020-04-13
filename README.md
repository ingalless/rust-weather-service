# Rust Weather Service
This is a fake weather service built in rust. It could be useful if you wanted weather to be the same across all local clients of a game.

This **does not** simulate real weather. That's for big computers and bigger brains than mine.

## Installation
Rust + VS Code isn't great on Windows yet, so I've setup a devcontainer. [You should check them out](https://code.visualstudio.com/docs/remote/containers).

As this uses a devcontainer, all you need are:
- VS Code
- Visual Studio Code Remote - Containers extension

Alternatively, you could use a local setup with the below:
- [rustup](https://www.rust-lang.org/tools/install)

## Running
`cargo run` will run the webserver on `localhost:8088`