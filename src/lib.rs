#![allow(non_snake_case)]
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};
// use windows::Media::SystemMediaTransportControls;

#[implement(IActivationFactory, ISystemMediaTransportControlsInterop)]
struct ActivationFactory;

impl IActivationFactory_Impl for ActivationFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        dbg!("AAA");
        Err(E_NOTIMPL.into())
    }
}

impl ISystemMediaTransportControlsInterop_Impl for ActivationFactory {
    // Required method
    fn GetForWindow(
        &self,
        appwindow: HWND,
        riid: *const GUID,
        mediatransportcontrol: *mut *mut std::ffi::c_void,
    ) -> Result<()> {
        dbg!(appwindow);
        dbg!(unsafe { *riid });
        unimplemented!()
    }
}

#[no_mangle]
extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    eprintln!("Hello World! {:?}", *name);

    if result.is_null() {
        return E_POINTER;
    }

    let mut factory: Option<IActivationFactory> = None;

    if *name == "Windows.Media.SystemMediaTransportControls" {
        factory = Some(ActivationFactory.into());
    }
    dbg!(&factory);

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
    rclsid: *const GUID,
    riid: *const GUID,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    eprintln!("Hello World!!!! {rclsid:?}, {riid:?}");
    CLASS_E_CLASSNOTAVAILABLE
}

mod propsys;
