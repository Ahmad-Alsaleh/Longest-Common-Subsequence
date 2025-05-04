# Longest Common Subsequence (LCS)

## Overview

This project contains two implementations of the Longest Common Subsequence (LCS) algorithm:

1. Dynamic Programming approach
2. Brute Force approach

Both algorithms are implemented in `Rust` and `Python`.

Additionally, a Windows `.exe` executable is included for running a DP table visualization.

## File Structure

### Python

- `python/lcs_dynamic.py`: Contains an efficient LCS implementation using dynamic programming.
- `python/lcs_bruteforce.py`: Contains a brute force implementation for LCS using recursive generation of all subsequences.
- `python/generate_test_strings.py`: A simple script that generates the test cases.

### Rust

- `rust/src/dynamic_programming.rs`: Contains an efficient LCS implementation using dynamic programming.
- `rust/src/brute_force.rs`: Contains a brute force implementation for LCS using recursive generation of all subsequences.
- `rust/src/main.rs`: Contains a script that test both algorithms.

### Other Files

- `gui/dynamic_lcs_ui.exe`: Executable version of the DP-based LCS tool. No Python installation needed to run.
- `test_strings.txt`: Text file containing all random strings used in evaluation, labeled and categorized by algorithm.

## How to Run

```bash
# Python
cd python/ && python lcs_dynamic.py

# Rust
cd rust/ && cargo run # make sure Rust is installed on your system, see `Dependencies` section below for details
```

### Executable (.exe)

- Located in: `gui/dynamic_lcs_ui.exe`.
- To run, double-click the file — it will open a GUI or terminal-based version.

## Dependencies

- Python 3.9+ [download here](https://www.python.org/downloads/)
- Rust [download here](https://www.rust-lang.org/tools/install)

## Notes

- The Brute Force method is only run for small input sizes (e.g., ≤ 20 characters) due to exponential time complexity.
- The DP method can handle strings up to 1000+ characters efficiently.
