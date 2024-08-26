#![allow(non_snake_case)]
use std::ffi::*;

use icube_sdk_sys::{v1, v2, SDK};


fn main() {
    let api = unsafe { SDK::load().unwrap() };

    match api {
        SDK::V1(api) => unsafe { main_v1(api) },
        SDK::V2(api) => unsafe { main_v2(api) },
    }
}

unsafe fn main_v1(api: v1::API) {
    println!("Using the iCube SDK v1 (NETUSBCAM)");

    let num_devices = (api.Init)();
    println!("Found {} cameras:", num_devices);

    for index in 0..num_devices {
        let index = index as u32;

        assert_eq!((api.Open)(index), 0, "Failed to open camera #{index}");

        let mut res = Res { w: 0, h: 0 };
        assert_eq!((api.GetSize)(index, &mut res.w, &mut res.h), 0, "Failed to set callback for camera #{}", index);
        println!("Camera #{index}: {}x{} = {}", res.w, res.h, res.w * res.h);

        assert_eq!((api.SetCallback)(index, v1::CALLBACK_RGB, callback, &mut res as *mut Res as *mut c_void), 0, "Failed to set callback for camera #{index}");

        assert_eq!((api.Start)(index), 0, "Failed to start camera #{index}");

        std::thread::sleep(std::time::Duration::from_secs(2));

        assert_eq!((api.Stop)(index), 0, "Failed to stop camera #{index}");

        assert_eq!((api.Close)(index), 0, "Failed to close camera #{index}");
    }
}

unsafe fn main_v2(api: v2::API) {
    println!("Using the iCube SDK v2");

    let num_devices = (api.Init)();
    println!("Found {} cameras:", num_devices);

    for index in 0..num_devices {
        assert_eq!((api.Open)(index), 0, "Failed to open camera #{index}");

        let mut res = Res { w: 0, h: 0 };
        assert_eq!((api.GetSize)(index, &mut res.w, &mut res.h), 0, "Failed to set callback for camera #{}", index);
        println!("Camera #{index}: {}x{} = {}", res.w, res.h, res.w * res.h);

        assert_eq!((api.SetCallback)(index, v1::CALLBACK_RGB, callback_v2, &mut res as *mut Res as *mut c_void), 0, "Failed to set callback for camera #{index}");

        assert_eq!((api.Start)(index, std::ptr::null_mut(), 0, 1), 0, "Failed to start camera #{index}");

        std::thread::sleep(std::time::Duration::from_secs(2));

        assert_eq!((api.Stop)(index), 0, "Failed to stop camera #{index}");

        assert_eq!((api.Close)(index), 0, "Failed to close camera #{index}");
    }
}

#[derive(Debug)]
struct Res {
    w: i32,
    h: i32,
}

extern "C" fn callback(buffer: *mut c_void, bufferSize: c_uint, context: *mut c_void) -> c_int {
    let res = unsafe { &*(context as *mut Res) };
    println!("Callback: buffer={buffer:?}, bufferSize={bufferSize}; res={res:?}");
    0
}

extern "C" fn callback_v2(pBuf: *mut u8, lBufferSize: c_long, pContext: *mut c_void) -> c_long {
    let res = unsafe { &*(pContext as *mut Res) };
    println!("Callback: buffer={pBuf:?}, bufferSize={lBufferSize}; res={res:?}");
    0
}