mod macros;

pub mod v1;
pub mod v2;

#[derive(Debug)]
pub enum SDK {
    V1(v1::API),
    V2(v2::API),
}

pub use libloading;

impl SDK {
    /// Try to load the SDK using [`default_lib_name`].
    pub unsafe fn load() -> Option<Self> { // TODO exhaustive error handling
        let lib_name = default_lib_name().ok()?;

        if let Ok(v2) = unsafe { v2::API::new(lib_name) } {
            if v2.check_version() {
                return Some(SDK::V2(v2));
            } else {
                log::error!("The version of the iCube SDK v2 is wrong");
            }
        }

        if let Ok(v1) = unsafe { v1::API::new(lib_name) } {
            if v1.check_version() {
                return Some(SDK::V1(v1));
            } else {
                log::error!("The version of the iCube SDK v1 (NETUSBCAM) is wrong");
            }
        }

        None
    }
}

pub fn default_lib_name() -> Result<&'static str, UnsupportedPlatformError> {
    #[cfg(target_os = "linux")]
    return Ok("libNETUSBCAM.so");
    #[cfg(target_os = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        return Ok("ICubeSDK64");
        #[cfg(target_arch = "x86")]
        return "ICubeSDK";
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    return Err(UnsupportedPlatformError);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UnsupportedPlatformError;

impl std::fmt::Display for UnsupportedPlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unsupported platform")
    }
}

impl std::error::Error for UnsupportedPlatformError {}