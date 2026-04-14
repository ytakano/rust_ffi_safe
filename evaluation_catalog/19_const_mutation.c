/*
 * Case 19: Mutation through a nominally immutable input
 *
 * When Rust passes a shared borrow (&[u8], &T, etc.) to C code, it
 * guarantees that the memory will not be mutated.  Writing through a
 * const pointer breaks that guarantee and violates Rust's AXM rule.
 *
 * Assumed Rust-side binding: Rust passed a shared borrow (e.g., &[u8]
 * or &T) and assumes the callee will not mutate it.
 */

#include <stddef.h>
#include <stdint.h>

/* Legal: only reads from the input; does not modify it. */
uint8_t
read_only_sum_legal(const uint8_t *p, size_t len) {
    size_t  i;
    uint8_t sum = 0;
    for (i = 0; i < len; i++) {
        sum = (uint8_t)(sum + p[i]);
    }
    return sum;
}

/* Illegal: casts away const and overwrites the first byte of the input.
 * This mutates memory that Rust holds a shared (immutable) borrow over. */
void
write_through_const_illegal(const uint8_t *p, size_t len) {
    if (len > 0) {
        ((uint8_t *)p)[0] = 0xFF; /* mutation through a shared borrow */
    }
}
