# BashRec

A superset of Bash that compiles down to plain, portable shell scripts.

Write safer, more modern shell scripts using familiar syntax — type
annotations, structured error handling, readable conditionals, and
functions with named parameters — then compile to a `.sh` file that runs
anywhere Bash does.

## Why?

Bash is everywhere (CI/CD, deployment scripts, automation) but it's easy
to get wrong: unquoted variables causing word-splitting bugs, unreadable
comparison operators (`-gt`, `-eq`), weak error handling, and manual
`$1`, `$2` argument mapping in functions.

BashRec doesn't replace Bash — it compiles *to* Bash. You keep using
Bash under the hood, but write it with a friendlier syntax.

## Features

- **Type annotations** — `let age: number = "25"` compiles to an
  assignment plus a runtime validation check
- **Structured error handling** — `try { } catch { }` blocks instead of
  manual exit code checking
- **Modern comparison operators** — `if age > 18 { }` instead of
  `[[ $age -gt 18 ]]`
- **Auto-quoting** — variables are automatically wrapped in quotes to
  avoid word-splitting bugs
- **Functions with named parameters** — no more manual `$1`, `$2` mapping
- **Arrays** — `let items: array = ["a", "b", "c"]`
- Full Bash passthrough — anything BashRec doesn't recognize is passed
  through unchanged

## Example

**Input (`script.brec`):**
```bash
let age: number = "25"

if age > 18 {
  echo "adult"
} else {
  echo "minor"
}

function greet(name) {
  echo "Hello, $name"
}
```

**Output (`script.sh`):**
```bash
age="25"
if ! [[ "$age" =~ ^-?[0-9]+$ ]]; then echo "TypeError: age must be a number" >&2; exit 1; fi

if [[ "$age" -gt "18" ]]; then
  echo "adult"
else
  echo "minor"
fi

greet() {
  local name="$1"
  echo "Hello, $name"
}
```

## Installation

Requires [Rust](https://rustup.rs) to build from source.

```bash
git clone https://github.com/mossveil-dev/bashrec.git
cd bashrec
cargo build --release
```

The compiled binary will be at `target/release/bashrec`.

## Usage

```bash
bashrec script.brec -o script.sh
```

If `-o` is omitted, the output file defaults to the input filename with
a `.sh` extension.

## Status

BashRec is an early-stage project. Core features work, but it hasn't
been extensively tested against real-world scripts yet. Contributions,
bug reports, and feedback are welcome.

## License

MIT — see [LICENSE](LICENSE) for details.