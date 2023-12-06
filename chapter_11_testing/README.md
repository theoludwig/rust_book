# 11. Testing

Tests are Rust functions that verify that the non-test code is functioning in the expected manner.

The bodies of test functions typically perform these 3 actions:

1. Set up any needed data or state.
1. Run the code you want to test.
1. Assert the results are what you expect.

Command to run tests: `cargo test`.

## Running Tests in Parallel or Consecutively

By default, tests are run in parallel using threads, so the order in which test functions are run is not guaranteed.

If you don't want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can use `cargo test -- --test-threads=1`.

## Showing Function Output

By default, if a test passes, Rust's test library captures anything printed to standard output. For example, if we call `println!` in a test and the test passes, we won't see the `println!` output in the terminal.

If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with `cargo test -- --show-output`.

## Running a Subset of Tests by Name

Sometimes, running a full test suite can take a long time. If you're working on code in a particular area, you might want to run only the tests pertaining to that code.

Example:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

We can run only the tests that have `add_two_and_two` in their names by running `cargo test add_two_and_two`.

Or, we can run all the tests that have `add` in their names by running `cargo test add`.

We can also run a particular test in a particular file by running `cargo test <filename>::<test_name>` (e.g. `cargo test tests::test_main_success`) or all tests in a particular file/folder by running `cargo test <filename>` (e.g. `cargo test tests`).

## Test Organization

The Rust community thinks about tests in terms of 2 main categories: unit tests and integration tests.

- **Unit tests** are small and more focused, testing one module in isolation at a time, and can test private interfaces.
- **Integration tests** are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

Rust's privacy rules do allow you to test private functions by importing them with `use super::*;` in the `tests` module (only possible in unit tests).

Integration tests are in the `tests` directory at the crate root.
