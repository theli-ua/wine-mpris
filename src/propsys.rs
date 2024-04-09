///! Stubs for functions unimplemented in wine
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
