/*
 * Case 24: Uninitialized scalar value
 *
 * Reading an uninitialized scalar in Rust is undefined behavior regardless
 * of the bit pattern.  Even if the C compiler happens to place a plausible
 * value there, the memory model treats the value as "uninitialized" and
 * Rust may rely on that to optimize in ways that break the program.
 *
 * Assumed Rust-side binding: Rust reads the return value immediately as a
 * valid initialized u32.
 */

#include <stdint.h>

/* Legal: explicitly initializes x before returning it. */
uint32_t
get_initialized_u32_legal(void) {
    uint32_t x = 1234;
    return x;
}

/*
 * Illegal: returns x without ever initializing it.
 * The compiler correctly warns about this; the warning is suppressed here
 * because this is an intentional catalog entry for undefined behavior
 * demonstration purposes.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wuninitialized"
uint32_t
get_uninitialized_u32_illegal(void) {
    uint32_t x; /* deliberately left uninitialized */
    return x;
}
#pragma GCC diagnostic pop
