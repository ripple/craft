#ifndef XRPLD_NUMBER_C_H
#define XRPLD_NUMBER_C_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Opaque pointer to the C++ Number object
typedef struct Number Number;

// Error codes
typedef enum {
    NUMBER_SUCCESS = 0,
    NUMBER_ERROR_OVERFLOW = 1,
    NUMBER_ERROR_DIVIDE_BY_ZERO = 2,
    NUMBER_ERROR_INVALID_ARGUMENT = 3,
    NUMBER_ERROR_OUT_OF_MEMORY = 4,
    NUMBER_ERROR_UNKNOWN = 5
} NumberError;

// Rounding modes
typedef enum {
    ROUNDING_TO_NEAREST = 0,
    ROUNDING_TOWARDS_ZERO = 1,
    ROUNDING_DOWNWARD = 2,
    ROUNDING_UPWARD = 3
} RoundingMode;

// Construction and destruction
Number* number_new();
Number* number_new_from_int64(int64_t mantissa, NumberError* error);
Number* number_new_from_mantissa_exponent(int64_t mantissa, int exponent, NumberError* error);
Number* number_new_from_string(const char* str, NumberError* error);
Number* number_clone(const Number* num, NumberError* error);
void number_free(Number* num);

// Getters
int64_t number_get_mantissa(const Number* num);
int number_get_exponent(const Number* num);

// Arithmetic operations
NumberError number_add(Number* result, const Number* lhs, const Number* rhs);
NumberError number_subtract(Number* result, const Number* lhs, const Number* rhs);
NumberError number_multiply(Number* result, const Number* lhs, const Number* rhs);
NumberError number_divide(Number* result, const Number* lhs, const Number* rhs);
NumberError number_negate(Number* result, const Number* num);
NumberError number_abs(Number* result, const Number* num);

// In-place operations
NumberError number_add_assign(Number* lhs, const Number* rhs);
NumberError number_subtract_assign(Number* lhs, const Number* rhs);
NumberError number_multiply_assign(Number* lhs, const Number* rhs);
NumberError number_divide_assign(Number* lhs, const Number* rhs);

// Comparison operations
bool number_equals(const Number* lhs, const Number* rhs);
bool number_not_equals(const Number* lhs, const Number* rhs);
bool number_less_than(const Number* lhs, const Number* rhs);
bool number_less_than_or_equal(const Number* lhs, const Number* rhs);
bool number_greater_than(const Number* lhs, const Number* rhs);
bool number_greater_than_or_equal(const Number* lhs, const Number* rhs);

// Conversion operations
NumberError number_to_int64(const Number* num, int64_t* result);
NumberError number_to_string(const Number* num, char* buffer, size_t buffer_size);
size_t number_string_length(const Number* num);

// Utility functions
int number_signum(const Number* num);
bool number_is_zero(const Number* num);

// Rounding mode control
RoundingMode number_get_rounding_mode();
RoundingMode number_set_rounding_mode(RoundingMode mode);

// Mathematical functions
NumberError number_power_uint(Number* result, const Number* base, unsigned int exponent);
NumberError number_root(Number* result, const Number* value, unsigned int degree);
NumberError number_sqrt(Number* result, const Number* value);
NumberError number_log10(Number* result, const Number* value);

// Constants
Number* number_min();
Number* number_max();
Number* number_lowest();

#ifdef __cplusplus
}
#endif

#endif // XRPLD_NUMBER_C_H
