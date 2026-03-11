from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
CARGO_TOML = ROOT / "Cargo.toml"
CARGO_LOCK = ROOT / "Cargo.lock"
TAG_PATTERN = re.compile(r"^(?:refs/tags/)?v(?P<version>\d+\.\d+\.\d+)$")


def _replace_once(path: Path, pattern: str, replacement: str) -> None:
    text = path.read_text(encoding="utf-8")
    updated, count = re.subn(pattern, replacement, text, count=1, flags=re.MULTILINE | re.DOTALL)
    if count != 1:
        raise SystemExit(f"failed to update {path}")
    path.write_text(updated, encoding="utf-8")


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: python scripts/set_release_version.py <tag>")

    match = TAG_PATTERN.fullmatch(sys.argv[1].strip())
    if match is None:
        raise SystemExit(
            "release tags must look like vMAJOR.MINOR.PATCH, for example v3.3.3"
        )

    version = match.group("version")

    _replace_once(
        CARGO_TOML,
        r'(^\[package\]\n.*?^version = ")([^"]+)(")',
        rf'\g<1>{version}\3',
    )
    _replace_once(
        CARGO_LOCK,
        r'(^\[\[package\]\]\nname = "pya2lfile"\nversion = ")([^"]+)(")',
        rf'\g<1>{version}\3',
    )

    print(version)


if __name__ == "__main__":
    main()
