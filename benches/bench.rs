#![feature(test)]
extern crate test;

use test::Bencher;

mod bench_xmss {
    use super::*;

    use xmss_rs::{
        keypair, sign, verify
    };

    #[bench]
    fn bench_keypair(b: &mut Bencher) {
        b.iter(|| keypair());
    }

    #[bench]
    fn bench_sign(b: &mut Bencher) {
        let msg = [0u8; 100];
        let (_pk, mut sk) = keypair();
        b.iter(|| sign(&mut sk, &msg));
    }

    #[bench]
    fn bench_verify(b: &mut Bencher) {
        let msg = [0u8; 100];
        let (pk, mut sk) = keypair();
        let sig = sign(&mut sk, &msg);
        b.iter(|| verify(&msg, &sig, &pk));
    }
}
