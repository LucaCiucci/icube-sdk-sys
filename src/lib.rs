#![doc = include_str!("../README.md")]
#![cfg(target_os = "windows")]

pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/NET_ICube_API.h.rs"));
}

/// Convert a string literal to a C string.
///
/// # Examples
/// ```
/// # use icube_sdk_sys::to_c_str;
/// to_c_str!(name = "ICubeSDK64");
///
/// let name: String = "Hello".into();
/// to_c_str!(name);
///
/// let name: String = "Hello".into();
/// to_c_str!(renamed_name = name);
#[macro_export]
macro_rules! to_c_str {
    ($name:ident = $e:expr) => {
        let tmp = std::ffi::CString::new($e).unwrap();
        let $name = tmp.as_ptr();
    };
    ($name:ident) => {
        $crate::to_c_str!($name = $name);
    };
    ($($name:ident $(= $e:expr)?),*) => {
        $($crate::to_c_str!( $name $(= $e)? );)*
    };
}

pub unsafe fn load() -> Result<(), ()> {
    #[cfg(target_arch = "x86_64")]
    {
        to_c_str!(name = "ICubeSDK64");
        (ffi::LoadICubeApi(name) != 0).then(|| ()).ok_or(())
    }
    #[cfg(target_arch = "x86")]
    {
        to_c_str!(name = "ICubeSDK");
        (ffi::LoadICubeApi(name) != 0).then(|| ()).ok_or(())
    }
}

pub unsafe fn unload() {
    ffi::UnloadICubeApi();
}