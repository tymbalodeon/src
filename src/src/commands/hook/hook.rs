use anyhow::Result;

const HOOK: &str = include_str!("./hook.nu");

pub fn hook() -> Result<()> {
    print!("{HOOK}");

    Ok(())
}
