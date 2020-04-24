use std::fs::File;
use std::io::prelude::*;

use xmss_rs::keypair;

fn main() -> std::io::Result<()> {
    let (pk, sk) = keypair();
    let mut pubfile = File::create("publickey.bin")?;
    let mut secfile = File::create("secretkey.bin")?;
    pubfile.write_all(pk.as_ref())?;
    secfile.write_all(sk.as_ref())?;
    Ok(())
}
