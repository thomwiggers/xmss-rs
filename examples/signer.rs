use std::fs::{self, File};
use std::io::prelude::*;

use xmss_rs::sign;


fn parseargs() -> (String, String, String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        panic!("Usage: {} sk in out", args[0]);
    }
    (args[1].clone(), args[2].clone(), args[3].clone())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sk_filename, in_filename, sig_filename) = parseargs();

    let (sk, signature) = {
        let sk = &fs::read(sk_filename.clone())?;
        let mut sk = sk.to_vec();
        let signature = sign(&mut sk, &fs::read(in_filename)?);
        (sk, signature)
    };

    let mut sigfile = File::create(sig_filename)?;
    let mut skfile = File::create(sk_filename)?;
    skfile.write_all(sk.as_ref())?;
    sigfile.write_all(signature.as_ref())?;

    Ok(())
}
