/*
 * Case 13: Function claims to return owned heap memory but returns static storage
 *
 * If the ownership contract says the caller must free the returned pointer
 * via the C-side allocator (e.g., free()), the pointer must come from
 * malloc/calloc/realloc.  Returning a static buffer and then freeing it
 * on the Rust side is undefined behavior.
 *
 * Assumed Rust-side binding: Rust treats the returned pointer as owned
 * heap memory that must later be freed through the C-side deallocator.
 */

#include <stdlib.h>
#include <string.h>

/* Legal: allocates heap memory and returns an owned pointer.
 * The caller is responsible for calling free() on the result. */
char *
make_owned_string_legal(void) {
    char *p = (char *)malloc(6);
    if (p == NULL) {
        return NULL;
    }
    memcpy(p, "hello", 6); /* includes the null terminator */
    return p;
}

/* Illegal: returns a pointer to static storage.
 * Calling free() on a static buffer is undefined behavior. */
char *
make_owned_string_illegal(void) {
    static char s[] = "hello"; /* static storage, not heap-allocated */
    return s;                  /* must not be passed to free() */
}
