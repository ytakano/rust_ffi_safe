/*
 * Case 18: Two mutable aliases to the same memory
 *
 * Rust requires that mutable borrows are exclusive.  If two *mut pointers
 * both point to the same object, any write through one invalidates the
 * other under Rust's memory model.
 *
 * Assumed Rust-side binding: Rust treats the two output pointers as
 * distinct mutable borrows pointing to disjoint memory.
 */

#include <stdint.h>

/* Legal: each output pointer points to a different static variable. */
void
split_mut_legal(uint8_t **a, uint8_t **b) {
    static uint8_t x = 1;
    static uint8_t y = 2;
    *a = &x; /* distinct allocation */
    *b = &y; /* distinct allocation */
}

/* Illegal: both output pointers alias the same static variable,
 * creating two simultaneous mutable borrows of one object. */
void
split_mut_illegal(uint8_t **a, uint8_t **b) {
    static uint8_t x = 1;
    *a = &x; /* mutable alias 1 */
    *b = &x; /* mutable alias 2 — same object as *a */
}
