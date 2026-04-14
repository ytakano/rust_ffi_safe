/*
 * Case 03: Invalid enum discriminant
 *
 * A Rust #[repr(i32)] enum with variants 0, 1, and 2 may only hold
 * those three discriminant values.  Any other integer is undefined
 * behavior when Rust interprets the memory as that enum type.
 *
 * Assumed Rust-side binding: Rust interprets the output as a
 * #[repr(i32)] enum with valid discriminants 0, 1, and 2.
 */

#include <stdint.h>

/* Legal: discriminant 1 is within the valid set {0, 1, 2}. */
void
fill_status_legal(int32_t *out) {
    *out = 1;
}

/* Illegal: discriminant 99 is outside the valid set {0, 1, 2}. */
void
fill_status_illegal(int32_t *out) {
    *out = 99;
}
