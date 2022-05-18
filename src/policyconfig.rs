/**
 * file: policyconfig.rs
 * desc: Defines an undocumented windows API that can be used to switch audio devices.
 *       IPolicyConfig is an undocumented API, beginning with Vista, that allows an application
 *       to set the default audio device. First RE'd by EreTIk, credits to him for first describing
 *       this interface.
 *       This has only been tested to work on Win10 but should maybe work with Win7+.
 */
use windows::Win32::Media::Audio::{ERole, WAVEFORMATEX};
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::UI::Shell::PropertiesSystem::PROPERTYKEY;

#[allow(non_upper_case_globals)]
// This the GUID for the undocumented PolicyConfig class.
pub const PolicyConfig: windows::core::GUID =
    windows::core::GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

/*
 * STRUCTS
 */

// PolicyConfig interface
#[repr(transparent)]
pub struct IPolicyConfig(windows::core::IUnknown);

// This defines the vtable for the PolicyConfig interface. Some of these function arguments
// are incorrect, e.g., *ShareMode functions. But since we only care about the
// SetDefaultEndpoint function, everything else doesn't matter. We could just use placeholder
// functions to get the padding correct but I've defined everything else just in case.
#[allow(non_snake_case)]
#[repr(C)]
pub struct IPolicyConfig_Vtbl {
    pub base__: windows::core::IUnknownVtbl,
    //
    pub GetMixFormat: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        objectformat: *const WAVEFORMATEX,
    ) -> windows::core::HRESULT,
    //
    pub GetDeviceFormat: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        someint: u32,
        objectformat: *const WAVEFORMATEX,
    ) -> windows::core::HRESULT,
    //
    pub ResetDeviceFormat: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
    ) -> windows::core::HRESULT,
    //
    pub SetDeviceFormat: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        objectformat: *const WAVEFORMATEX,
        otherformat: *const WAVEFORMATEX,
    ) -> windows::core::HRESULT,
    //
    pub GetProcessingPeriod: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        someint: u32,
        pint0: *const u64,
        pint1: *const u64,
    ) -> windows::core::HRESULT,
    //
    pub SetProcessingPeriod: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        pint0: *const u64,
    ) -> windows::core::HRESULT,
    //
    pub GetShareMode: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        pint0: *const u64,
    ) -> windows::core::HRESULT,
    //
    pub SetShareMode: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        pint0: *const u64,
    ) -> windows::core::HRESULT,
    //
    pub GetPropertyValue: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        key: *const PROPERTYKEY,
        otherformat: *const PROPVARIANT,
    ) -> windows::core::HRESULT,
    //
    pub SetPropertyValue: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        key: *const PROPERTYKEY,
        otherformat: *const PROPVARIANT,
    ) -> windows::core::HRESULT,
    //
    pub SetDefaultEndpoint: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        role: ERole,
    ) -> windows::core::HRESULT,
    //
    pub SetEndpointVisibility: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        pwstrdeviceid: windows::core::PCWSTR,
        someint: u32,
    ) -> windows::core::HRESULT,
}

/*
 * IMPLEMENTATIONS
 */

// Function implementations for the PolicyConfig interface. Since we're only using
// SetDefaultEndpoint, this is the only function we need to implement.
#[allow(non_snake_case)]
impl IPolicyConfig {
    pub unsafe fn SetDefaultEndpoint<
        'a,
        Param0: windows::core::IntoParam<'a, windows::core::PCWSTR>,
    >(
        &self,
        pwstrdeviceid: Param0,
        role: ERole,
    ) -> windows::core::Result<IPolicyConfig> {
        let result__: windows::core::RawPtr = core::mem::zeroed();
        (windows::core::Interface::vtable(self).SetDefaultEndpoint)(
            core::mem::transmute_copy(self),
            pwstrdeviceid.into_param().abi(),
            core::mem::transmute(role),
        )
        .from_abi::<IPolicyConfig>(result__)
    }
}
impl core::convert::From<IPolicyConfig> for windows::core::IUnknown {
    fn from(value: IPolicyConfig) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl core::convert::From<&IPolicyConfig> for ::windows::core::IUnknown {
    fn from(value: &IPolicyConfig) -> Self {
        core::convert::From::from(core::clone::Clone::clone(value))
    }
}
impl<'a> windows::core::IntoParam<'a, windows::core::IUnknown> for IPolicyConfig {
    fn into_param(self) -> windows::core::Param<'a, windows::core::IUnknown> {
        windows::core::Param::Owned(unsafe { core::mem::transmute(self) })
    }
}
impl<'a> windows::core::IntoParam<'a, windows::core::IUnknown> for &'a IPolicyConfig {
    fn into_param(self) -> windows::core::Param<'a, windows::core::IUnknown> {
        windows::core::Param::Borrowed(unsafe { core::mem::transmute(self) })
    }
}
impl core::clone::Clone for IPolicyConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// Interface for IPolicyConfig. This sets the vtable and appropriate GUID.
// This GUID should work for Windows 7+ but I've only tested on Windows 10.
unsafe impl windows::core::Interface for IPolicyConfig {
    type Vtable = IPolicyConfig_Vtbl;
    const IID: windows::core::GUID =
        windows::core::GUID::from_u128(0xf8679f50_850a_41cf_9c72_430f290290c8);
}

#[cfg(test)]
#[cfg(target_os = "windows")]
#[path = "tests/policyconfig_tests.rs"]
mod policyconfig_tests;
