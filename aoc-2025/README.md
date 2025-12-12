# Advent of Code 2025

Solutions for Advent of Code 2025 in Rust.

## Dependencies

### Z3 Solver

Day 10 Part 2 uses the Z3 SMT solver for optimization problems. Z3 must be installed on your system.

#### Installation

**macOS (Homebrew):**
```bash
brew install z3
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install libz3-dev
```

**Linux (Arch):**
```bash
sudo pacman -S z3
```

#### Configuration

The Z3 library paths are configured in the workspace root `.cargo/config.toml` (at `../..cargo/config.toml` relative to this directory):

- **Apple Silicon (M1/M2/M3):** `/opt/homebrew/opt/z3` (default in config)
- **Intel Mac:** `/usr/local/opt/z3`
- **Linux (x86_64):** `/usr/lib` and `/usr/lib/x86_64-linux-gnu`
- **Linux (ARM64):** `/usr/lib` and `/usr/lib/aarch64-linux-gnu`

**For non-Apple Silicon systems**, override the Z3 header location:

```bash
# Intel Mac
export Z3_SYS_Z3_HEADER=/usr/local/opt/z3/include/z3.h

# Linux
export Z3_SYS_Z3_HEADER=/usr/include/z3.h
```

Or add it to your shell profile (`.bashrc`, `.zshrc`, etc.).

#### GitHub Actions / CI

The `.github/workflows/test.yml` workflow automatically:
1. Installs Z3 on Ubuntu runners via `apt-get install libz3-dev`
2. Sets `Z3_SYS_Z3_HEADER=/usr/include/z3.h` for Linux

This ensures tests pass in CI environments without manual configuration.

## Running Solutions

```bash
# Run tests for all days
cargo test

# Run tests for a specific day
cargo test day10::tests --lib

# Run the solution
cargo run
```

## Building

```bash
cargo build --release
```


## Visualization

An interactive visualization for Day 12's backtracking solver is included in `visualization.html`.

### How to use

1.  Open `visualization.html` in your web browser:
    ```bash
    open visualization.html
    ```
2.  Controls:
    -   **Play**: Start the solver automatically.
    -   **Step**: Advance one step at a time.
    -   **Speed**: Adjust playback speed.
    -   **Reset**: Restart the simulation.
