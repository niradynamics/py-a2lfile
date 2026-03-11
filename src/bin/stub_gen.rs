use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    pya2lfile::stub_info()?.generate()?;
    Ok(())
}
