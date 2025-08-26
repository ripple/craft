pub mod assert;
pub mod error_codes;
mod host_bindings;
mod host_bindings_for_testing;
pub mod trace;

//////////////////////////////////////
// Host functions (defined by the host)
//////////////////////////////////////

#[cfg(not(target_arch = "wasm32"))]
include!("host_bindings_for_testing.rs");

// host functions defined by the host.
#[cfg(target_arch = "wasm32")]
include!("host_bindings.rs");

/// `Result` is a type alias to the standard Result with our custom Error type.
/// This maintains API compatibility while using standard Rust error handling.
pub type Result<T> = core::result::Result<T, Error>;

// Helper functions for WASM response code handling
pub fn unwrap_or_panic<T>(result: Result<T>) -> T {
    result.unwrap_or_else(|error| {
        let _ = trace::trace_num("error_code=", error.code() as i64);
        core::panic!(
            "Failed in {}: error_code={}",
            core::panic::Location::caller(),
            error.code()
        );
    })
}

pub fn result_from_i64(value: i64) -> Result<u64> {
    match value {
        res if res >= 0 => Ok(value as _),
        _ => Err(Error::from_code(value as _)),
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
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(Error::FieldNotFound),
        Err(err) => Err(err),
    }
}

/// Possible errors returned by XRPL Programmability APIs.
///
/// Errors are global across all Programmability APIs.
/// Each error variant maps to a specific negative i32 code used in WASM response codes.
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Error {
    /// Reserved for internal invariant trips, generally unrelated to inputs.
    /// These should be reported with an issue.
    InternalError = error_codes::INTERNAL_ERROR,

    /// The requested serialized field could not be found in the specified object.
    /// This error is returned when attempting to access a field that doesn't exist
    /// in the current transaction or ledger object.
    FieldNotFound = error_codes::FIELD_NOT_FOUND,

    /// The provided buffer is too small to hold the requested data.
    /// Increase the buffer size and retry the operation.
    BufferTooSmall = error_codes::BUFFER_TOO_SMALL,

    /// The API was asked to assume the object under analysis is an STArray but it was not.
    /// This error occurs when trying to perform array operations on non-array objects.
    NoArray = error_codes::NO_ARRAY,

    /// The specified field is not a leaf field and cannot be accessed directly.
    /// Leaf fields are primitive types that contain actual data values.
    NotLeafField = error_codes::NOT_LEAF_FIELD,

    /// The provided locator string is malformed or invalid.
    /// Locators must follow the proper format for field identification.
    LocatorMalformed = error_codes::LOCATOR_MALFORMED,

    /// The specified slot number is outside the valid range.
    /// Slot numbers must be within the allowed bounds for the current context.
    SlotOutRange = error_codes::SLOT_OUT_RANGE,

    /// No free slots are available for allocation.
    /// All available slots are currently in use. Consider reusing existing slots.
    SlotsFull = error_codes::SLOTS_FULL,

    /// The specified slot did not contain any slotted data (i.e., is empty).
    /// This error occurs when trying to access a slot that hasn't been allocated
    /// or has been freed.
    EmptySlot = error_codes::EMPTY_SLOT,

    /// The requested ledger object could not be found.
    /// This may occur if the object doesn't exist or the keylet is invalid.
    LedgerObjNotFound = error_codes::LEDGER_OBJ_NOT_FOUND,

    /// An error occurred while decoding serialized data.
    /// This typically indicates corrupted or invalidly formatted data.
    InvalidDecoding = error_codes::INVALID_DECODING,

    /// The data field is too large to be processed.
    /// Consider reducing the size of the data or splitting it into smaller chunks.
    DataFieldTooLarge = error_codes::DATA_FIELD_TOO_LARGE,

    /// A pointer or buffer length provided as a parameter described memory outside the allowed memory region.
    /// This error indicates a memory access violation.
    PointerOutOfBounds = error_codes::POINTER_OUT_OF_BOUNDS,

    /// No memory has been exported by the WebAssembly module.
    /// The module must export its memory for host functions to access it.
    NoMemoryExported = error_codes::NO_MEM_EXPORTED,

    /// One or more of the parameters provided to the API are invalid.
    /// Check the API documentation for valid parameter ranges and formats.
    InvalidParams = error_codes::INVALID_PARAMS,

    /// The provided account identifier is invalid.
    /// Account IDs must be valid 20-byte addresses in the proper format.
    InvalidAccount = error_codes::INVALID_ACCOUNT,

    /// The specified field identifier is invalid or not recognized.
    /// Field IDs must correspond to valid XRPL serialization fields.
    InvalidField = error_codes::INVALID_FIELD,

    /// The specified index is outside the valid bounds of the array or collection.
    /// Ensure the index is within the valid range for the target object.
    IndexOutOfBounds = error_codes::INDEX_OUT_OF_BOUNDS,

    /// The input provided for floating-point parsing is malformed.
    /// Floating-point values must be in the correct format for XFL operations.
    InvalidFloatInput = error_codes::INVALID_FLOAT_INPUT,

    /// An error occurred during floating-point computation.
    /// This may indicate overflow, underflow, or other arithmetic errors.
    InvalidFloatComputation = error_codes::INVALID_FLOAT_COMPUTATION,
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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = match self {
            Error::InternalError => "Internal error occurred",
            Error::FieldNotFound => "Requested field not found",
            Error::BufferTooSmall => "Buffer too small to hold requested data",
            Error::NoArray => "Object is not an STArray",
            Error::NotLeafField => "Field is not a leaf field",
            Error::LocatorMalformed => "Locator string is malformed",
            Error::SlotOutRange => "Slot number is out of range",
            Error::SlotsFull => "No free slots available",
            Error::EmptySlot => "Slot is empty",
            Error::LedgerObjNotFound => "Ledger object not found",
            Error::InvalidDecoding => "Invalid data decoding",
            Error::DataFieldTooLarge => "Data field too large",
            Error::PointerOutOfBounds => "Pointer out of bounds",
            Error::NoMemoryExported => "No memory exported by WASM module",
            Error::InvalidParams => "Invalid parameters",
            Error::InvalidAccount => "Invalid account identifier",
            Error::InvalidField => "Invalid field identifier",
            Error::IndexOutOfBounds => "Index out of bounds",
            Error::InvalidFloatInput => "Invalid float input",
            Error::InvalidFloatComputation => "Invalid float computation",
        };
        write!(f, "{} (code: {})", message, self.code())
    }
}

impl core::error::Error for Error {}
