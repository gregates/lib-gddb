macro_rules! ensure_len {
    ($vec:ident, $len:expr, $default:expr) => {
        if $vec.len() <= $len {
            $vec.extend_from_slice(&vec![$default; $len - $vec.len() + 1])
        }
    };
}

pub(crate) use ensure_len;
