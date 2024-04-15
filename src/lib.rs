#![allow(non_snake_case)]
use std::{
    ptr::{null, null_mut},
    sync::OnceLock,
};

use log::{info, warn};
use windows::{core::*, Win32::Foundation::*, Win32::System::WinRT::*};

use crate::foo::__wine_init_unix_call;

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
    // let res  = unixlib::__wine_unix_call_funcs

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

#[no_mangle]
extern "system" fn DllMain(_: *const u8, _: u32, _: *const u8) -> u32 {
    init_log();
    assert!(__wine_init_unix_call().is_ok());
    // let res = unsafe { foo::hello_world(null_mut()) };
    // info!("res: {res}");
    1
}

mod unixlib {
    use std::ffi::c_void;

    use windows_core::HRESULT;

    unsafe fn hello_world(args: *mut c_void) -> HRESULT {
        HRESULT(16)
    }
    #[no_mangle]
    #[used]
    static mut __wine_unix_call_funcs: [unsafe fn(args: *mut c_void) -> HRESULT; 1usize] =
        [hello_world];
}
mod foo {
    use std::{
        ffi::c_void,
        mem::size_of_val,
        ptr::{addr_of, null_mut},
    };

    use windows::{
        Wdk::Storage::FileSystem::{NtQueryVirtualMemory, MEMORY_INFORMATION_CLASS},
        Win32::{
            Foundation::HMODULE,
            System::{
                LibraryLoader::{GetModuleHandleA, GetModuleHandleExA, GetProcAddress},
                Threading::GetCurrentProcess,
            },
        },
    };
    use windows_core::{HRESULT, PCSTR};

    pub type UnixlibHandle = u64;
    pub static mut __WINE_UNIXLIB_HANDLE: UnixlibHandle = 0;
    pub static mut __WINE_UNIX_CALL_DISPATCHER: unsafe fn(
        UnixlibHandle,
        usize,
        *mut std::ffi::c_void,
    ) -> HRESULT = __dummy_dispatcher;

    fn __dummy_dispatcher(_: UnixlibHandle, _: usize, _: *mut std::ffi::c_void) -> HRESULT {
        HRESULT(0)
    }
    const NTDLL_NAME: &[u8] = b"ntdll.dll\0";
    const CALL_DISPATCHER_NAME: &[u8] = b"__wine_unix_call_dispatcher\0";
    extern "system" {
        static __ImageBase: windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
    }

    pub fn __wine_init_unix_call() -> HRESULT {
        let mut ntdll: HMODULE = HMODULE(0);
        unsafe { GetModuleHandleExA(0, PCSTR::from_raw(NTDLL_NAME.as_ptr()), &mut ntdll).unwrap() };
        unsafe {
            __WINE_UNIX_CALL_DISPATCHER = std::mem::transmute(
                GetProcAddress(ntdll, PCSTR::from_raw(CALL_DISPATCHER_NAME.as_ptr())).unwrap(),
            )
        };
        unsafe {
            let var_name = GetModuleHandleA(PCSTR::from_raw(std::ptr::null()))
                .unwrap()
                .0 as *const c_void;
            log::debug!("base: {:?} ?= {:?}", &__ImageBase as *const _, var_name);
            let status = NtQueryVirtualMemory(
                GetCurrentProcess(),
                Some(&__ImageBase as *const _ as *const _),
                MEMORY_INFORMATION_CLASS(1000),
                addr_of!(__WINE_UNIXLIB_HANDLE) as *mut _,
                size_of_val(&__WINE_UNIXLIB_HANDLE),
                None,
            );
            log::debug!("AAAA: {status:?}, {__WINE_UNIXLIB_HANDLE:?}");
        };
        HRESULT(0)
    }
    pub unsafe fn hello_world(args: *mut std::ffi::c_void) -> HRESULT {
        __WINE_UNIX_CALL_DISPATCHER(__WINE_UNIXLIB_HANDLE, 0usize, args)
    }
}
