/*
 * Case 14: Success return without initializing the out-parameter
 *
 * When a function reports success, the Rust caller may immediately read
 * the out-parameter.  Leaving it uninitialized while returning a success
 * code means Rust reads uninitialized memory, which is undefined behavior.
 *
 * Assumed Rust-side binding: On success, Rust immediately reads the
 * out-parameter as a valid initialized value.
 */

#include <stdint.h>

/* Legal: writes a valid value to the out-parameter before returning
 * success. */
int
compute_value_legal(uint32_t *out) {
    *out = 1234; /* out-parameter initialized before success return */
    return 1;    /* success */
}

/* Illegal: returns success without initializing the out-parameter;
 * the caller will read uninitialized memory. */
int
compute_value_illegal(uint32_t *out) {
    (void)out;   /* out-parameter deliberately left uninitialized */
    return 1;    /* reports success — caller will read garbage */
}
