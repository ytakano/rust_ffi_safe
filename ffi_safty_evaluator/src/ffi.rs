// FFI declarations for the evaluation_catalog C functions.
//
// Each extern "C" block corresponds to one catalog entry.  Rust structs are
// defined with the correct repr to match the C-side layout.

use std::os::raw::{c_char, c_int, c_uchar};

// ---------------------------------------------------------------------------
// Aggregate types used as return values by some catalog entries
// ---------------------------------------------------------------------------

/// Matches `struct ByteSlice { const uint8_t *ptr; size_t len; }` (case 10).
#[repr(C)]
pub struct ByteSlice {
    pub ptr: *const u8,
    pub len: usize,
}

/// Matches the naturally-aligned `struct Pair { uint32_t tag; uint64_t value; }` (case 15 legal).
#[repr(C)]
pub struct Pair {
    pub tag: u32,
    pub value: u64,
}

/// Matches the packed `struct PairPacked` with `#pragma pack(push,1)` (case 15 illegal).
#[repr(C, packed)]
pub struct PairPacked {
    pub tag: u32,
    pub value: u64,
}

/// Matches `struct Header { uint32_t tag; uint32_t len; }` (case 16 legal).
#[repr(C)]
pub struct Header {
    pub tag: u32,
    pub len: u32,
}

/// Matches `struct HeaderReversed { uint32_t len; uint32_t tag; }` (case 16 illegal).
#[repr(C)]
pub struct HeaderReversed {
    pub len: u32,
    pub tag: u32,
}

// ---------------------------------------------------------------------------
// extern "C" declarations
// ---------------------------------------------------------------------------

unsafe extern "C" {
    // --- Case 01: Invalid boolean bit pattern ---
    pub fn fill_bool_legal(out: *mut u8);
    pub fn fill_bool_illegal(out: *mut u8);

    // --- Case 02: Invalid Rust char value ---
    pub fn fill_char_legal(out: *mut u32);
    pub fn fill_char_illegal(out: *mut u32);

    // --- Case 03: Invalid enum discriminant ---
    pub fn fill_status_legal(out: *mut i32);
    pub fn fill_status_illegal(out: *mut i32);

    // --- Case 04: Null pointer for a non-null Rust type ---
    pub fn get_nonnull_ptr_legal() -> *mut c_int;
    pub fn get_nonnull_ptr_illegal() -> *mut c_int;

    // --- Case 05: Zero used for a non-zero Rust handle ---
    pub fn make_handle_legal() -> u32;
    pub fn make_handle_illegal() -> u32;

    // --- Case 06: Dangling heap pointer ---
    pub fn make_heap_ptr_legal() -> *mut c_int;
    pub fn make_heap_ptr_illegal() -> *mut c_int;

    // --- Case 07: Returning the address of a stack local ---
    pub fn return_static_ptr_legal() -> *mut c_int;
    pub fn return_stack_ptr_illegal() -> *mut c_int;

    // --- Case 08: Misaligned pointer ---
    pub fn pass_through_u32_ptr_legal(p: *const u32) -> *const u32;
    pub fn make_misaligned_ptr_illegal(buf: *const u8) -> *const u32;

    // --- Case 09: Out-of-bounds pointer ---
    pub fn get_in_bounds_ptr_legal() -> *const u8;
    pub fn get_out_of_bounds_ptr_illegal() -> *const u8;

    // --- Case 10: Slice length larger than the pointed allocation ---
    pub fn get_slice_legal() -> ByteSlice;
    pub fn get_slice_illegal() -> ByteSlice;

    // --- Case 11: Forged pointer from an arbitrary integer ---
    pub fn make_pointer_from_roundtrip_legal() -> *mut c_int;
    pub fn make_forged_pointer_illegal() -> *mut c_int;

    // --- Case 12: Borrowed input retained after the call ---
    pub fn observe_input_legal(p: *const u8, len: usize);
    pub fn observe_input_illegal(p: *const u8, len: usize);
    pub fn read_saved_illegal() -> u8;

    // --- Case 13: Ownership contract violation ---
    pub fn make_owned_string_legal() -> *mut c_char;
    pub fn make_owned_string_illegal() -> *mut c_char;

    // --- Case 14: Success return without initializing the out-parameter ---
    pub fn compute_value_legal(out: *mut u32) -> c_int;
    pub fn compute_value_illegal(out: *mut u32) -> c_int;

    // --- Case 15: Struct packing/alignment mismatch ---
    pub fn make_pair_legal() -> Pair;
    pub fn make_pair_illegal() -> PairPacked;

    // --- Case 16: Struct field order mismatch ---
    pub fn make_header_legal() -> Header;
    pub fn make_header_illegal() -> HeaderReversed;

    // --- Case 17: Mutable and shared alias to the same memory ---
    pub fn separate_output_legal(src: *mut u8, len: usize, dst: *mut *const u8);
    pub fn in_place_alias_illegal(src: *mut u8, len: usize, dst: *mut *const u8);

    // --- Case 18: Two mutable aliases to the same memory ---
    pub fn split_mut_legal(a: *mut *mut u8, b: *mut *mut u8);
    pub fn split_mut_illegal(a: *mut *mut u8, b: *mut *mut u8);

    // --- Case 19: Mutation through a nominally immutable input ---
    pub fn read_only_sum_legal(p: *const u8, len: usize) -> u8;
    pub fn write_through_const_illegal(p: *const u8, len: usize);

    // --- Case 20: Concurrent background mutation after the call returns ---
    pub fn no_background_mutation_legal(p: *mut u8);
    pub fn start_background_mutation_illegal(p: *mut u8);

    // --- Case 21: Asynchronous mutation via a signal handler ---
    pub fn no_async_mutation_legal(p: *mut u8);
    pub fn async_signal_mutation_illegal(p: *mut u8);

    // --- Case 22: longjmp across the FFI boundary ---
    pub fn foreign_call_legal() -> c_int;
    pub fn set_env_for_demo();
    pub fn foreign_call_illegal();

    // --- Case 23: Invalid UTF-8 for a Rust str ---
    pub fn get_utf8_legal() -> *const c_char;
    pub fn get_utf8_illegal() -> *const c_char;

    // --- Case 24: Uninitialized scalar value ---
    pub fn get_initialized_u32_legal() -> u32;
    pub fn get_uninitialized_u32_illegal() -> u32;

    // --- Case 25: Overlapping buffers where Rust assumes non-overlap ---
    pub fn copy_nonoverlap_legal(dst: *mut c_uchar, src: *const c_uchar, len: usize);
    pub fn copy_nonoverlap_illegal(buf: *mut c_uchar, len: usize);

    // --- Case 26: Pointer-sized integer used with stale provenance ---
    pub fn export_live_pointer_legal() -> usize;
    pub fn export_stale_pointer_illegal() -> usize;
}
