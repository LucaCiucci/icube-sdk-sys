use icube_sdk_sys::ffi;

fn main() {
    unsafe {
        icube_sdk_sys::load();

        let num_devices = ffi::ICubeSDK_Init.unwrap()();

        println!("Found {} cameras", num_devices);
        for i in 0..num_devices {
            let mut name = [0i8; ffi::NETCAM_NAME_LENGTH as usize];
            let result = ffi::ICubeSDK_GetName.unwrap()(i, name.as_mut_ptr()) == ffi::IC_SUCCESS as i32;
            result.then(|| ()).expect(&format!("Failed to get name for camera #{i}"));
            let name = std::ffi::CStr::from_ptr(name.as_ptr()).to_str().unwrap();
            println!("Camera #{}: {:?}", i, name);
        }

        icube_sdk_sys::unload();
    }
}