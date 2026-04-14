/*
 * Case 11: Forged pointer from an arbitrary integer
 *
 * A pointer cast from an arbitrary integer value does not carry
 * provenance to any live allocation.  Rust cannot safely dereference
 * such a pointer; doing so is undefined behavior.
 *
 * Assumed Rust-side binding: Rust treats the returned pointer as valid
 * memory with a live allocation behind it.
 */

#include <stdint.h>

/* Legal: round-trips a real pointer through uintptr_t and back,
 * preserving provenance to the original static allocation. */
int *
make_pointer_from_roundtrip_legal(void) {
    static int value = 42;
    uintptr_t raw = (uintptr_t)&value; /* preserve provenance via round-trip */
    return (int *)raw;
}

/* Illegal: manufactures a pointer from the literal integer 0x1, which
 * has no associated allocation or provenance. */
int *
make_forged_pointer_illegal(void) {
    return (int *)(uintptr_t)0x1; /* fabricated address, no live allocation */
}
