/*
 * Case 20: Concurrent background mutation after the call returns
 *
 * After the FFI call returns, Rust may validate or borrow the memory
 * under the assumption that no uncoordinated concurrent mutation is
 * occurring.  Spawning a background thread that continues writing to
 * shared memory violates that assumption and is a data race.
 *
 * Assumed Rust-side binding: Rust may validate or borrow the memory
 * after the function returns, assuming no concurrent mutation.
 *
 * Note: case 20 uses pthreads.  When linking a final executable or shared
 * library that includes this translation unit, add -lpthread (or -pthread)
 * to the linker flags.
 */

/* _DEFAULT_SOURCE exposes POSIX extensions such as usleep() under -std=c11. */
#define _DEFAULT_SOURCE
#include <pthread.h>
#include <stdint.h>
#include <unistd.h>

/* Legal: performs a single synchronous mutation and returns with no
 * ongoing background activity. */
void
no_background_mutation_legal(uint8_t *p) {
    p[0] ^= 0x01; /* mutation completes before this function returns */
}

/* Background thread function for the illegal variant. */
static void *
writer_thread(void *arg) {
    uint8_t *p = (uint8_t *)arg;
    usleep(1000);      /* short delay, then mutate */
    p[0] ^= 0xFF;      /* data race: mutates memory concurrently with Rust */
    return NULL;
}

/* Illegal: detaches a background thread that mutates *p after this
 * function returns.  The mutation races with any Rust-side access. */
void
start_background_mutation_illegal(uint8_t *p) {
    pthread_t t;
    pthread_create(&t, NULL, writer_thread, p);
    pthread_detach(t); /* thread continues running after this call returns */
}
