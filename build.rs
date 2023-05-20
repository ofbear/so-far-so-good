use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    EmitBuilder::builder()
        .quiet()
        .all_cargo()
        .all_git()
        .emit()?;
    Ok(())
}
