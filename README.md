# java_detect

A Rust library that detects Java (JDK/JRE) installations on the local system across Windows, Linux, and macOS.

## Overview

`java_detect` scans platform-specific directories to find Java executables, parses their version numbers, and returns a list of available installations.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
java_detect = { git = "https://github.com/syncraft/java_detect" }
```

For serialization support:

```toml
[dependencies]
java_detect = { git = "https://github.com/syncraft/java_detect", features = ["serde"] }
```

## Usage

```rust
let installations = java_detect::detect();
for java in installations {
    println!("Java {}", java);
}

if let Some(java17) = java_detect::get_java(17) {
    println!("Found Java 17 at {}", java17.path.display());
}
```

## API

### `detect() -> Vec<JavaInstallation>`

Scans common Java installation directories and returns all detected installations. Finds up to 5 levels deep.

### `get_java(version: u8) -> Option<JavaInstallation>`

Returns the first Java installation matching the given major version (e.g., 8, 17, 21).

### `JavaInstallation`

| Field    | Type     | Description                          |
|----------|----------|--------------------------------------|
| `path`   | `PathBuf`| Path to the Java executable          |
| `version`| `u8`     | Java major version (e.g., 8, 17, 21)|

Implements `Debug`, `Clone`, `Display`. Optionally derives `Serialize`/`Deserialize` with the `serde` feature.

## Search Paths

| Platform | Directories scanned                                      |
|----------|----------------------------------------------------------|
| Linux    | `/usr/lib/jvm`, `/usr/java`, `/opt`, `/snap`, etc.      |
| Windows  | `C:\Program Files`, `D:\Program Files` and `(x86)` variants |
| macOS    | `/Library/Java/JavaVirtualMachines`, Homebrew paths     |

## License

MIT
