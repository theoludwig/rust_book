# An I/O Project: Building a Command Line Program: `minigrep`

Command line tool that interacts with file and command line input/output to practice some of the Rust concepts we've learned so far.

Rust's speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools, so for our project, we'll make our own version of the classic command line search tool `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).

`grep` takes as its arguments a file path and a string. Then it reads the file, finds lines in that file that contain the string argument, and prints only those lines (containing the string).

We'll call our project `minigrep` to distinguish it from the `grep` tool that you might already have on your system.

The first task is to make minigrep accept its two command line arguments: the file path and a string to search for.

## Example usage

```sh
cargo test
```

### Basic search

It searches for the string `"the"` in the file `poem.txt` and prints all the lines that included the string to the terminal:

```sh
cargo run -- "the" "poem.txt"
# or
IGNORE_CASE="false" cargo run -- "the" "poem.txt"
```

By default `IGNORE_CASE` is `false` and the search is **case-sensitive**.

similar to:

```sh
grep "the" "poem.txt"
```

### Case-Insensitive search

```sh
IGNORE_CASE="true" cargo run -- "the" "poem.txt"
```

similar to:

```sh
grep -i "the" "poem.txt"
```
