pub mod ffi;

#[cfg(test)]
mod tests {
    // -----------------------------------------------------------------------
    // Case 01: Invalid boolean bit pattern
    // -----------------------------------------------------------------------

    #[test]
    fn fill_bool_legal() {
        let mut out: u8 = 0;
        unsafe { super::ffi::fill_bool_legal(&mut out) };
        assert_eq!(out, 1);
    }

    #[test]
    fn fill_bool_illegal() {
        // Writes 2 — valid from C's view, invalid as a Rust bool bit pattern.
        let mut out: u8 = 0;
        unsafe { super::ffi::fill_bool_illegal(&mut out) };
        assert_eq!(out, 2);
    }

    // -----------------------------------------------------------------------
    // Case 02: Invalid Rust char value
    // -----------------------------------------------------------------------

    #[test]
    fn fill_char_legal() {
        let mut out: u32 = 0;
        unsafe { super::ffi::fill_char_legal(&mut out) };
        assert_eq!(out, 0x0041); // 'A'
    }

    #[test]
    fn fill_char_illegal() {
        // Writes 0xD800 — a surrogate, which is not a valid Rust char.
        let mut out: u32 = 0;
        unsafe { super::ffi::fill_char_illegal(&mut out) };
        assert_eq!(out, 0xD800);
    }

    // -----------------------------------------------------------------------
    // Case 03: Invalid enum discriminant
    // -----------------------------------------------------------------------

    #[test]
    fn fill_status_legal() {
        let mut out: i32 = 0;
        unsafe { super::ffi::fill_status_legal(&mut out) };
        assert_eq!(out, 1);
    }

    #[test]
    fn fill_status_illegal() {
        // Writes 99 — outside the valid discriminant set {0, 1, 2}.
        let mut out: i32 = 0;
        unsafe { super::ffi::fill_status_illegal(&mut out) };
        assert_eq!(out, 99);
    }

    // -----------------------------------------------------------------------
    // Case 04: Null pointer for a non-null Rust type
    // -----------------------------------------------------------------------

    #[test]
    fn get_nonnull_ptr_legal() {
        let p = unsafe { super::ffi::get_nonnull_ptr_legal() };
        assert!(!p.is_null());
        assert_eq!(unsafe { *p }, 42);
    }

    #[test]
    fn get_nonnull_ptr_illegal() {
        // Returns NULL — violation of a NonNull contract on the Rust side.
        let p = unsafe { super::ffi::get_nonnull_ptr_illegal() };
        assert!(p.is_null());
    }

    // -----------------------------------------------------------------------
    // Case 05: Zero used for a non-zero Rust handle
    // -----------------------------------------------------------------------

    #[test]
    fn make_handle_legal() {
        let h = unsafe { super::ffi::make_handle_legal() };
        assert_ne!(h, 0);
    }

    #[test]
    fn make_handle_illegal() {
        // Returns 0 — invalid bit pattern for NonZeroU32.
        let h = unsafe { super::ffi::make_handle_illegal() };
        assert_eq!(h, 0);
    }

    // -----------------------------------------------------------------------
    // Case 06: Dangling heap pointer
    // -----------------------------------------------------------------------

    #[test]
    fn make_heap_ptr_legal() {
        let p = unsafe { super::ffi::make_heap_ptr_legal() };
        assert!(!p.is_null());
        // The pointer is live; read and then leak it (acceptable in tests).
        assert_eq!(unsafe { *p }, 42);
        // Ownership is transferred to the caller; leak intentionally here.
        let _ = p;
    }

    #[test]
    fn make_heap_ptr_illegal() {
        // Returns a dangling pointer (freed before return).
        // Do NOT dereference — the violation is observable on the Rust side
        // only when the pointer is used as a live reference.
        let p = unsafe { super::ffi::make_heap_ptr_illegal() };
        let _ = p; // just verify the FFI call completes
    }

    // -----------------------------------------------------------------------
    // Case 07: Returning the address of a stack local
    // -----------------------------------------------------------------------

    #[test]
    fn return_static_ptr_legal() {
        let p = unsafe { super::ffi::return_static_ptr_legal() };
        assert!(!p.is_null());
        assert_eq!(unsafe { *p }, 42);
    }

    #[test]
    fn return_stack_ptr_illegal() {
        // Returns a dangling stack pointer.
        // Do NOT dereference — the stack frame is already gone.
        let p = unsafe { super::ffi::return_stack_ptr_illegal() };
        let _ = p;
    }

    // -----------------------------------------------------------------------
    // Case 08: Misaligned pointer
    // -----------------------------------------------------------------------

    #[test]
    fn pass_through_u32_ptr_legal() {
        let value: u32 = 0xDEAD_BEEF;
        let p = unsafe { super::ffi::pass_through_u32_ptr_legal(&value) };
        assert_eq!(p, &value as *const u32);
        assert_eq!(unsafe { *p }, 0xDEAD_BEEF);
    }

    #[test]
    fn make_misaligned_ptr_illegal() {
        // Returns a *const u32 at a 1-byte offset — misaligned.
        // Do NOT dereference; that would be misaligned access UB.
        let buf: [u8; 8] = [0u8; 8];
        let p = unsafe { super::ffi::make_misaligned_ptr_illegal(buf.as_ptr()) };
        // Verify the raw address is indeed misaligned for u32.
        assert_ne!((p as usize) % std::mem::align_of::<u32>(), 0);
    }

    // -----------------------------------------------------------------------
    // Case 09: Out-of-bounds pointer
    // -----------------------------------------------------------------------

    #[test]
    fn get_in_bounds_ptr_legal() {
        let p = unsafe { super::ffi::get_in_bounds_ptr_legal() };
        assert!(!p.is_null());
        assert_eq!(unsafe { *p }, 1); // data[0] == 1
    }

    #[test]
    fn get_out_of_bounds_ptr_illegal() {
        // Returns a pointer 8 bytes past a 4-byte array.
        // Do NOT dereference — it is outside the allocation.
        let p = unsafe { super::ffi::get_out_of_bounds_ptr_illegal() };
        let _ = p;
    }

    // -----------------------------------------------------------------------
    // Case 10: Slice length larger than the pointed allocation
    // -----------------------------------------------------------------------

    #[test]
    fn get_slice_legal() {
        let s = unsafe { super::ffi::get_slice_legal() };
        assert!(!s.ptr.is_null());
        assert_eq!(s.len, 4);
        // Safe to read all 4 bytes.
        let bytes = unsafe { std::slice::from_raw_parts(s.ptr, s.len) };
        assert_eq!(bytes, &[1, 2, 3, 4]);
    }

    #[test]
    fn get_slice_illegal() {
        // len is overstated (1024 vs 4 actual bytes).
        // Do NOT call from_raw_parts — that would create an out-of-bounds slice.
        let s = unsafe { super::ffi::get_slice_illegal() };
        assert!(!s.ptr.is_null());
        assert_eq!(s.len, 1024); // confirms the contract violation
    }

    // -----------------------------------------------------------------------
    // Case 11: Forged pointer from an arbitrary integer
    // -----------------------------------------------------------------------

    #[test]
    fn make_pointer_from_roundtrip_legal() {
        let p = unsafe { super::ffi::make_pointer_from_roundtrip_legal() };
        assert!(!p.is_null());
        assert_eq!(unsafe { *p }, 42);
    }

    #[test]
    fn make_forged_pointer_illegal() {
        // Returns a pointer cast from the literal 0x1 — no live allocation.
        // Do NOT dereference.
        let p = unsafe { super::ffi::make_forged_pointer_illegal() };
        assert_eq!(p as usize, 0x1);
    }

    // -----------------------------------------------------------------------
    // Case 12: Borrowed input retained after the call
    // -----------------------------------------------------------------------

    #[test]
    fn observe_input_legal() {
        let data: [u8; 4] = [10, 20, 30, 40];
        // Legal: callee only observes the data within the call.
        unsafe { super::ffi::observe_input_legal(data.as_ptr(), data.len()) };
    }

    #[test]
    fn observe_input_illegal() {
        let data: [u8; 4] = [10, 20, 30, 40];
        // Illegal: callee saves the pointer for later use.
        unsafe { super::ffi::observe_input_illegal(data.as_ptr(), data.len()) };
        // data is still live here, so read_saved_illegal is safe to call now.
    }

    #[test]
    fn read_saved_illegal() {
        // read_saved_illegal reads from the pointer saved by observe_input_illegal.
        // We must ensure observe_input_illegal was called first with live data.
        let data: [u8; 4] = [10, 20, 30, 40];
        unsafe {
            super::ffi::observe_input_illegal(data.as_ptr(), data.len());
            // data is still live; call while the borrow is technically valid.
            let v = super::ffi::read_saved_illegal();
            assert_eq!(v, data[0]);
        }
    }

    // -----------------------------------------------------------------------
    // Case 13: Ownership contract violation
    // -----------------------------------------------------------------------

    #[test]
    fn make_owned_string_legal() {
        // Returns heap-allocated memory; must be freed via the C allocator.
        let p = unsafe { super::ffi::make_owned_string_legal() };
        assert!(!p.is_null());
        // Verify content.
        let s = unsafe { std::ffi::CStr::from_ptr(p) };
        assert_eq!(s.to_str().unwrap(), "hello");
        // Free via the C allocator to honour the ownership contract.
        unsafe { free(p as *mut std::ffi::c_void) };
    }

    #[test]
    fn make_owned_string_illegal() {
        // Returns static storage — must NOT be freed.
        let p = unsafe { super::ffi::make_owned_string_illegal() };
        assert!(!p.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(p) };
        assert_eq!(s.to_str().unwrap(), "hello");
        // Do NOT call free(p); that would be UB (static storage, not heap).
    }

    // Helper: call the C standard library free() for the owned-string test.
    unsafe extern "C" {
        fn free(ptr: *mut std::ffi::c_void);
    }

    // -----------------------------------------------------------------------
    // Case 14: Success return without initializing the out-parameter
    // -----------------------------------------------------------------------

    #[test]
    fn compute_value_legal() {
        let mut out: u32 = 0;
        let rc = unsafe { super::ffi::compute_value_legal(&mut out) };
        assert_eq!(rc, 1); // success
        assert_eq!(out, 1234);
    }

    #[test]
    fn compute_value_illegal() {
        // Reports success but leaves out uninitialized.
        // We cannot safely read out after this call on the Rust side.
        let mut out: u32 = 0xDEAD_BEEF; // sentinel
        let rc = unsafe { super::ffi::compute_value_illegal(&mut out) };
        assert_eq!(rc, 1); // confirms success was reported
        // out may or may not have been written; do not rely on its value.
    }

    // -----------------------------------------------------------------------
    // Case 15: Struct packing/alignment mismatch
    // -----------------------------------------------------------------------

    #[test]
    fn make_pair_legal() {
        let p = unsafe { super::ffi::make_pair_legal() };
        assert_eq!(p.tag, 1);
        assert_eq!(p.value, 0x1122334455667788u64);
    }

    #[test]
    fn make_pair_illegal() {
        // Returns a packed struct; Rust reads it with repr(C, packed).
        let p = unsafe { super::ffi::make_pair_illegal() };
        assert_eq!({ p.tag }, 1); // braces avoid unaligned read lint
        assert_eq!({ p.value }, 0x1122334455667788u64);
    }

    // -----------------------------------------------------------------------
    // Case 16: Struct field order mismatch
    // -----------------------------------------------------------------------

    #[test]
    fn make_header_legal() {
        let h = unsafe { super::ffi::make_header_legal() };
        assert_eq!(h.tag, 7);
        assert_eq!(h.len, 16);
    }

    #[test]
    fn make_header_illegal() {
        // Fields are swapped relative to what Rust's Header expects.
        let h = unsafe { super::ffi::make_header_illegal() };
        // h.len is at offset 0 (what Rust would call tag), h.tag at offset 4.
        assert_eq!(h.len, 16);
        assert_eq!(h.tag, 7);
    }

    // -----------------------------------------------------------------------
    // Case 17: Mutable and shared alias to the same memory
    // -----------------------------------------------------------------------

    #[test]
    fn separate_output_legal() {
        let mut src = [1u8, 2, 3, 4];
        let mut dst: *const u8 = std::ptr::null();
        unsafe { super::ffi::separate_output_legal(src.as_mut_ptr(), src.len(), &mut dst) };
        assert!(!dst.is_null());
        // dst points to a fresh copy; different address from src.
        assert_ne!(dst, src.as_ptr());
        let copy = unsafe { std::slice::from_raw_parts(dst, src.len()) };
        assert_eq!(copy, &[1, 2, 3, 4]);
        // Free the copy allocated by the C function.
        unsafe { free(dst as *mut std::ffi::c_void) };
    }

    #[test]
    fn in_place_alias_illegal() {
        let mut src = [1u8, 2, 3, 4];
        let mut dst: *const u8 = std::ptr::null();
        unsafe { super::ffi::in_place_alias_illegal(src.as_mut_ptr(), src.len(), &mut dst) };
        // dst now aliases src — both point to the same memory.
        assert_eq!(dst, src.as_ptr());
    }

    // -----------------------------------------------------------------------
    // Case 18: Two mutable aliases to the same memory
    // -----------------------------------------------------------------------

    #[test]
    fn split_mut_legal() {
        let mut a: *mut u8 = std::ptr::null_mut();
        let mut b: *mut u8 = std::ptr::null_mut();
        unsafe { super::ffi::split_mut_legal(&mut a, &mut b) };
        assert!(!a.is_null());
        assert!(!b.is_null());
        // a and b must point to distinct objects.
        assert_ne!(a, b);
    }

    #[test]
    fn split_mut_illegal() {
        let mut a: *mut u8 = std::ptr::null_mut();
        let mut b: *mut u8 = std::ptr::null_mut();
        unsafe { super::ffi::split_mut_illegal(&mut a, &mut b) };
        // Both point to the same object — two simultaneous mutable aliases.
        assert_eq!(a, b);
    }

    // -----------------------------------------------------------------------
    // Case 19: Mutation through a nominally immutable input
    // -----------------------------------------------------------------------

    #[test]
    fn read_only_sum_legal() {
        let data: [u8; 4] = [1, 2, 3, 4];
        let sum = unsafe { super::ffi::read_only_sum_legal(data.as_ptr(), data.len()) };
        assert_eq!(sum, 10);
        // data is unchanged.
        assert_eq!(data, [1, 2, 3, 4]);
    }

    #[test]
    fn write_through_const_illegal() {
        // Mutates through a const pointer — violates shared borrow.
        let data: [u8; 4] = [1, 2, 3, 4];
        unsafe { super::ffi::write_through_const_illegal(data.as_ptr(), data.len()) };
        // data[0] has been overwritten with 0xFF by the C function.
        assert_eq!(data[0], 0xFF);
    }

    // -----------------------------------------------------------------------
    // Case 20: Concurrent background mutation after the call returns
    // -----------------------------------------------------------------------

    #[test]
    fn no_background_mutation_legal() {
        let mut buf = [0xAAu8];
        unsafe { super::ffi::no_background_mutation_legal(buf.as_mut_ptr()) };
        // XOR with 0x01: 0xAA ^ 0x01 == 0xAB
        assert_eq!(buf[0], 0xAB);
    }

    #[test]
    fn start_background_mutation_illegal() {
        // Spawns a detached thread that mutates buf after this call returns.
        // We just verify the call completes without panic; we do NOT rely on
        // the value of buf[0] since the race is non-deterministic.
        let mut buf = [0u8];
        unsafe { super::ffi::start_background_mutation_illegal(buf.as_mut_ptr()) };
    }

    // -----------------------------------------------------------------------
    // Case 21: Asynchronous mutation via a signal handler
    // -----------------------------------------------------------------------

    #[test]
    fn no_async_mutation_legal() {
        let mut buf = [0xAAu8];
        unsafe { super::ffi::no_async_mutation_legal(buf.as_mut_ptr()) };
        assert_eq!(buf[0], 0xAB); // 0xAA ^ 0x01
    }

    /// Raises SIGUSR1 which may interfere with the test harness signal
    /// disposition; skipped by default.
    #[test]
    #[ignore]
    fn async_signal_mutation_illegal() {
        let mut buf = [0u8];
        unsafe { super::ffi::async_signal_mutation_illegal(buf.as_mut_ptr()) };
    }

    // -----------------------------------------------------------------------
    // Case 22: longjmp across the FFI boundary
    // -----------------------------------------------------------------------

    #[test]
    fn foreign_call_legal() {
        let rc = unsafe { super::ffi::foreign_call_legal() };
        assert_eq!(rc, -1);
    }

    #[test]
    fn set_env_for_demo() {
        // Calling setjmp inside set_env_for_demo is safe on its own.
        unsafe { super::ffi::set_env_for_demo() };
    }

    /// Calls longjmp, which unwinds past Rust frames without running
    /// destructors.  Unsafe to run in the test harness.
    #[test]
    #[ignore]
    fn foreign_call_illegal() {
        unsafe {
            super::ffi::set_env_for_demo();
            super::ffi::foreign_call_illegal();
        }
    }

    // -----------------------------------------------------------------------
    // Case 23: Invalid UTF-8 for a Rust str
    // -----------------------------------------------------------------------

    #[test]
    fn get_utf8_legal() {
        let p = unsafe { super::ffi::get_utf8_legal() };
        assert!(!p.is_null());
        let s = unsafe { std::ffi::CStr::from_ptr(p) };
        assert_eq!(s.to_str().unwrap(), "hello");
    }

    #[test]
    fn get_utf8_illegal() {
        // Returns bytes containing 0xFF — not valid UTF-8.
        let p = unsafe { super::ffi::get_utf8_illegal() };
        assert!(!p.is_null());
        let bytes = unsafe { std::ffi::CStr::from_ptr(p).to_bytes() };
        assert_eq!(bytes[0], 0xFF);
        // Confirm that converting to &str fails as expected.
        let s = unsafe { std::ffi::CStr::from_ptr(p) };
        assert!(s.to_str().is_err());
    }

    // -----------------------------------------------------------------------
    // Case 24: Uninitialized scalar value
    // -----------------------------------------------------------------------

    #[test]
    fn get_initialized_u32_legal() {
        let v = unsafe { super::ffi::get_initialized_u32_legal() };
        assert_eq!(v, 1234);
    }

    #[test]
    fn get_uninitialized_u32_illegal() {
        // Reads an uninitialized stack slot; the value is indeterminate.
        // We just verify the FFI call itself does not abort.
        let _v = unsafe { super::ffi::get_uninitialized_u32_illegal() };
    }

    // -----------------------------------------------------------------------
    // Case 25: Overlapping buffers where Rust assumes non-overlap
    // -----------------------------------------------------------------------

    #[test]
    fn copy_nonoverlap_legal() {
        let src: [u8; 4] = [1, 2, 3, 4];
        let mut dst: [u8; 4] = [0; 4];
        unsafe { super::ffi::copy_nonoverlap_legal(dst.as_mut_ptr(), src.as_ptr(), src.len()) };
        assert_eq!(dst, [1, 2, 3, 4]);
    }

    #[test]
    fn copy_nonoverlap_illegal() {
        // Copies buf[0..len] to buf[1..len+1] — source and destination overlap.
        // The result is implementation-defined but the call itself is observable.
        let mut buf: [u8; 8] = [1, 2, 3, 4, 0, 0, 0, 0];
        unsafe { super::ffi::copy_nonoverlap_illegal(buf.as_mut_ptr(), 4) };
        // We do not assert a specific value; the overlap makes it UB.
    }

    // -----------------------------------------------------------------------
    // Case 26: Pointer-sized integer used with stale provenance
    // -----------------------------------------------------------------------

    #[test]
    fn export_live_pointer_legal() {
        let raw = unsafe { super::ffi::export_live_pointer_legal() };
        assert_ne!(raw, 0);
        // Round-trip: cast back to pointer and dereference (live static).
        let val = unsafe { *(raw as *const i32) };
        assert_eq!(val, 42);
    }

    #[test]
    fn export_stale_pointer_illegal() {
        // Returns the address of a freed allocation — stale provenance.
        // Do NOT dereference; just confirm the call returns a non-zero value.
        let raw = unsafe { super::ffi::export_stale_pointer_illegal() };
        let _ = raw;
    }
}
