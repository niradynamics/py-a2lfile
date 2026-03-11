# `nira-py-a2lfile`

[![Github Actions](https://github.com/Accelerox/a2lfile/actions/workflows/test.yml/badge.svg)](https://github.com/Accelerox/a2lfile/actions)
[![codecov](https://codecov.io/gh/Accelerox/a2lfile/branch/main/graph/badge.svg)](https://codecov.io/gh/Accelerox/a2lfile)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue)](#license)

`nira-py-a2lfile` is a parse-only Python package built with maturin and PyO3 on top of the published Rust crate [`a2lfile`](https://crates.io/crates/a2lfile).

The Python API is intentionally small and read-only. It is aimed at loading A2L files, walking modules and measurements, inspecting conversion metadata such as `COMPU_METHOD`, and traversing generic `IF_DATA` for XCP-related extraction.

## Features

- CPython 3.12 extension module built with maturin
- Read-only wrappers around parsed A2L objects
- Access to modules, measurements, conversion objects, and generic `IF_DATA`
- Checked-in typing support via `_a2lfile.pyi` and `py.typed`

## Local Development

```bash
uv sync --dev --python python3.12
uv run maturin develop --uv
.venv/bin/pytest -q
```

## Install

```bash
pip install nira-py-a2lfile
```

```python
import a2lfile
```

## Example

```python
import a2lfile

a2l = a2lfile.load("example.a2l")

for module in a2l.modules:
    print(module.name)
    for measurement in module.measurements:
        print(" ", measurement.name, measurement.conversion)
```

## Notes

- The Rust package in this repository is named `pya2lfile`.
- The published PyPI package is `nira-py-a2lfile`.
- The runtime import name remains `a2lfile`.
- This repository no longer contains the upstream Rust implementation source; it wraps the published `a2lfile` crate instead.

## License

Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
