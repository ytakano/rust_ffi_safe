/*
 * Case 04: Null pointer for a non-null Rust type
 *
 * Types such as NonNull<i32>, &i32, and &mut i32 carry a language-level
 * guarantee that the pointer is never null.  Returning NULL from a
 * function whose Rust binding uses one of these types is undefined
 * behavior on the Rust side.
 *
 * Assumed Rust-side binding: Rust binds the return value as NonNull<i32>,
 * &i32, or &mut i32 (any non-null pointer contract).
 */

#include <stddef.h>

/* Legal: returns a pointer to a valid static object (never null). */
static int value = 42;

int *
get_nonnull_ptr_legal(void) {
    return &value;
}

/* Illegal: returns NULL, violating the non-null pointer contract. */
int *
get_nonnull_ptr_illegal(void) {
    return NULL;
}
