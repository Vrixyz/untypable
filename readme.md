# Rustlang unexported type leak

follow up to [discussion on X](https://x.com/Vrixyz/status/1831350042256613568).

![rustdoc screenshot showing unlinkable type `ShouldNotLeak` as a return type for function `returns_leaked_type`, this type is not highlighted.](rustdoc_unknown_type.png)

## Usage

run `cargo doc --open`, and try to find documentation for the return type of public function `returns_leaked_type()`.

Also, run the example `cargo run --example user`.