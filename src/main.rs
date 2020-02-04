use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Result, Write};
use std::os::unix::fs::OpenOptionsExt;

fn main() -> Result<()> {
    let mut p = env::temp_dir();
    p.push("tmp-ssh-key");

    let _ = fs::remove_file(&p);

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .mode(0o600)
        .open(&p)?;

    let mut buf = vec![];
    io::stdin().read_to_end(&mut buf)?;
    f.write_all(&buf)?;
    f.sync_all()?;

    if let Some(path) = p.to_str() {
        io::stdout().write_all(path.as_bytes())?;
    } else {
        panic!("bad path")
    }

    Ok(())
}
