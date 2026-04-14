/*
 * Case 01: Invalid boolean bit pattern
 *
 * Rust's bool has exactly two valid bit patterns: 0 (false) and 1 (true).
 * Writing any other byte value into memory that Rust will read as bool
 * is undefined behavior on the Rust side.
 *
 * Assumed Rust-side binding: Rust reads the output byte as bool.
 */

#include <stdint.h>

/* Legal: writes a valid bool value (1 == true). */
void
fill_bool_legal(uint8_t *out) {
    *out = 1;
}

/* Illegal: writes 2, which is not a valid Rust bool bit pattern. */
void
fill_bool_illegal(uint8_t *out) {
    *out = 2;
}
