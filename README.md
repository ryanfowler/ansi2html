# ansi2html

Convert text with ANSI escape sequences to HTML

## Install

Install `ansi2html` by compiling with `cargo`:

```sh
git clone git@github.com:ryanfowler/ansi2html.git

cd ansi2html

cargo install --force --locked --path .
```

## Usage

`ansi2html` reads text with ANSI escape sequences from stdin, and writes the
equivalent HTML to stdout:

```sh
cat file-with-ansi.txt | ansi2html
```

#### License

MIT

