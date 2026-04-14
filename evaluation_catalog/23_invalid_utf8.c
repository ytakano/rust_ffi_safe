/*
 * Case 23: Invalid UTF-8 for a Rust str
 *
 * Rust's str type requires its bytes to be valid UTF-8.  A C string that
 * is valid as raw bytes or as a C string but contains invalid UTF-8
 * sequences cannot be safely converted to &str on the Rust side.
 *
 * Assumed Rust-side binding: Rust converts the returned bytes directly
 * to str or otherwise assumes UTF-8 validity.
 */

/* Legal: returns a well-formed ASCII (and thus valid UTF-8) string. */
const char *
get_utf8_legal(void) {
    return "hello"; /* pure ASCII is always valid UTF-8 */
}

/* Illegal: returns a byte sequence containing 0xFF, which is never a
 * valid UTF-8 byte.  Interpreting this as a Rust str is undefined
 * behavior. */
const char *
get_utf8_illegal(void) {
    static const char s[] = { (char)0xFF, 0 }; /* 0xFF is invalid UTF-8 */
    return s;
}
