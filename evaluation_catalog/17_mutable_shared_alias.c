/*
 * Case 17: Mutable and shared alias to the same memory
 *
 * Rust's aliasing XOR mutability rule (AXM) requires that at any given
 * time, either one mutable reference OR any number of shared references
 * to a value exist — never both.  Producing a shared view (*dst) that
 * aliases the mutable input (src) violates AXM.
 *
 * Assumed Rust-side binding: Rust holds a mutable borrow of src and also
 * creates a shared borrow from *dst after this call returns.
 */

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* Legal: copies src into freshly allocated memory so that dst points to
 * a completely separate allocation — no aliasing with src. */
void
separate_output_legal(uint8_t *src, size_t len, const uint8_t **dst) {
    uint8_t *p = (uint8_t *)malloc(len);
    if (p == NULL) {
        *dst = NULL;
        return;
    }
    memcpy(p, src, len); /* independent copy; no aliasing */
    *dst = p;
}

/* Illegal: makes *dst point directly at src, creating a shared alias to
 * memory that Rust holds a mutable borrow over.  This violates AXM. */
void
in_place_alias_illegal(uint8_t *src, size_t len, const uint8_t **dst) {
    (void)len;
    *dst = src; /* dst aliases src — shared and mutable borrow coexist */
}
