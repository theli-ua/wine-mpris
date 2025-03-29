use std::collections::HashMap;

use log::{info, warn};
use windows::{
    core::*,
    Win32::{Foundation::*, System::WinRT::*},
};

use crate::controls::MediaControls;

use super::bindings::Media::SystemMediaTransportControls;

#[derive(Default)]
#[implement(IActivationFactory, ISystemMediaTransportControlsInterop)]
pub struct ActivationFactory {
    controls: std::sync::Mutex<HashMap<isize, SystemMediaTransportControls>>,
}

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
        let result = self
            .controls
            .lock()
            .unwrap()
            .entry(appwindow.0)
            .or_insert_with(|| MediaControls::new(appwindow).into())
            .clone();
        // let result: SystemMediaTransportControls = MediaControls::new(appwindow).into();

        unsafe {
            core::ptr::write(mediatransportcontrol, core::mem::transmute_copy(&result));
            core::mem::forget(result);
        }

        Ok(())
    }
}
