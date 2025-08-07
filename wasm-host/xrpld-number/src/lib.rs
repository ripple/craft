mod xrpl_iou_value;

// Include the generated bindings in a module to avoid naming conflicts
mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CStr;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

/// Error types for Number operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Overflow,
    DivideByZero,
    InvalidArgument,
    OutOfMemory,
    Unknown,
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::Overflow => write!(f, "Number overflow"),
            NumberError::DivideByZero => write!(f, "Division by zero"),
            NumberError::InvalidArgument => write!(f, "Invalid argument"),
            NumberError::OutOfMemory => write!(f, "Out of memory"),
            NumberError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for NumberError {}

impl From<ffi::NumberError> for NumberError {
    fn from(error: ffi::NumberError) -> Self {
        match error {
            ffi::NumberError_NUMBER_ERROR_OVERFLOW => NumberError::Overflow,
            ffi::NumberError_NUMBER_ERROR_DIVIDE_BY_ZERO => NumberError::DivideByZero,
            ffi::NumberError_NUMBER_ERROR_INVALID_ARGUMENT => NumberError::InvalidArgument,
            ffi::NumberError_NUMBER_ERROR_OUT_OF_MEMORY => NumberError::OutOfMemory,
            _ => NumberError::Unknown,
        }
    }
}

/// Rounding mode for Number operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    ToNearest,
    TowardsZero,
    Downward,
    Upward,
}

impl From<RoundingMode> for ffi::RoundingMode {
    fn from(mode: RoundingMode) -> Self {
        match mode {
            RoundingMode::ToNearest => ffi::RoundingMode_ROUNDING_TO_NEAREST,
            RoundingMode::TowardsZero => ffi::RoundingMode_ROUNDING_TOWARDS_ZERO,
            RoundingMode::Downward => ffi::RoundingMode_ROUNDING_DOWNWARD,
            RoundingMode::Upward => ffi::RoundingMode_ROUNDING_UPWARD,
        }
    }
}

impl From<ffi::RoundingMode> for RoundingMode {
    fn from(mode: ffi::RoundingMode) -> Self {
        match mode {
            ffi::RoundingMode_ROUNDING_TO_NEAREST => RoundingMode::ToNearest,
            ffi::RoundingMode_ROUNDING_TOWARDS_ZERO => RoundingMode::TowardsZero,
            ffi::RoundingMode_ROUNDING_DOWNWARD => RoundingMode::Downward,
            ffi::RoundingMode_ROUNDING_UPWARD => RoundingMode::Upward,
            _ => RoundingMode::ToNearest,
        }
    }
}

/// High-precision decimal number
pub struct Number {
    ptr: *mut ffi::Number,
}

unsafe impl Send for Number {}
unsafe impl Sync for Number {}

impl Number {
    /// Create a new Number initialized to zero
    pub fn new() -> Self {
        let ptr = unsafe { ffi::number_new() };
        if ptr.is_null() {
            panic!("Failed to allocate Number");
        }
        Number { ptr }
    }

    /// Create a Number from an i64 mantissa
    pub fn from_i64(mantissa: i64) -> Result<Self, NumberError> {
        let mut error = ffi::NumberError_NUMBER_SUCCESS;
        let ptr = unsafe { ffi::number_new_from_int64(mantissa, &mut error) };
        if ptr.is_null() {
            return Err(error.into());
        }
        Ok(Number { ptr })
    }

    /// Create a Number from mantissa and exponent
    pub fn from_mantissa_exponent(mantissa: i64, exponent: i32) -> Result<Self, NumberError> {
        let mut error = ffi::NumberError_NUMBER_SUCCESS;
        let ptr = unsafe { ffi::number_new_from_mantissa_exponent(mantissa, exponent, &mut error) };
        if ptr.is_null() {
            return Err(error.into());
        }
        Ok(Number { ptr })
    }

    /// Get the mantissa of this Number
    pub fn mantissa(&self) -> i64 {
        unsafe { ffi::number_get_mantissa(self.ptr) }
    }

    /// Get the exponent of this Number
    pub fn exponent(&self) -> i32 {
        unsafe { ffi::number_get_exponent(self.ptr) }
    }

    /// Convert to i64 (with potential precision loss)
    pub fn to_i64(&self) -> Result<i64, NumberError> {
        let mut result = 0i64;
        let error = unsafe { ffi::number_to_int64(self.ptr, &mut result) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Get the sign of this Number (-1, 0, or 1)
    pub fn signum(&self) -> i32 {
        unsafe { ffi::number_signum(self.ptr) }
    }

    /// Check if this Number is zero
    pub fn is_zero(&self) -> bool {
        unsafe { ffi::number_is_zero(self.ptr) }
    }

    /// Compute absolute value
    pub fn abs(&self) -> Result<Self, NumberError> {
        let result = Number::new();
        let error = unsafe { ffi::number_abs(result.ptr, self.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Raise this Number to an unsigned integer power
    pub fn pow(&self, exponent: u32) -> Result<Self, NumberError> {
        let result = Number::new();
        let error = unsafe { ffi::number_power_uint(result.ptr, self.ptr, exponent) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Compute the nth root of this Number
    pub fn root(&self, degree: u32) -> Result<Self, NumberError> {
        let result = Number::new();
        let error = unsafe { ffi::number_root(result.ptr, self.ptr, degree) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Compute the square root of this Number
    pub fn sqrt(&self) -> Result<Self, NumberError> {
        let result = Number::new();
        let error = unsafe { ffi::number_sqrt(result.ptr, self.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Compute the base-10 logarithm of this Number
    pub fn log10(&self) -> Result<Self, NumberError> {
        let result = Number::new();
        let error = unsafe { ffi::number_log10(result.ptr, self.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }

    /// Get the current global rounding mode
    pub fn get_rounding_mode() -> RoundingMode {
        let mode = unsafe { ffi::number_get_rounding_mode() };
        mode.into()
    }

    /// Set the global rounding mode, returning the previous mode
    pub fn set_rounding_mode(mode: RoundingMode) -> RoundingMode {
        let prev_mode = unsafe { ffi::number_set_rounding_mode(mode.into()) };
        prev_mode.into()
    }

    /// Get the minimum representable Number
    pub fn min() -> Self {
        let ptr = unsafe { ffi::number_min() };
        if ptr.is_null() {
            panic!("Failed to create min Number");
        }
        Number { ptr }
    }

    /// Get the maximum representable Number
    pub fn max() -> Self {
        let ptr = unsafe { ffi::number_max() };
        if ptr.is_null() {
            panic!("Failed to create max Number");
        }
        Number { ptr }
    }

    /// Get the lowest (most negative) representable Number
    pub fn lowest() -> Self {
        let ptr = unsafe { ffi::number_lowest() };
        if ptr.is_null() {
            panic!("Failed to create lowest Number");
        }
        Number { ptr }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        let mut error = ffi::NumberError_NUMBER_SUCCESS;
        let ptr = unsafe { ffi::number_clone(self.ptr, &mut error) };
        if ptr.is_null() {
            panic!("Failed to clone Number: {:?}", NumberError::from(error));
        }
        Number { ptr }
    }
}

impl Drop for Number {
    fn drop(&mut self) {
        unsafe { ffi::number_free(self.ptr) };
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = unsafe { ffi::number_string_length(self.ptr) };
        let mut buffer = vec![0u8; len + 1];
        let error = unsafe { 
            ffi::number_to_string(self.ptr, buffer.as_mut_ptr() as *mut i8, buffer.len()) 
        };
        if error != ffi::NumberError_NUMBER_SUCCESS {
            return Err(fmt::Error);
        }
        let cstr = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
        let s = cstr.to_str().map_err(|_| fmt::Error)?;
        write!(f, "{}", s)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Number")
            .field("mantissa", &self.mantissa())
            .field("exponent", &self.exponent())
            .field("value", &self.to_string())
            .finish()
    }
}

// Arithmetic operations
impl Add for &Number {
    type Output = Result<Number, NumberError>;

    fn add(self, rhs: &Number) -> Self::Output {
        let result = Number::new();
        let error = unsafe { ffi::number_add(result.ptr, self.ptr, rhs.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }
}

impl Add for Number {
    type Output = Result<Number, NumberError>;

    fn add(self, rhs: Number) -> Self::Output {
        &self + &rhs
    }
}

impl Sub for &Number {
    type Output = Result<Number, NumberError>;

    fn sub(self, rhs: &Number) -> Self::Output {
        let result = Number::new();
        let error = unsafe { ffi::number_subtract(result.ptr, self.ptr, rhs.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }
}

impl Sub for Number {
    type Output = Result<Number, NumberError>;

    fn sub(self, rhs: Number) -> Self::Output {
        &self - &rhs
    }
}

impl Mul for &Number {
    type Output = Result<Number, NumberError>;

    fn mul(self, rhs: &Number) -> Self::Output {
        let result = Number::new();
        let error = unsafe { ffi::number_multiply(result.ptr, self.ptr, rhs.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }
}

impl Mul for Number {
    type Output = Result<Number, NumberError>;

    fn mul(self, rhs: Number) -> Self::Output {
        &self * &rhs
    }
}

impl Div for &Number {
    type Output = Result<Number, NumberError>;

    fn div(self, rhs: &Number) -> Self::Output {
        let result = Number::new();
        let error = unsafe { ffi::number_divide(result.ptr, self.ptr, rhs.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }
}

impl Div for Number {
    type Output = Result<Number, NumberError>;

    fn div(self, rhs: Number) -> Self::Output {
        &self / &rhs
    }
}

impl Neg for &Number {
    type Output = Result<Number, NumberError>;

    fn neg(self) -> Self::Output {
        let result = Number::new();
        let error = unsafe { ffi::number_negate(result.ptr, self.ptr) };
        if error == ffi::NumberError_NUMBER_SUCCESS {
            Ok(result)
        } else {
            Err(error.into())
        }
    }
}

impl Neg for Number {
    type Output = Result<Number, NumberError>;

    fn neg(self) -> Self::Output {
        -&self
    }
}

// Assignment operations
impl AddAssign<&Number> for Number {
    fn add_assign(&mut self, rhs: &Number) {
        let error = unsafe { ffi::number_add_assign(self.ptr, rhs.ptr) };
        if error != ffi::NumberError_NUMBER_SUCCESS {
            panic!("Number addition failed: {:?}", NumberError::from(error));
        }
    }
}

impl AddAssign<Number> for Number {
    fn add_assign(&mut self, rhs: Number) {
        *self += &rhs;
    }
}

impl SubAssign<&Number> for Number {
    fn sub_assign(&mut self, rhs: &Number) {
        let error = unsafe { ffi::number_subtract_assign(self.ptr, rhs.ptr) };
        if error != ffi::NumberError_NUMBER_SUCCESS {
            panic!("Number subtraction failed: {:?}", NumberError::from(error));
        }
    }
}

impl SubAssign<Number> for Number {
    fn sub_assign(&mut self, rhs: Number) {
        *self -= &rhs;
    }
}

impl MulAssign<&Number> for Number {
    fn mul_assign(&mut self, rhs: &Number) {
        let error = unsafe { ffi::number_multiply_assign(self.ptr, rhs.ptr) };
        if error != ffi::NumberError_NUMBER_SUCCESS {
            panic!("Number multiplication failed: {:?}", NumberError::from(error));
        }
    }
}

impl MulAssign<Number> for Number {
    fn mul_assign(&mut self, rhs: Number) {
        *self *= &rhs;
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        let error = unsafe { ffi::number_divide_assign(self.ptr, rhs.ptr) };
        if error != ffi::NumberError_NUMBER_SUCCESS {
            panic!("Number division failed: {:?}", NumberError::from(error));
        }
    }
}

impl DivAssign<Number> for Number {
    fn div_assign(&mut self, rhs: Number) {
        *self /= &rhs;
    }
}

// Comparison operations
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::number_equals(self.ptr, other.ptr) }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if unsafe { ffi::number_less_than(self.ptr, other.ptr) } {
            std::cmp::Ordering::Less
        } else if unsafe { ffi::number_greater_than(self.ptr, other.ptr) } {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

// Convenience constructors
impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::from_i64(value).expect("Failed to create Number from i64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = Number::from(100);
        let b = Number::from(50);
        
        let sum = (&a + &b).expect("Addition failed");
        assert_eq!(sum.to_i64().expect("Conversion failed"), 150);
        
        let diff = (&a - &b).expect("Subtraction failed");
        assert_eq!(diff.to_i64().expect("Conversion failed"), 50);
        
        let prod = (&a * &b).expect("Multiplication failed");
        assert_eq!(prod.to_i64().expect("Conversion failed"), 5000);
        
        let quot = (&a / &b).expect("Division failed");
        assert_eq!(quot.to_i64().expect("Conversion failed"), 2);
    }

    #[test]
    fn test_comparison() {
        let a = Number::from(100);
        let b = Number::from(50);
        let c = Number::from(100);
        
        assert!(a > b);
        assert!(b < a);
        assert!(a == c);
        assert!(a >= c);
        assert!(b <= a);
    }

    #[test]
    fn test_display() {
        let n = Number::from(12345);
        let s = format!("{}", n);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_mantissa_exponent() {
        let n = Number::from_mantissa_exponent(12345, -2).expect("Failed to create number");
        // The Number class normalizes values, so the actual mantissa/exponent may differ
        // Let's just verify the value is correct when converted back
        assert_eq!(n.to_i64().expect("Conversion failed"), 123);
    }

    #[test] 
    fn test_zero() {
        let zero = Number::new();
        assert!(zero.is_zero());
        assert_eq!(zero.signum(), 0);
    }

    #[test]
    fn test_mathematical_functions() {
        let four = Number::from(4);
        let sqrt_four = four.sqrt().expect("Square root failed");
        assert_eq!(sqrt_four.to_i64().expect("Conversion failed"), 2);

        let two = Number::from(2);
        let eight = two.pow(3).expect("Power failed");
        assert_eq!(eight.to_i64().expect("Conversion failed"), 8);
    }
}