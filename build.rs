use rustc_version::version;
use std::{error::Error, fs::write};

fn main() -> Result<(), Box<dyn Error>> {
    write(
        "version.txt",
        format!(
            "\"@infobip/rust-sdk/{} rust/{}\"",
            env!("CARGO_PKG_VERSION"),
            version()?
        ),
    )?;

    Ok(())
}
