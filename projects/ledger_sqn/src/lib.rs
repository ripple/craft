#![no_std]
use xrpl_std::host;

#[unsafe(no_mangle)]
pub fn finish() -> i32 {
    let mut sqn = 0i32;
    let res = unsafe { host::get_ledger_sqn((&mut sqn) as *mut i32 as *mut u8,4,)};
    if res > 0 {
        sqn
    } else {
        res
    } 
}

// pub fn finish() -> i32 {
//     let mut h = [0; 32];    
//     unsafe { host::get_parent_ledger_hash(h.as_mut_ptr(),32,)}  
// }
