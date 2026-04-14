// Build script: compiles all 26 C source files from evaluation_catalog/
// into a static library that Cargo links into the Rust crate.

fn main() {
    let catalog = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../evaluation_catalog");

    cc::Build::new()
        .files([
            "01_invalid_bool.c",
            "02_invalid_char.c",
            "03_invalid_enum.c",
            "04_null_pointer.c",
            "05_nonzero_handle.c",
            "06_dangling_heap_ptr.c",
            "07_stack_ptr.c",
            "08_misaligned_ptr.c",
            "09_out_of_bounds_ptr.c",
            "10_slice_length.c",
            "11_forged_pointer.c",
            "12_borrowed_input.c",
            "13_owned_string.c",
            "14_uninitialized_out_param.c",
            "15_struct_packing.c",
            "16_struct_field_order.c",
            "17_mutable_shared_alias.c",
            "18_two_mutable_aliases.c",
            "19_const_mutation.c",
            "20_concurrent_mutation.c",
            "21_signal_mutation.c",
            "22_longjmp.c",
            "23_invalid_utf8.c",
            "24_uninitialized_scalar.c",
            "25_overlapping_buffers.c",
            "26_stale_pointer.c",
        ].map(|f| catalog.join(f)))
        .compile("eval_catalog");

    // Case 20 (20_concurrent_mutation.c) uses pthreads.
    println!("cargo:rustc-link-lib=pthread");
}
