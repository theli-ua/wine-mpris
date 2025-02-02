#![allow(non_snake_case)]
use std::sync::OnceLock;

use log::{info, warn};
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

pub mod bindings;
pub mod controls;
mod factory;
mod propsys;

static LOG_INIT: OnceLock<()> = OnceLock::new();

fn init_log() {
    LOG_INIT.get_or_init(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .target(env_logger::Target::Stdout)
            .init();
    });
}

#[no_mangle]
extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    init_log();
    if result.is_null() {
        return E_POINTER;
    }

    let mut factory: Option<IActivationFactory> = None;

    if *name == "Windows.Media.SystemMediaTransportControls" {
        info!("Returning factory SystemMediaTransportControls");
        factory = Some(factory::ActivationFactory.into());
    } else {
        warn!("No factory for {}", *name);
    }

    // Dereferencing `result` is safe because we've validated that the pointer is not null and
    // transmuting `factory` is safe because `DllGetActivationFactory` is expected to return an
    // `IActivationFactory` pointer and that's what `factory` contains.
    unsafe {
        if let Some(factory) = factory {
            *result = std::mem::transmute(factory);
            S_OK
        } else {
            *result = std::ptr::null_mut();
            CLASS_E_CLASSNOTAVAILABLE
        }
    }
}

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    _rclsid: *const GUID,
    _riid: *const GUID,
    _result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    init_log();
    warn!("DllGetClassObject not implemented");
    CLASS_E_CLASSNOTAVAILABLE
}
