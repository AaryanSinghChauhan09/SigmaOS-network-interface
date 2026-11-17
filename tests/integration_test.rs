// Integration test for SigmaOS-network-interface
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use network_interface as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
