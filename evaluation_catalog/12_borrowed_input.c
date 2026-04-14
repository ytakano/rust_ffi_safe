/*
 * Case 12: Borrowed input retained after the call
 *
 * When Rust passes a pointer valid only for the duration of the call,
 * the callee must not store that pointer for later use.  Accessing the
 * stored pointer after the call returns may read freed or reused memory.
 *
 * Assumed Rust-side binding: Rust passes a pointer valid only for the
 * duration of the call (a temporary borrow).
 */

#include <stddef.h>
#include <stdint.h>

/* Legal: observes the input within the call and discards it on return. */
void
observe_input_legal(const uint8_t *p, size_t len) {
    (void)p;   /* read within the call if needed, then discard */
    (void)len;
}

/* Illegal: saves the pointer in a static variable and reads it later,
 * after the Rust-side borrow may have expired. */
static const uint8_t *saved_ptr;
static size_t         saved_len;

void
observe_input_illegal(const uint8_t *p, size_t len) {
    saved_ptr = p;   /* retaining a borrow beyond the call lifetime */
    saved_len = len;
}

/* Accessing saved_ptr after the original borrow has expired is UB on
 * the Rust side; the pointer may be dangling. */
uint8_t
read_saved_illegal(void) {
    return saved_ptr[0]; /* use-after-borrow */
}
