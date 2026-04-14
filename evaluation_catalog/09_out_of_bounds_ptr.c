/*
 * Case 09: Out-of-bounds pointer
 *
 * A pointer must point within a valid allocation (or one-past-the-end
 * for comparison only).  Returning a pointer that lies entirely outside
 * the allocation means Rust cannot safely dereference it.
 *
 * Assumed Rust-side binding: Rust assumes the pointer points into a
 * valid object or slice.
 */

#include <stdint.h>

/* Legal: returns a pointer to the first element of a valid 4-byte array. */
const uint8_t *
get_in_bounds_ptr_legal(void) {
    static uint8_t data[4] = {1, 2, 3, 4};
    return &data[0];
}

/* Illegal: returns a pointer 8 bytes past the start of a 4-byte array,
 * which is well outside the allocation. */
const uint8_t *
get_out_of_bounds_ptr_illegal(void) {
    static uint8_t data[4] = {1, 2, 3, 4};
    return data + 8; /* 8 bytes past start; array is only 4 bytes */
}
