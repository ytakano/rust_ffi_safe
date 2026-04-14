/*
 * Case 25: Overlapping buffers where Rust assumes non-overlap
 *
 * memcpy (and Rust's copy_nonoverlapping) requires that source and
 * destination regions do not overlap.  Passing overlapping regions
 * violates the contract and is undefined behavior.
 *
 * Assumed Rust-side binding: The Rust wrapper assumes that the input and
 * output regions are disjoint (e.g., uses ptr::copy_nonoverlapping).
 */

#include <stddef.h>
#include <string.h>

/* Legal: source and destination are entirely separate buffers. */
void
copy_nonoverlap_legal(unsigned char *dst, const unsigned char *src, size_t len) {
    memcpy(dst, src, len); /* dst and src must not overlap */
}

/* Illegal: passes overlapping source and destination to memcpy.
 * buf+1 overlaps buf by (len-1) bytes, violating the non-overlap
 * contract. */
void
copy_nonoverlap_illegal(unsigned char *buf, size_t len) {
    memcpy(buf + 1, buf, len); /* overlapping source and destination */
}
