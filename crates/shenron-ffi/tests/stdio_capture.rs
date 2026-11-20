//! `with_captured_stdio` redirects the process-wide stdout and stderr descriptors, so it is tested from
//! its own test binary with a single test: inside the crate's unit tests, libtest prints the results of
//! tests running on other threads to the same descriptor, and those lines landed in the capture.
#![cfg(not(target_arch = "wasm32"))]

use shenron_ffi::stdio_capture::with_captured_stdio;
use std::io::Write;

#[test]
fn captures_output_and_propagates_the_return_code() {
    let (code, out, err) = with_captured_stdio(|| {
        let _ = std::io::stdout().write_all(b"captured-out-line\n");
        let _ = std::io::stderr().write_all(b"captured-err-line\n");
        7
    });
    assert_eq!(code, 7);
    assert!(out.contains("captured-out-line"), "stdout was: {out:?}");
    assert!(err.contains("captured-err-line"), "stderr was: {err:?}");

    let (code, out, err) = with_captured_stdio(|| 42);
    assert_eq!(code, 42);
    assert!(out.is_empty(), "stdout was: {out:?}");
    assert!(err.is_empty(), "stderr was: {err:?}");
}
