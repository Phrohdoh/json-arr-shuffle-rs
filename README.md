# `json-arr-shuffle`

A command-line utility which takes a JSON array in via stdin (a pipe), shuffles
the contents, and prints the result to stdout with a trailing `\n` character.

## License

See the [LICENSE] file.

## Installation

Binaries for Windows, GNU/Linux, and macOS will be available [from GitHub] once
[CI/CD has been set up].

In the meantime you will need to build this tool from source.

## Building from source

Assuming you have:

- [the Rust tools] installed and are able to run them successfully
- cloned this repository to your machine
- `cd`-ed into the root of this repository

Run the following command (excluding the `$`):

```shell
$ cargo build --release
```

If it builds successfully you will have an executable at
`target/release/json-arr-shuffle` that you will need to put in one of the
directories on your [`$PATH`].

## Usage

```shell
$ echo "[5, 6, 7]" | json-arr-shuffle
[5,7,6]
```

```shell
$ echo '["hello", 6, true]' | json-arr-shuffle
[6,"hello",true]
```

## Development

Development is done on [GitHub].

## Support the project financially

Please consider financially supporting the project if it has made your life
easier, saved you time, etc., or as a way to say "thanks!" by [becoming a Patron]
of the maintainer.

[LICENSE]: ./LICENSE
[GitHub]: https://github.com/Phrohdoh/json-arr-shuffle-rs
[CI/CD has been set up]: https://github.com/Phrohdoh/json-arr-shuffle-rs/issues/1
[from GitHub]: https://github.com/Phrohdoh/json-arr-shuffle-rs/releases
[the Rust tools]: https://rustup.rs/
[`$PATH`]: https://en.wikipedia.org/wiki/PATH_(variable)#Unix_and_Unix-like
[becoming a Patron]: https://www.patreon.com/Phrohdoh
