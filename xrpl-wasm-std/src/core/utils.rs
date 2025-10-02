#[macro_export]
macro_rules! require {
    ($condition:expr, $error_code:expr, $error_msg:expr) => {
        if !$condition {
            unsafe {
                let msg = $error_msg.as_bytes();
                exit($error_code, msg.as_ptr(), msg.len() as u32);
            }
        }
    };
    // Simplified version with default error code
    ($condition:expr, $error_msg:expr) => {
        require!($condition, -1, $error_msg);
    };
}