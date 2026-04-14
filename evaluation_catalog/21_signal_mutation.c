/*
 * Case 21: Asynchronous mutation via a signal handler
 *
 * Signal handlers run asynchronously and can interrupt Rust code at any
 * point.  Mutating shared memory inside a signal handler that was
 * triggered after Rust may have validated or borrowed that memory creates
 * an asynchronous data race.
 *
 * Assumed Rust-side binding: Rust assumes the memory stays unchanged
 * except through explicit, coordinated writes.
 */

#include <signal.h>
#include <stdint.h>

/* Legal: performs a single synchronous mutation with no signal-handler
 * involvement. */
void
no_async_mutation_legal(uint8_t *p) {
    p[0] ^= 0x01; /* straightforward synchronous write */
}

/* Signal handler for the illegal variant — mutates the globally saved
 * pointer asynchronously. */
static uint8_t *global_ptr;

static void
handler(int signo) {
    (void)signo;
    global_ptr[0] = 0x42; /* asynchronous mutation of shared memory */
}

/* Illegal: saves the pointer in a global, installs a signal handler that
 * mutates it, and immediately raises the signal.  The handler fires
 * asynchronously and may race with Rust-side access. */
void
async_signal_mutation_illegal(uint8_t *p) {
    global_ptr = p;
    signal(SIGUSR1, handler); /* install async mutation handler */
    raise(SIGUSR1);           /* trigger it immediately (demo) */
}
