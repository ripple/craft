use crate::core::error_codes;

pub mod trace;

// host functions defined by the host.
include!("host_bindings.rs");

/// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]) that better
/// conforms to the xrpld programmability APIs.
#[must_use]
pub enum Result<T> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(Error), // TODO: Test if the WASM size is expanded if we use an enum here instead of i32
}

impl<T> Result<T> {
    /// Returns `true` if the result is [`Ok`].
    #[inline]
    pub fn is_ok(&self) -> bool {
        matches!(*self, Result::Ok(_))
    }

    /// Returns `true` if the result is [`Err`].
    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Converts from `Result<T>` to `Option<T>`.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`,
    /// and discarding the error, if any.
    #[inline]
    pub fn ok(self) -> Option<T> {
        match self {
            Result::Ok(x) => Some(x),
            Result::Err(_) => None,
        }
    }

    /// Converts from `Result<T>` to `Option<Error>`.
    ///
    /// Converts `self` into an `Option<Error>`, consuming `self`,
    /// and discarding the success value, if any.
    #[inline]
    pub fn err(self) -> Option<Error> {
        match self {
            Result::Ok(_) => None,
            Result::Err(x) => Some(x),
        }
    }

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(error) => {
                let _ = trace::trace_num("error_code=", error.code() as i64);
                panic!(
                    "called `Result::unwrap()` on an `Err` with code: {}",
                    error.code()
                )
            }
        }
    }

    /// Returns the contained [`Ok`] value or a provided default.
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(_) => default,
        }
    }

    /// Returns the contained [`Ok`] value or computes it from a closure.
    #[inline]
    pub fn unwrap_or_else<F: FnOnce(Error) -> T>(self, op: F) -> T {
        match self {
            Result::Ok(t) => t,
            Result::Err(e) => op(e),
        }
    }

    #[inline]
    pub fn unwrap_or_panic(self) -> T {
        self.unwrap_or_else(|error| {
            let _ = trace::trace_num("error_code=", error.code() as i64);
            core::panic!(
                "Failed in {}: error_code={}",
                core::panic::Location::caller(),
                error.code()
            );
        })
    }
}

impl From<i64> for Result<u64> {
    #[inline(always)] // <-- Inline because this function is very small
    fn from(value: i64) -> Self {
        match value {
            res if res >= 0 => Result::Ok(value as _),
            _ => Result::Err(Error::from_code(value as _)),
        }
    }
}

/// Possible errors returned by XRPL Programmability APIs.
///
/// Errors are global across all Programmability APIs.
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Error {
    /// A pointer or buffer length provided as a parameter described memory outside of the Hook's allowed memory region.
    OutOfBounds = error_codes::POINTER_OUT_OF_BOUNDS,
    /// Reserved for internal invariant trips, generally unrelated to inputs.
    /// These should be reported with an issue.
    InternalError = error_codes::INTERNAL_ERROR,
    // TODO: Remove Option and check for this error for any optional fields.
    FieldNotFound = error_codes::FIELD_NOT_FOUND,
    NoFreeSlots = error_codes::NO_FREE_SLOTS,
    // /// Attempted to set a parameter or value larger than the allowed space .
    // TooBig = _c::TOO_BIG,
    // /// The API was unable to produce output to the write_ptr because the specified write_len was too small
    // TooSmall = _c::TOO_SMALL,
    // /// The requested object or item wasn't found
    // DoesntExist = _c::DOESNT_EXIST,
    // /// The Hook attempted to allocate an item into a slot, but there were no slots free.
    // /// To avoid ensure re-use of existing slots. The maximum number of slots is 255.
    // NoFreeSlots = _c::NO_FREE_SLOTS,
    // /// One or more of the parameters to the API were invalid according to the individual API's specification.
    // InvalidArgument = _c::INVALID_ARGUMENT,
    // /// Some APIs allow for a once-per-execution parameter to be set.
    // /// A second attempt to set a once-per-execution parameter results in this error.
    // AlreadySet = _c::ALREADY_SET,
    // /// An API required the Hook to do something before the API is allowed to be called.
    // /// Check the API's documentation.
    // PrerequisiteNotMet = _c::PREREQUISITE_NOT_MET,
    // /// During fee calculation if an absurdly large fee is calculated this error is returned.
    // FeeTooLarge = _c::FEE_TOO_LARGE,
    // /// An attempt to emit() a TXN was unsccessful for any of a number of reasons.
    // /// Check the trace log of the rippled to which you are submitting the originating TXN.
    // EmissionFailure = _c::EMISSION_FAILURE,
    // /// A Hook may only use up to 256 calls to nonce() per execution.
    // /// Further calls result in this error code.
    // TooManyNonces = _c::TOO_MANY_NONCES,
    // /// A Hook must declare ahead of time how many TXN it intends to emit().
    // /// If it emits fewer than this many, this is allowed.
    // /// If it emits more than this many this error is returned.
    // TooManyEmittedTxn = _c::TOO_MANY_EMITTED_TXN,
    // /// While Hooks is/was in development an API may return this if some or all of that API is planned but not yet implemented.
    // NotImplemented = _c::NOT_IMPLEMENTED,
    // /// An API which accepts a 20 byte Account ID may return this if, in its opinion, the Account ID was not valid for any reason.
    // InvalidAccount = _c::INVALID_ACCOUNT,
    // /// All loops inside a Hook must declare at the top of the loop, as the first non trivial instruction,
    // /// before any branch instruction, the promised maximum number of iterations of the loop.
    // /// If this promise is violated the hook terminates immediately with this error code.
    // GuardViolation = _c::GUARD_VIOLATION,
    // The requested serialized field could not be found in the specified object.
    // InvalidField = _c::INVALID_FIELD,
    // /// While parsing serialized content an error was encountered (typically indicating an invalidly serialized object).
    // ParseError = _c::PARSE_ERROR,
    // /// Used internally to communicate a rollback event.
    // RcRollback = _c::RC_ROLLBACK,
    // /// Used internally to communicate an accept event.
    // RcAccept = _c::RC_ACCEPT,
    // /// Specified keylet could not be found, or keylet is invalid
    // NoSuchKeylet = _c::NO_SUCH_KEYLET,
    // /// API was asked to assume object under analysis is an STArray but it was not.
    // NotAnArray = -22,
    // /// API was asked to assume object under analysis is an STObject but it was not.
    // NotAnObject = -23,
    // /// A floating point operation resulted in Not-A-Number or API call attempted to specify an XFL floating point number outside of the expressible range of XFL.
    // InvalidFloat = _c::INVALID_FLOAT,
    // /// API call would result in a division by zero, so API ended early.
    // DivisionByZero = -25,
    // /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    // ManitssaOversized = -26,
    // /// When attempting to create an XFL the mantissa must be 16 decimal digits.
    // MantissaUndersized = -27,
    // /// When attempting to create an XFL the exponent must not exceed 80.
    // ExponentOversized = -28,
    // /// When attempting to create an XFL the exponent must not be less than -96.
    // ExponentUndersized = -29,
    // /// A floating point operation done on an XFL resulted in a value larger than XFL format is able to represent.
    // Overflow = -30,
    // /// An API assumed an STAmount was an IOU when in fact it was XRP.
    // NotIouAmount = -31,
    // /// An API assumed an STObject was an STAmount when in fact it was not.
    // NotAnAmount = -32,
    // /// An API would have returned a negative integer except that negative integers are reserved for error codes (i.e. what you are reading.)
    // CantReturnNegative = -33,
}

impl Error {
    // TODO: Use Trait instead?
    #[inline(always)] // <-- Inline because this function is very small
    pub fn from_code(code: i32) -> Self {
        unsafe { core::mem::transmute(code) }
    }

    /// Error code
    #[inline(always)] // <-- Inline because this function is very small
    pub fn code(self) -> i32 {
        self as _
    }
}

impl From<Error> for i64 {
    fn from(val: Error) -> Self {
        val as i64
    }
}

/// Converts a `Result<Option<T>>` to a `Result<T>` by treating `None` as an error.
///
/// This utility function is commonly used in the XRPL Programmability API context
/// where operations may return optional values that should be treated as errors
/// when absent.
///
/// # Arguments
///
/// * `result` - A `Result` containing an `Option<T>` that needs to be unwrapped
///
/// # Returns
///
/// * `Result::Ok(value)` - If the input was `Result::Ok(Some(value))`
/// * `Result::Err(Error::FieldNotFound)` - If the input was `Result::Ok(None)`
/// * `Result::Err(err)` - If the input was `Result::Err(err)`, the error is propagated
///
/// # Error Handling
///
/// When the optional value is `None`, this function returns `Error::FieldNotFound`,
/// which is appropriate for cases where a required field or value is missing from
/// XRPL ledger objects or API responses.
pub(crate) fn to_non_optional<T>(result: Result<Option<T>>) -> Result<T> {
    match result {
        Result::Ok(Some(value)) => Result::Ok(value),
        Result::Ok(None) => Result::Err(Error::FieldNotFound),
        Result::Err(err) => Result::Err(err),
    }
}
