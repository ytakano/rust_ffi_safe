/*
 * Case 05: Zero used for a non-zero Rust handle
 *
 * core::num::NonZeroU32 (and similar types) store their niche-optimization
 * guarantee by treating 0 as an illegal bit pattern.  Returning 0 for a
 * handle typed as NonZeroU32 on the Rust side is undefined behavior.
 *
 * Assumed Rust-side binding: Rust binds the return value as
 * core::num::NonZeroU32.
 */

#include <stdint.h>

/* Legal: returns a non-zero handle value. */
uint32_t
make_handle_legal(void) {
    return 1;
}

/* Illegal: returns 0, which is not a valid NonZeroU32 bit pattern. */
uint32_t
make_handle_illegal(void) {
    return 0;
}
