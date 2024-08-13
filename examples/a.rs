use icube_sdk_sys::{SDK};


fn main() {
    let api = unsafe { SDK::load().unwrap() };

    match &api {
        SDK::V1(_) => println!("Using the iCube SDK v1 (NETUSBCAM)"),
        SDK::V2(_) => println!("Using the iCube SDK v2"),
    }
}