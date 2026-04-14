/*
 * Case 02: Invalid Rust char value
 *
 * A valid Rust char must be a Unicode scalar value: a code point in
 * [0x0000, 0xD7FF] or [0xE000, 0x10FFFF].  Surrogate code points
 * (0xD800-0xDFFF) are never valid Rust chars.
 *
 * Assumed Rust-side binding: Rust reads the output as char or validates
 * it as a Unicode scalar value.
 */

#include <stdint.h>

/* Legal: 0x0041 is 'A', a valid Unicode scalar value. */
void
fill_char_legal(uint32_t *out) {
    *out = 0x0041; /* 'A' */
}

/* Illegal: 0xD800 is a high surrogate, not a valid Rust char. */
void
fill_char_illegal(uint32_t *out) {
    *out = 0xD800; /* surrogate, invalid Rust char */
}
