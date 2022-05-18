/**
 * file: policyconfig_tests.rs
 * desc: Test the PolicyConfig implementation. Just checks to see that the PolicyConfig
 *       interface can be retrieved and the VTable is correctly defined.
 */
use windows::core::Error;
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};

use crate::policyconfig::{IPolicyConfig, PolicyConfig};

#[test]
fn test_policyconfig_interface() {
    let init_result = unsafe { CoInitialize(std::ptr::null_mut()) };

    assert!(init_result.is_ok());

    // Generate an interface to PolicyConfig, this should fail if the VTable definition
    // is wrong.
    let policy_wrapper: Result<IPolicyConfig, Error> =
        unsafe { CoCreateInstance(&PolicyConfig, None, CLSCTX_ALL) };

    assert!(policy_wrapper.is_ok());
}
