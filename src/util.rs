macro_rules! ensure_len {
    ($vec:ident, $len:expr, $default:expr) => {
        if $len > $vec.len() {
            $vec.extend_from_slice(&vec![$default; $len - $vec.len()])
        }
    };
}

pub(crate) use ensure_len;
