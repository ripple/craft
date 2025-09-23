#include "number_c.h"
#include "Number.h"
#include <cstring>
#include <new>
#include <stdexcept>
#include <string>

// Helper functions to convert between C and C++ types
static ripple::Number *to_cpp(Number *num) {
  return reinterpret_cast<ripple::Number *>(num);
}

static const ripple::Number *to_cpp(const Number *num) {
  return reinterpret_cast<const ripple::Number *>(num);
}

static Number *to_c(ripple::Number *num) {
  return reinterpret_cast<Number *>(num);
}

// Helper function to convert C++ exceptions to error codes
NumberError handle_exception() {
  try {
    throw;
  } catch (const std::overflow_error &) {
    return NUMBER_ERROR_OVERFLOW;
  } catch (const std::invalid_argument &) {
    return NUMBER_ERROR_INVALID_ARGUMENT;
  } catch (const std::bad_alloc &) {
    return NUMBER_ERROR_OUT_OF_MEMORY;
  } catch (...) {
    return NUMBER_ERROR_UNKNOWN;
  }
}

// Helper macro for exception handling
#define TRY_CATCH_RETURN_ERROR(code)                                           \
  try {                                                                        \
    code return NUMBER_SUCCESS;                                                \
  } catch (...) {                                                              \
    return handle_exception();                                                 \
  }

#define TRY_CATCH_RETURN_NULL(code)                                            \
  try {                                                                        \
    code                                                                       \
  } catch (...) {                                                              \
    if (error)                                                                 \
      *error = handle_exception();                                             \
    return nullptr;                                                            \
  }

// Construction and destruction
Number *number_new() {
  try {
    return to_c(new ripple::Number());
  } catch (...) {
    return nullptr;
  }
}

Number *number_new_from_int64(int64_t mantissa, NumberError *error) {
  TRY_CATCH_RETURN_NULL(if (error) *error = NUMBER_SUCCESS;
                        return to_c(new ripple::Number(mantissa));)
}

Number *number_new_from_mantissa_exponent(int64_t mantissa, int exponent,
                                          NumberError *error) {
  TRY_CATCH_RETURN_NULL(if (error) *error = NUMBER_SUCCESS;
                        return to_c(new ripple::Number(mantissa, exponent));)
}

Number *number_new_from_string(const char *str, NumberError *error) {
  if (!str) {
    if (error)
      *error = NUMBER_ERROR_INVALID_ARGUMENT;
    return nullptr;
  }

  // Note: This is a basic implementation. The actual Number class doesn't have
  // a string constructor, so we'd need to implement string parsing
  // For now, we'll return an error
  if (error)
    *error = NUMBER_ERROR_INVALID_ARGUMENT;
  return nullptr;
}

Number *number_clone(const Number *num, NumberError *error) {
  if (!num) {
    if (error)
      *error = NUMBER_ERROR_INVALID_ARGUMENT;
    return nullptr;
  }

  TRY_CATCH_RETURN_NULL(if (error) *error = NUMBER_SUCCESS;
                        return to_c(new ripple::Number(*to_cpp(num)));)
}

void number_free(Number *num) { delete to_cpp(num); }

// Getters
int64_t number_get_mantissa(const Number *num) {
  if (!num)
    return 0;
  return to_cpp(num)->mantissa();
}

int number_get_exponent(const Number *num) {
  if (!num)
    return 0;
  return to_cpp(num)->exponent();
}

// Arithmetic operations
NumberError number_add(Number *result, const Number *lhs, const Number *rhs) {
  if (!result || !lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = *to_cpp(lhs) + *to_cpp(rhs);)
}

NumberError number_subtract(Number *result, const Number *lhs,
                            const Number *rhs) {
  if (!result || !lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = *to_cpp(lhs) - *to_cpp(rhs);)
}

NumberError number_multiply(Number *result, const Number *lhs,
                            const Number *rhs) {
  if (!result || !lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = *to_cpp(lhs) * *to_cpp(rhs);)
}

NumberError number_divide(Number *result, const Number *lhs,
                          const Number *rhs) {
  if (!result || !lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = *to_cpp(lhs) / *to_cpp(rhs);)
}

NumberError number_negate(Number *result, const Number *num) {
  if (!result || !num)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = -*to_cpp(num);)
}

NumberError number_abs(Number *result, const Number *num) {
  if (!result || !num)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = abs(*to_cpp(num));)
}

// In-place operations
NumberError number_add_assign(Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(lhs) += *to_cpp(rhs);)
}

NumberError number_subtract_assign(Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(lhs) -= *to_cpp(rhs);)
}

NumberError number_multiply_assign(Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(lhs) *= *to_cpp(rhs);)
}

NumberError number_divide_assign(Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(lhs) /= *to_cpp(rhs);)
}

// Comparison operations
bool number_equals(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return false;
  return *to_cpp(lhs) == *to_cpp(rhs);
}

bool number_not_equals(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return true;
  return *to_cpp(lhs) != *to_cpp(rhs);
}

bool number_less_than(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return false;
  return *to_cpp(lhs) < *to_cpp(rhs);
}

bool number_less_than_or_equal(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return false;
  return *to_cpp(lhs) <= *to_cpp(rhs);
}

bool number_greater_than(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return false;
  return *to_cpp(lhs) > *to_cpp(rhs);
}

bool number_greater_than_or_equal(const Number *lhs, const Number *rhs) {
  if (!lhs || !rhs)
    return false;
  return *to_cpp(lhs) >= *to_cpp(rhs);
}

// Conversion operations
NumberError number_to_int64(const Number *num, int64_t *result) {
  if (!num || !result)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*result = static_cast<int64_t>(*to_cpp(num));)
}

NumberError number_to_string(const Number *num, char *buffer,
                             size_t buffer_size) {
  if (!num || !buffer || buffer_size == 0)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  try {
    std::string str = to_string(*to_cpp(num));
    if (str.length() + 1 > buffer_size) {
      return NUMBER_ERROR_INVALID_ARGUMENT;
    }
    std::strcpy(buffer, str.c_str());
    return NUMBER_SUCCESS;
  } catch (...) {
    return handle_exception();
  }
}

size_t number_string_length(const Number *num) {
  if (!num)
    return 0;

  try {
    return to_string(*to_cpp(num)).length();
  } catch (...) {
    return 0;
  }
}

// Utility functions
int number_signum(const Number *num) {
  if (!num)
    return 0;
  return to_cpp(num)->signum();
}

bool number_is_zero(const Number *num) {
  if (!num)
    return true;
  return *to_cpp(num) == ripple::Number{};
}

// Rounding mode control
RoundingMode number_get_rounding_mode() {
  return static_cast<RoundingMode>(ripple::Number::getround());
}

RoundingMode number_set_rounding_mode(RoundingMode mode) {
  return static_cast<RoundingMode>(ripple::Number::setround(
      static_cast<ripple::Number::rounding_mode>(mode)));
}

// Mathematical functions
NumberError number_power_uint(Number *result, const Number *base,
                              unsigned int exponent) {
  if (!result || !base)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = power(*to_cpp(base), exponent);)
}

NumberError number_root(Number *result, const Number *value,
                        unsigned int degree) {
  if (!result || !value)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = root(*to_cpp(value), degree);)
}

NumberError number_sqrt(Number *result, const Number *value) {
  if (!result || !value)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = root2(*to_cpp(value));)
}

NumberError number_log10(Number *result, const Number *value) {
  if (!result || !value)
    return NUMBER_ERROR_INVALID_ARGUMENT;

  TRY_CATCH_RETURN_ERROR(*to_cpp(result) = lg(*to_cpp(value));)
}

// Constants
Number *number_min() {
  try {
    return to_c(new ripple::Number(ripple::Number::min()));
  } catch (...) {
    return nullptr;
  }
}

Number *number_max() {
  try {
    return to_c(new ripple::Number(ripple::Number::max()));
  } catch (...) {
    return nullptr;
  }
}

Number *number_lowest() {
  try {
    return to_c(new ripple::Number(ripple::Number::lowest()));
  } catch (...) {
    return nullptr;
  }
}
