use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(IActivationFactory, ISystemMediaTransportControlsInterop)]
pub struct ActivationFactory;

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
