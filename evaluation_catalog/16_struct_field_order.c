/*
 * Case 16: Struct field order mismatch
 *
 * Rust's #[repr(C)] struct Header { tag: u32, len: u32 } lays out tag
 * at offset 0 and len at offset 4.  Swapping the field order on the C
 * side places len at offset 0 and tag at offset 4, so Rust reads the
 * fields with their bytes swapped.
 *
 * Assumed Rust-side binding: Rust expects
 *   struct Header { uint32_t tag; uint32_t len; }
 * (tag first, len second).
 *
 * Note: The illegal variant uses a distinct C name (struct HeaderReversed)
 * to avoid a redefinition error in this translation unit.  The mismatch
 * is the field order, not the name.
 */

#include <stdint.h>

/* --- Legal variant: field order matches Rust's expectation --- */

struct Header {
    uint32_t tag;
    uint32_t len;
};

struct Header
make_header_legal(void) {
    return (struct Header){ 7, 16 }; /* tag=7, len=16 */
}

/* --- Illegal variant: field order reversed relative to Rust's expectation ---
 *
 * struct HeaderReversed represents the same logical type as struct Header
 * but with the fields in the opposite order.  When Rust maps this as
 * Header { tag, len }, it will read len as tag and tag as len.
 */

struct HeaderReversed {
    uint32_t len; /* at offset 0 — Rust reads this as tag */
    uint32_t tag; /* at offset 4 — Rust reads this as len */
};

struct HeaderReversed
make_header_illegal(void) {
    return (struct HeaderReversed){ 16, 7 }; /* len=16, tag=7 (swapped) */
}
