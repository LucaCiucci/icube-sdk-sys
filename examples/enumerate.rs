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
        let mut name = [0i8; v2::NETCAM_NAME_LENGTH as usize];
        let result = (api.GetName)(index as u32, name.as_mut_ptr(), v2::NETCAM_NAME_LENGTH as _);
        assert_eq!(result, 0, "Failed to get name for camera #{}", index);
        let name = name.into_iter().map(|c| c as u8).collect::<Vec<_>>();
        let name = CStr::from_bytes_until_nul(&name).unwrap();
        println!("  - {}", name.to_string_lossy());
    }
}

unsafe fn main_v2(api: v2::API) {
    println!("Using the iCube SDK v2");

    let num_devices = (api.Init)();
    println!("Found {} cameras:", num_devices);

    for index in 0..num_devices {
        let mut name = [0i8; v2::NETCAM_NAME_LENGTH as usize];
        let result = (api.GetName)(index, name.as_mut_ptr());
        assert_eq!(result, 0, "Failed to get name for camera #{}", index);
        let name = name.into_iter().map(|c| c as u8).collect::<Vec<_>>();
        let name = CStr::from_bytes_until_nul(&name).unwrap();
        println!("  - {}", name.to_string_lossy());
    }
}