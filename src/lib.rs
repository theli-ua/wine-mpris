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

mod propsys {
    use windows::{core::HRESULT, Win32};

    #[no_mangle]
    pub extern "system" fn VariantToDouble(
        varin: *const std::mem::MaybeUninit<windows::core::VARIANT>,
        pdblret: *mut f64,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToUInt64(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut u64,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToInt64(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut i64,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToInt32(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut i32,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToUInt32(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut u32,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToInt16(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut i16,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToUInt16(
        varin: *const ::std::mem::MaybeUninit<windows::core::VARIANT>,
        pullret: *mut u16,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToBoolean(
        varin: *const ::std::mem::MaybeUninit<::windows::core::VARIANT>,
        pfret: *mut Win32::Foundation::BOOL,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn PropVariantToBSTR(
        propvar: *const std::mem::MaybeUninit<windows::core::PROPVARIANT>,
        pbstrout: *mut std::mem::MaybeUninit<windows::core::BSTR>,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn PropVariantToVariant(
        ppropvar: *const std::mem::MaybeUninit<windows::core::PROPVARIANT>,
        pvar: *mut std::mem::MaybeUninit<windows::core::VARIANT>,
    ) -> HRESULT {
        unimplemented!()
    }

    #[no_mangle]
    pub extern "system" fn VariantToPropVariant(
        pvar: *const std::mem::MaybeUninit<windows::core::VARIANT>,
        ppropvar: *mut std::mem::MaybeUninit<windows::core::PROPVARIANT>,
    ) -> HRESULT {
        unimplemented!()
    }
}
