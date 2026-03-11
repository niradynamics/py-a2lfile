from pathlib import Path
import subprocess


def main() -> None:
    root = Path(__file__).parent
    cargo_toml = root / "Cargo.toml"
    stub_path = root / "python" / "a2lfile" / "_a2lfile.pyi"

    subprocess.run(
        [
            "cargo",
            "run",
            "--locked",
            "--bin",
            "stub_gen",
            f"--manifest-path={cargo_toml}",
        ],
        check=True,
        cwd=root,
    )

    if not stub_path.is_file():
        raise SystemExit(f"Generated stub file is missing: {stub_path}")

    print(stub_path)


if __name__ == "__main__":
    main()
