/*
 * Case 15: Struct packing/alignment mismatch
 *
 * Rust's #[repr(C)] struct Pair { tag: u32, value: u64 } uses the
 * natural C layout: 4-byte tag, 4-byte padding, 8-byte value (total 16
 * bytes on most targets).  Using #pragma pack(push,1) removes the
 * padding, producing a 12-byte layout that mismatches Rust's expectation.
 *
 * Assumed Rust-side binding: Rust expects the natural (unpadded) C layout
 * of struct Pair.
 *
 * Note: The illegal variant uses a distinct C name (struct PairPacked) to
 * avoid a redefinition error in this translation unit.  The mismatch is
 * the layout, not the name.
 */

#include <stdint.h>

/* --- Legal variant: natural C layout --- */

struct Pair {
    uint32_t tag;
    uint64_t value;
};

struct Pair
make_pair_legal(void) {
    return (struct Pair){ 1, 0x1122334455667788ULL };
}

/* --- Illegal variant: packed layout (no padding between fields) ---
 *
 * struct PairPacked represents the same logical type as struct Pair but
 * with #pragma pack(push,1), which removes internal padding.  When Rust
 * maps this as its naturally-aligned repr(C) Pair, field offsets will be
 * wrong and reads will be misaligned or read incorrect bytes.
 */

#pragma pack(push, 1)
struct PairPacked {
    uint32_t tag;
    uint64_t value;
};
#pragma pack(pop)

struct PairPacked
make_pair_illegal(void) {
    return (struct PairPacked){ 1, 0x1122334455667788ULL };
}
