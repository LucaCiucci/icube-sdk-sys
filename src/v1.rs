#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ffi::*;
use crate::macros::decl_api;

pub const IC_SUCCESS: i32 = 0;
pub const IC_ERROR: i32 = -1;

pub const IC_IF_NOT_OPEN: i32 = -2;
pub const IC_WRONG_PARAM: i32 = -3;
pub const IC_OUT_OF_MEMORY: i32 = -4;
pub const IC_ALREADY_DONE: i32 = -5;
pub const IC_WRONG_CLOCK_VAL: i32 = -6;
pub const IC_COM_LIB_INIT: i32 = -7;
pub const IC_NOT_IF_STARTED: i32 = -8;

pub const MAX_MODE_COUNT: i32 = 10;

pub const MOC_MODE_320x240: i32 = 0;
pub const MOC_MODE_640x480: i32 = 1;
pub const MOC_MODE_752x480: i32 = 2;
pub const MOC_MODE_800x600: i32 = 3;
pub const MOC_MODE_1024x768: i32 = 4;
pub const MOC_MODE_1280x1024: i32 = 5;
pub const MOC_MODE_1600x1200: i32 = 6;
pub const MOC_MODE_2048x1536: i32 = 7;
pub const MOC_MODE_2592x1944: i32 = 8;
pub const MOC_MODE_3840x2748: i32 = 9;

pub const MODE_SKIP: i32 = 0;
pub const MODE_BIN: i32 = 1;

pub const BIN_SKIP_OFF: i32 = 0;
pub const BIN_SKIP_2ND_PIXEL: i32 = 1;
pub const BIN_SKIP_4TH_PIXEL: i32 = 2;

pub const NET_1044_BIN_MIN_CLOCK_VAL: i32 = 15;

pub const MEASUREFIELD_100: i32 = 2;
pub const MEASUREFIELD_60: i32 = 1;
pub const MEASUREFIELD_30: i32 = 0;

pub const SHUTTER_ROLLING: i32 = 0;
pub const SHUTTER_GLOBAL_RESET: i32 = 1;
pub const SHUTTER_GLOBAL: i32 = 2;

pub const OFF: i32 = 0;
pub const ON: i32 = 1;

pub const FALLING_EDGE: i32 = 0;
pub const RISING_EDGE: i32 = 1;

pub const DELAYED_TRIGGER_RETURN: i32 = 0;

pub const IMMEDIATE_TRIGGER_RETURN: i32 = 1;

pub const REG_BRIGHTNESS: i32 = 1;
pub const REG_CONTRAST: i32 = 2;
pub const REG_GAMMA: i32 = 3;
pub const REG_FLIPPED_V: i32 = 4;
pub const REG_FLIPPED_H: i32 = 5;
pub const REG_WHITE_BALANCE: i32 = 6;
pub const REG_EXPOSURE_TIME: i32 = 7;
pub const REG_EXPOSURE_TARGET: i32 = 8;
pub const REG_RED: i32 = 9;
pub const REG_GREEN: i32 = 10;
pub const REG_BLUE: i32 = 11;
pub const REG_BLACKLEVEL: i32 = 12;
pub const REG_GAIN: i32 = 13;
pub const REG_COLOR: i32 = 14;
pub const REG_PLL: i32 = 15;
pub const REG_STROBE_LENGHT: i32 = 16;
pub const REG_STROBE_DELAY: i32 = 17;
pub const REG_TRIGGER_DELAY: i32 = 18;
pub const REG_TRIGGER_INVERT: i32 = 21;
pub const REG_MEASURE_FIELD_AE: i32 = 22;
pub const REG_SHUTTER: i32 = 26;
pub const REG_ROI_ID: i32 = 27;
pub const REG_ROI_CYCLE: i32 = 28;
pub const REG_DEFECT_COR: i32 = 43;
pub const REG_SW_TRIG_MODE: i32 = 94;
pub const REG_CALLBACK_BR_FRAMES: i32 = 97;
pub const REG_INVERT_PIXEL: i32 = 113;
pub const REG_PIXEL_DEPTH: i32 = 120;
pub const REG_SENSOR_TIMING: i32 = 123;

pub const DEFAULT_GAMMA: i32 = 64;

pub const TRIG_SW_START: i32 = 0;
pub const TRIG_SW_DO: i32 = 1;
pub const TRIG_HW_START: i32 = 2;
pub const TRIG_STOP: i32 = 3;
pub const TRIG_SW_START_2: i32 = 4;
pub const TRIG_HW_START_2: i32 = 5;

pub const CALLBACK_RAW: i32 = 0;
pub const CALLBACK_RGB: i32 = 1;
pub const CALLBACK_Y_8: i32 = 2;

pub const ROI_ID_1_2: i32 = 0;
pub const ROI_ID_1_1: i32 = 1;
pub const ROI_ID_2: i32 = 2;
pub const ROI_ID_3: i32 = 3;
pub const ROI_ID_4: i32 = 4;

#[repr(C)]
pub struct PARAM_PROPERTY {
    pub bEnabled: bool,
    pub bAuto: bool,
    pub bOnePush: bool,
    pub nDef: u32,
    pub nMin: u32,
    pub nMax: u32,
}

#[repr(C)]
pub struct PARAM_PROPERTY_f {
    pub bEnabled: bool,
    pub bAuto: bool,
    pub bOnePush: bool,
    pub nDef: f32,
    pub nMin: f32,
    pub nMax: f32,
}

#[repr(C)]
pub struct ROI_RANGE_PROPERTY {
    pub nXMin: i32,
    pub nXMax: i32,
    pub nYMin: i32,
    pub nYMax: i32,
}

decl_api! {
    NETUSBCAM {
        fn Init() -> c_int;
        fn GetApiVersion(version: *mut c_char, length: c_uint) -> c_int;
        fn Open(deviceIndex: c_uint) -> c_int;
        fn Close(deviceIndex: c_uint) -> c_int;
        fn Start(deviceIndex: c_uint) -> c_int;
        fn Stop(deviceIndex: c_uint) -> c_int;
        fn GetName(deviceIndex: c_uint, name: *mut c_char, length: c_uint) -> c_int;
        fn GetModeList(deviceIndex: c_uint, length: *mut c_uint, modesList: *mut c_uint) -> c_int;
        fn SetMode(deviceIndex: c_uint, mode: c_uint) -> c_int;
        fn GetMode(deviceIndex: c_uint, mode: *mut c_uint) -> c_int;
        fn SetCallback(
            deviceIndex: c_uint,
            mode: c_int,
            pCallbackFunc: extern "C" fn (buffer: *mut c_void, bufferSize: c_uint, context: *mut c_void) -> c_int,
            context: *mut c_void,
        ) -> c_int;
        fn GetBinSkipList(deviceIndex: c_uint, type_: c_uint, length: *mut c_uint, modesList: *mut c_uint) -> c_int;
        fn SetBinSkip(deviceIndex: c_uint, nFaktor: c_int, nMode: c_int) -> c_int;
        fn GetResolution(deviceIndex: c_uint, nXRes: *mut c_int, nYRes: *mut c_int, nXPos: *mut c_int, nYPos: *mut c_int) -> c_int;
        fn SetResolution(deviceIndex: c_uint, nXRes: c_int, nYRes: c_int, nXPos: c_int, nYPos: c_int) -> c_int;
        fn GetSize(deviceIndex: c_uint, nXRes: *mut c_int, nYRes: *mut c_int) -> c_int;
        fn GetBinSkip(deviceIndex: c_uint, nFaktor: *mut c_int, nMode: c_int) -> c_int;
        fn GetResolutionRange(deviceIndex: c_uint, property: *mut ROI_RANGE_PROPERTY) -> c_int;
        fn SetCamParameter(deviceIndex: c_uint, Type: c_int, Value: c_ulong) -> c_int;
        fn GetCamParameter(deviceIndex: c_uint, Type: c_int, Value: *mut c_ulong) -> c_int;
        fn GetCamParameterRange(deviceIndex: c_uint, Type: c_int, property: *mut PARAM_PROPERTY) -> c_int;
        fn GetParamAuto(deviceIndex: c_uint, Type: c_int, bAuto: *mut c_int) -> c_int;
        fn SetParamAuto(deviceIndex: c_uint, Type: c_int, bAuto: c_int) -> c_int;
        fn SetParamAutoDef(deviceIndex: c_uint, Type: c_int) -> c_int;
        fn SetParamOnePush(deviceIndex: c_uint, Type: c_int) -> c_int;
        fn GetFWVersion(deviceIndex: c_uint, version: *mut c_char, length: c_uint) -> c_int;
        fn GetTrigger(deviceIndex: c_uint, nMode: *mut c_int) -> c_int;
        fn SetTrigger(deviceIndex: c_uint, nMode: c_int) -> c_int;
        fn SaveToFile(deviceIndex: c_uint, name: *const c_char) -> c_int;
        fn GetBrokenFrames(deviceIndex: c_uint, pnFrames: *mut c_uint) -> c_int;
        fn GetGoodFrames(deviceIndex: c_uint, pnFrames: *mut c_uint) -> c_int;
        fn SetExposure(deviceIndex: c_uint, Value: f32) -> c_int;
        fn GetExposure(deviceIndex: c_uint, Value: *mut f32) -> c_int;
        fn GetExposureRange(deviceIndex: c_uint, property: *mut PARAM_PROPERTY_f) -> c_int;
    }
}

impl API {

    /// Check that the version of the SDK is correct.
    ///
    /// This ffi were provided for the `1.39`` version of the SDK,
    /// so we require this **exact** version.
    pub fn check_version(&self) -> bool {
        let mut version = [0; 32];
        let res = unsafe { (self.GetApiVersion)(version.as_mut_ptr() as *mut _, version.len() as _) };

        if res != IC_SUCCESS {
            log::error!("Failed to get the version of the SDK: returned 0x{:X}", res);
            return false;
        }

        let version = unsafe { CStr::from_ptr(version.as_ptr()) };
        if version != c"1.39" {
            log::error!("The version of the SDK is not 1.39: {}", version.to_string_lossy());
            return false;
        }

        true
    }
}