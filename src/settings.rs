use crate::XMSSParams;
pub(crate) const XMSS_SETTINGS: XMSSParams = XMSSParams {
    func: 1,
    n: 16,
    wots_w: 256,
    wots_log_w: 8,
    wots_len1: 16,
    wots_len2: 2,
    wots_len: 18,
    wots_sig_bytes: 288,
    full_height: 16,
    tree_height: 8,
    d: 2,
    index_bytes: 2,
    sig_bytes: 850,
    pk_bytes: 32,
    sk_bytes: 1965,
    bds_k: 0,
};