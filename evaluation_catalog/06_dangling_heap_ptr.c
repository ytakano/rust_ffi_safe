/*
 * Case 06: Dangling heap pointer
 *
 * A pointer that has been freed no longer refers to a valid live
 * allocation.  Dereferencing such a pointer on the Rust side causes
 * use-after-free undefined behavior.
 *
 * Assumed Rust-side binding: Rust treats the returned pointer as live
 * and dereferenceable.
 */

#include <stdlib.h>

/* Legal: allocates memory, writes a value, and returns the live pointer. */
int *
make_heap_ptr_legal(void) {
    int *p = (int *)malloc(sizeof(int));
    if (p == NULL) {
        return NULL;
    }
    *p = 42;
    return p;
}

/*
 * Illegal: frees the allocation before returning the pointer (use-after-free).
 * The compiler correctly warns about this; the warning is suppressed here
 * because this is an intentional catalog entry for undefined behavior
 * demonstration purposes.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wuse-after-free"
int *
make_heap_ptr_illegal(void) {
    int *p = (int *)malloc(sizeof(int));
    if (p == NULL) {
        return NULL;
    }
    *p = 42;
    free(p);   /* pointer becomes dangling here */
    return p;  /* returning a dangling pointer */
}
#pragma GCC diagnostic pop
