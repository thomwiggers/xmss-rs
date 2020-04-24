use std::fs;

use xmss_rs::verify;

fn parseargs() -> (String, String, String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        panic!("Usage: {} pk msg sig", args[0]);
    }
    (args[1].clone(), args[2].clone(), args[3].clone())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (pk_filename, in_filename, sig_filename) = parseargs();
    let pk = &fs::read(pk_filename)?;
    let msg = &fs::read(in_filename)?;
    let sig = &fs::read(sig_filename)?;

    if verify(msg, sig, pk) {
        println!("Verification success!");
    } else {
        println!("Verification failed!");
    }

    Ok(())
}
