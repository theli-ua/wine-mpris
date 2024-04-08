use windows::core::GUID;
use windows::core::*;
use windows::Win32::Foundation::{CLASS_E_CLASSNOTAVAILABLE, E_POINTER};

#[no_mangle]
extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    eprintln!("Hello World!!!!");
    if result.is_null() {
        return E_POINTER;
    }
    CLASS_E_CLASSNOTAVAILABLE

    // let mut factory: Option<IActivationFactory> = None;

    // if *name == "Sample.JsonValidator" {
    //     factory = Some(JsonValidatorFactory.into());
    // }

    // Dereferencing `result` is safe because we've validated that the pointer is not null and
    // transmuting `factory` is safe because `DllGetActivationFactory` is expected to return an
    // `IActivationFactory` pointer and that's what `factory` contains.
    // unsafe {
    //     if let Some(factory) = factory {
    //         *result = std::mem::transmute(factory);
    //         S_OK
    //     } else {
    //         *result = std::ptr::null_mut();
    //         CLASS_E_CLASSNOTAVAILABLE
    //     }
    // }
}

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    eprintln!("Hello World!!!! 2");
    CLASS_E_CLASSNOTAVAILABLE
}
