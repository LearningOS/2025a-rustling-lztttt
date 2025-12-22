//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    let your_command = "cargo:rustc-cfg=feature=\"tests7\"";

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
