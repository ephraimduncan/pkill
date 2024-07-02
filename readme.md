# `pkill`

`pkill` CLI is a simple, efficient command-line tool written in Rust that allows you to quickly terminate processes running on specified ports on Linux and macOS systems.

## Installation

```
cargo install killp
```

## Usage

To use Port Killer CLI, simply run the executable followed by the port numbers you want to kill processes on:

```
pkill <PORT1> <PORT2> <PORT3> ...
```

## Examples

1. Kill a process on a single port:

   ```
   pkill 8080
   ```

2. Kill processes on multiple ports:

   ```
   pkill 3000 3001 3002
   ```

## License

This project is licensed under the MIT License
