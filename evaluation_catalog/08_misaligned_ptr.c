/*
 * Case 08: Misaligned pointer
 *
 * uint32_t requires 4-byte alignment on virtually all targets.  Creating
 * a reference or dereferencing a pointer that is not properly aligned is
 * undefined behavior in Rust.
 *
 * Assumed Rust-side binding: Rust dereferences the returned pointer as
 * *const u32 or creates &u32 from it.
 */

#include <stdint.h>

/* Legal: returns the caller's pointer unchanged (alignment is the caller's
 * responsibility, which is the expected contract). */
const uint32_t *
pass_through_u32_ptr_legal(const uint32_t *p) {
    return p;
}

/* Illegal: advances a byte pointer by 1, producing a pointer misaligned
 * for uint32_t (alignment requirement: 4 bytes). */
const uint32_t *
make_misaligned_ptr_illegal(const uint8_t *buf) {
    return (const uint32_t *)(buf + 1); /* offset +1 breaks 4-byte alignment */
}
