// SPDX-License-Identifier: GPL-3.0-only
use anyhow::{bail, ensure, Result};
use emwm_appearance::{images, lightdm};
use std::io::{self, Read};
use std::path::Path;

fn run() -> Result<()> {
    // SAFETY: geteuid takes no arguments and has no preconditions.
    ensure!(
        unsafe { libc::geteuid() } == 0,
        "Esta operação requer autenticação administrativa."
    );
    let root = Path::new("/");
    let args: Vec<_> = std::env::args().skip(1).collect();
    let backup = match args.as_slice() {
        [option, mode] if option == "--background" => {
            let mut data = Vec::new();
            io::stdin()
                .lock()
                .take(images::MAX_BYTES as u64 + 1)
                .read_to_end(&mut data)?;
            lightdm::wallpaper(root, &data, mode)?
        }
        [theme, icons, cursor, size] => lightdm::theme(root, theme, icons, cursor, size.parse()?)?,
        _ => bail!("Uso: login_helper.py TEMA ICONES CURSOR TAMANHO"),
    };
    println!("{}", backup.display());
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
