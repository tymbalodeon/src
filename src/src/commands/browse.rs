use anyhow::Result;
use std::process::Command;

pub fn browse() -> Result<()> {
    // TODO: allow passing a repo other than the current directory (by path or
    // other repo format)

    // TODO: check if is gh vs glab and then attempt appropriate command

    // TODO: print a message if the command is not found with a message to
    // install
    Command::new("gh")
        .args(["repo", "view", "--web"])
        .status()?;

    Ok(())
}
