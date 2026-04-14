/*
 * Case 22: longjmp across the FFI boundary
 *
 * longjmp unwinds the call stack non-locally, bypassing normal function
 * returns.  When called from C code invoked via FFI, it skips Rust frames
 * without running their destructors, corrupts Rust's execution model, and
 * is unconditionally undefined behavior.
 *
 * Assumed Rust-side binding: Rust expects the foreign function to return
 * normally and not bypass Rust stack frames.
 */

#include <setjmp.h>

/* Legal: returns normally with an error code rather than performing a
 * non-local jump. */
int
foreign_call_legal(void) {
    return -1; /* ordinary error return */
}

/* --- Illegal variant setup ---
 *
 * set_env_for_demo() primes the jmp_buf so that the demo compiles and
 * links without crashing immediately.  In a real scenario the jmp_buf
 * would be set in a calling Rust frame, and longjmp would jump over it.
 */
static jmp_buf env;

void
set_env_for_demo(void) {
    (void)setjmp(env); /* prime the jmp_buf for demonstration */
}

/* Illegal: performs a non-local jump instead of returning normally.
 * Any Rust frames between the setjmp site and this call are bypassed
 * without running their destructors. */
void
foreign_call_illegal(void) {
    longjmp(env, 1); /* non-local exit — bypasses Rust stack frames */
}
