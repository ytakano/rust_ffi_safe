/*
 * Case 07: Returning the address of a stack local
 *
 * A pointer to a stack-allocated variable becomes invalid as soon as
 * the function returns.  Dereferencing it on the Rust side is
 * use-after-stack-frame undefined behavior.
 *
 * Assumed Rust-side binding: Rust dereferences the returned pointer
 * after the call completes.
 */

/* Legal: returns the address of a static (program-lifetime) variable. */
static int value = 42;

int *
return_static_ptr_legal(void) {
    return &value;
}

/*
 * Illegal: returns the address of a local (stack) variable.
 * The pointer is dangling as soon as the function returns.
 *
 * The compiler correctly warns about this pattern; the warning is
 * suppressed here because this is an intentional catalog entry for
 * undefined behavior demonstration purposes.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-local-addr"
int *
return_stack_ptr_illegal(void) {
    int local_value = 42; /* lives on the stack */
    return &local_value;  /* dangling after return */
}
#pragma GCC diagnostic pop
