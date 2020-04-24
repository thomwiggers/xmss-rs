use std::os::raw;

#[repr(C)]
struct XMSSParams {
    func: raw::c_uint,
    n: raw::c_uint,
    wots_w: raw::c_uint,
    wots_log_w: raw::c_uint,
    wots_len1: raw::c_uint,
    wots_len2: raw::c_uint,
    wots_len: raw::c_uint,
    wots_sig_bytes: raw::c_uint,
    full_height: raw::c_uint,
    tree_height: raw::c_uint,
    d: raw::c_uint,
    index_bytes: raw::c_uint,
    sig_bytes: raw::c_uint,
    pk_bytes: raw::c_uint,
    sk_bytes: raw::c_ulonglong,
    bds_k: raw::c_uint,
}


#[link(name = "xmss")]
extern "C" {
    fn xmss_core_sign_verify(params: *const XMSSParams, m: *const raw::c_uchar, mlen: raw::c_ulonglong, sig: *const raw::c_uchar, pk: *const raw::c_uchar) -> raw::c_int;
    fn xmss_core_keypair(params: *const XMSSParams, pk: *mut raw::c_uchar, sk: *mut raw::c_uchar) -> raw::c_int;
    fn xmss_core_sign_signature(params: *const XMSSParams, sk: *mut raw::c_uchar, sig: *mut raw::c_uchar, m: *const raw::c_uchar, mlen: raw::c_ulonglong) -> raw::c_int;

}

// use checkparams script
const XMSS_SETTINGS: XMSSParams = XMSSParams{
    func: 0,
    n: 32,
    wots_w: 16,
    wots_log_w: 4,
    wots_len1: 64,
    wots_len2: 3,
    wots_len: 67,
    wots_sig_bytes: 2144,
    full_height: 10,
    tree_height: 10,
    d: 1,
    index_bytes: 4,
    sig_bytes: 2500,
    pk_bytes: 64,
    sk_bytes: 1373,
    bds_k: 0,
};

pub fn verify(msg: &[u8], sig: &[u8], pk: &[u8]) -> bool {
    let res = unsafe {
        xmss_core_sign_verify(
            &XMSS_SETTINGS as *const _,
            msg.as_ptr(), msg.len() as raw::c_ulonglong,
            sig.as_ptr(), pk.as_ptr())
    };

    return res == 0;
}

pub fn keypair() -> (Vec<u8>, Vec<u8>) {
    let mut pk = Vec::with_capacity(XMSS_SETTINGS.pk_bytes as usize);
    let mut sk = Vec::with_capacity(XMSS_SETTINGS.sk_bytes as usize);

    let res = unsafe {
        xmss_core_keypair(
            &XMSS_SETTINGS as *const _,
            pk.as_mut_ptr(), sk.as_mut_ptr())
    };
    assert!(res == 0);

    unsafe{
        pk.set_len(XMSS_SETTINGS.pk_bytes as usize);
        sk.set_len(XMSS_SETTINGS.sk_bytes as usize);
    }

    return (pk, sk);
}

pub fn sign(sk: &mut [u8], msg: &[u8]) -> Vec<u8> {
    let mut sig = Vec::with_capacity(XMSS_SETTINGS.sig_bytes as usize);

    let res = unsafe {
        xmss_core_sign_signature(
            &XMSS_SETTINGS as *const _,
            sk.as_mut_ptr(),
            sig.as_mut_ptr(),
            msg.as_ptr(),
            msg.len() as raw::c_ulonglong,
        )
    };
    assert!(res == 0);

    unsafe {
        sig.set_len(XMSS_SETTINGS.sig_bytes as usize);
    }
    return sig;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_keygen_sign_verify() {
        let msg = [0; 100];
        let (pk, mut sk) = keypair();
        let orig_sk = sk.clone();
        let sig = sign(&mut sk, &msg);
        assert!(orig_sk != sk, "sk not updated");
        assert!(verify(&msg, &sig, &pk));
    }
}