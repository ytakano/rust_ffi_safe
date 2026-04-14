/*
 * Case 26: Pointer-sized integer used with the wrong provenance/ownership meaning
 *
 * A uintptr_t that was obtained from a pointer after the underlying
 * allocation has been freed no longer carries valid provenance.  When
 * Rust casts it back to a pointer and dereferences it, the result is
 * use-after-free undefined behavior.
 *
 * Assumed Rust-side binding: Rust interprets the returned integer as a
 * live pointer or trusted handle to currently valid memory.
 */

#include <stdint.h>
#include <stdlib.h>

/* Legal: exports the address of a static variable; the provenance is
 * always valid for the lifetime of the program. */
uintptr_t
export_live_pointer_legal(void) {
    static int value = 42;
    return (uintptr_t)&value; /* provenance of a live static object */
}

/* Illegal: allocates memory, converts the pointer to uintptr_t, frees
 * the allocation, and then returns the now-stale address.  Any attempt
 * by Rust to use it as a pointer is use-after-free. */
uintptr_t
export_stale_pointer_illegal(void) {
    int *p = (int *)malloc(sizeof(int));
    if (p == NULL) {
        return 0;
    }
    *p = 42;
    uintptr_t raw = (uintptr_t)p;
    free(p);   /* allocation freed — raw is now a stale pointer value */
    return raw; /* provenance is invalid; dereferencing is UB */
}
