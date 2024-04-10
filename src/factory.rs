use log::{info, warn};
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

use crate::controls::MediaControls;

#[implement(IActivationFactory, ISystemMediaTransportControlsInterop)]
pub struct ActivationFactory;

impl IActivationFactory_Impl for ActivationFactory {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        eprintln!("ActivateInstance not implemented!");
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
        info!("GetForWindow hwnd:{appwindow:?}, riid:{:?}", riid);

        let result: super::bindings::Media::SystemMediaTransportControls =
            MediaControls(appwindow).into();

        unsafe {
            core::ptr::write(mediatransportcontrol, core::mem::transmute_copy(&result));
            core::mem::forget(result);
        }

        Ok(())
    }
}
