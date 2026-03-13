# `py-a2lfile`

[![CI](https://github.com/niradynamics/py-a2lfile/actions/workflows/ci.yml/badge.svg)](https://github.com/niradynamics/py-a2lfile/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue)](#license)

`py-a2lfile` is a Python wrapper around the Rust crate [`a2lfile`](https://crates.io/crates/a2lfile). It provides a small, read-only API for loading and inspecting A2L files from Python while relying on the upstream Rust implementation for parsing performance and stability.

## Current Capabilities

- Parse A2L content from a file path with `a2lfile.load(...)`
- Parse A2L content directly from a string with `a2lfile.load_from_string(...)`
- Inspect modules and measurements through a small, read-only Python API
- Access conversion metadata including `COMPU_METHOD`, `COMPU_TAB`, `COMPU_VTAB`, and `COMPU_VTAB_RANGE`
- Resolve references from measurements to conversion tables and engineering units
- Inspect module-level metadata such as `MOD_COMMON`, `MOD_PAR`, and unit definitions
- Traverse generic `IF_DATA`, including tagged structures used for XCP-oriented extraction
- Access detailed measurement metadata such as annotations, bit operations, addresses, refresh information, symbol links, and virtual channels
- Stable and fast parsing backed by the Rust `a2lfile` library
- Typed distribution with `_a2lfile.pyi` and `py.typed`
- Intended for CPython 3.12

## Limitations

- Parse-only API; this package does not currently provide editing or write-back support
- Intentionally small wrapper surface focused on inspection rather than full ASAP2 authoring workflows
- Currently targeted at CPython 3.12 rather than a broad ABI-compatible interpreter matrix

## Installation

```bash
pip install py-a2lfile
```

The package is installed from PyPI as `py-a2lfile`, but imported in Python as `a2lfile`.

## Example

```python
import a2lfile

a2l = a2lfile.load("example.a2l")

for module in a2l.modules:
    print(module.name)
    for measurement in module.measurements:
        print(" ", measurement.name, measurement.conversion)
```

## Development

```bash
uv sync --dev --python python3.12
uv run maturin develop --uv
uv run pytest
```

## Acknowledgements

This package is a Python wrapper around the Rust crate [`a2lfile`](https://crates.io/crates/a2lfile).

Credit for the underlying A2L parsing implementation belongs to the `a2lfile` authors and maintainers. This wrapper exists to make that parser available from Python, because `a2lfile` was the most capable and well-maintained A2L parser identified for this use case.

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
