/*
 * Case 10: Slice length larger than the pointed allocation
 *
 * Rust reconstructs a slice as (ptr, len).  The len field must not exceed
 * the number of valid bytes starting at ptr.  Overstating the length
 * allows Rust to read or write memory outside the actual allocation.
 *
 * Assumed Rust-side binding: Rust reconstructs a slice from the returned
 * (ptr, len) pair.
 */

#include <stddef.h>
#include <stdint.h>

struct ByteSlice {
    const uint8_t *ptr;
    size_t         len;
};

/* Legal: length matches the actual allocation size (4 bytes). */
struct ByteSlice
get_slice_legal(void) {
    static uint8_t data[4] = {1, 2, 3, 4};
    return (struct ByteSlice){ data, 4 };
}

/* Illegal: claims 1024 bytes are accessible but the array is only 4 bytes. */
struct ByteSlice
get_slice_illegal(void) {
    static uint8_t data[4] = {1, 2, 3, 4};
    return (struct ByteSlice){ data, 1024 }; /* length vastly overstated */
}
