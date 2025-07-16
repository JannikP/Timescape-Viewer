# Timescape-Viewer - Test Examples

This folder contains example files used in unit tests, benchmarks and demonstrations.
Reference files as such in code:

```Rust
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn read_foo_txt() {
        let file: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "assets",
            "examples",
            "foo.txt",
        ].iter().collect();
        // As proposed here: https://stackoverflow.com/a/61107861
        if file.exists() {
            // Write the tests in a way that they also pass when the assets are
            // missing. This allows Crater to test your crate successfully.

            // TODO: Put your actual test here.
        }
    }
}
```
