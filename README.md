# checkers

To build this project, you should make sure to have installed [Rust](https://www.rust-lang.org/tools/install) and [npm](https://nodejs.org/en/download/).

## Building the backend

```sh
cargo build
```

The output binary will be in `target/debug/checkers`.

## Building the frontend

```sh
cd www
npm run build
```

## Running the server

```sh
./target/debug/checkers
```

This will print the adresses that the server is listening on. Connect to any of them. Once there are two players connected,
a new game between them will be started.

## Playing the game

Click on a piece, then click on the field you want to move to. There is currently no indication or explanation for why
a move isn't allowed, so if nothing happens, it probably means that you tried to do something illegal.
