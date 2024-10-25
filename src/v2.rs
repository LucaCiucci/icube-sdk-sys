#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ffi::*;
use crate::macros::decl_api;

pub const IC_SUCCESS: c_int = 0;
pub const IC_ERROR: c_int = 1;

pub const IC_IF_NOT_OPEN: c_int = -1;
pub const IC_WRONG_PARAM: c_int = -2;
pub const IC_OUT_OF_MEMORY: c_int = -3;
pub const IC_ALREADY_DONE: c_int = -4;
pub const IC_WRONG_CLOCK_VAL: c_int = -5;
pub const IC_COM_LIB_INIT: c_int = -6;
pub const IC_NOT_IF_STARTED: c_int = -7;
pub const IC_WRONG_ROI_ID: c_int = -8;
pub const IC_IF_NOT_ENABLED: c_int = -9;
pub const IC_COLOR_CAM_ONLY: c_int = -10;
pub const IC_DRIVER_VERSION: c_int = -11;
pub const IC_D3D_INIT: c_int = -12;
pub const IC_BAD_POINTER: c_int = -13;
pub const IC_ERROR_FILE_SIZE: c_int = -14;
pub const IC_RECONNECTION_ACTIVE: c_int = -15;
pub const IC_USB_REQUEST_FAIL: c_int = -16;
pub const IC_RESOURCE_IN_USE: c_int = -17;
pub const IC_DEVICE_GONE: c_int = -18;
pub const IC_DLL_MISMATCH: c_int = -19;
pub const IC_WRONG_FW_VERSION: c_int = -20;
pub const IC_NO_RGB_CALLBACK: c_int = -21;
pub const IC_NO_USB30_CAMERA: c_int = -22;
pub const IC_ERR_FIX_RELATION: c_int = -23;
pub const IC_CRC_CONFIG_DATA: c_int = -24;
pub const IC_CONFIG_DATA: c_int = -25;
pub const IC_ERR_START_PNP: c_int = -26;
pub const IC_INVALID_CAM_TYPE: c_int = -27;
pub const IC_NOT_IF_STREAMING: c_int = -28;
pub const IC_USB_STARTUP: c_int = -29;

pub const MODE_320x240: c_int = 0;
pub const MODE_640x480: c_int = 1;
pub const MODE_752x480: c_int = 2;
pub const MODE_800x600: c_int = 3;
pub const MODE_1024x768: c_int = 4;
pub const MODE_1280x1024: c_int = 5;
pub const MODE_1600x1200: c_int = 6;
pub const MODE_2048x1536: c_int = 7;
pub const MODE_2592x1944: c_int = 8;
pub const MODE_3840x2748: c_int = 9;
pub const MODE_1920x1200: c_int = 10;

pub const MODE_SKIP: c_int = 0;
pub const MODE_BIN: c_int = 1;

pub const BIN_SKIP_OFF: c_int = 0;
pub const BIN_SKIP_2ND_PIXEL: c_int = 1;
pub const BIN_SKIP_4TH_PIXEL: c_int = 2;

pub const MEASUREFIELD_100: c_int = 2;
pub const MEASUREFIELD_60: c_int = 1;
pub const MEASUREFIELD_30: c_int = 0;

pub const SHUTTER_ROLLING: c_int = 0;
pub const SHUTTER_GLOBAL_RESET: c_int = 1;
pub const SHUTTER_GLOBAL: c_int = 2;

pub const OFF: c_int = 0;
pub const ON: c_int = 1;

pub const FALLING_EDGE: c_int = 0;
pub const RISING_EDGE: c_int = 1;

pub const DELAYED_TRIGGER_RETURN: c_int = 0;

pub const IMMEDIATE_TRIGGER_RETURN: c_int = 1;

pub const IS_8BIT: c_int = 0;
pub const IS_16BIT: c_int = 1;

pub const DE_BAYER_3x3: c_int = 0;
pub const DE_BAYER_5x5: c_int = 1;

pub const GAMMA_MODE_DEFAULT: c_int = 2;
pub const GAMMA_MODE_sRGB: c_int = 1;
pub const GAMMA_MODE_ITU709: c_int = 0;

pub const DISPLAY_GDI: c_int = 0;
pub const DISPLAY_D3D: c_int = 1;

pub const REG_BRIGHTNESS: c_int = 1;
pub const REG_CONTRAST: c_int = 2;
pub const REG_GAMMA: c_int = 3;
pub const REG_FLIPPED_V: c_int = 4;
pub const REG_FLIPPED_H: c_int = 5;
pub const REG_WHITE_BALANCE: c_int = 6;
pub const REG_EXPOSURE_TIME: c_int = 7;
pub const REG_EXPOSURE_TARGET: c_int = 8;
pub const REG_RED: c_int = 9;
pub const REG_GREEN: c_int = 10;
pub const REG_BLUE: c_int = 11;
pub const REG_BLACKLEVEL: c_int = 12;
pub const REG_GAIN: c_int = 13;
pub const REG_COLOR: c_int = 14;
pub const REG_PLL: c_int = 15;
pub const REG_STROBE_LENGHT: c_int = 16;
pub const REG_STROBE_DELAY: c_int = 17;
pub const REG_TRIGGER_DELAY: c_int = 18;
pub const REG_SATURATION: c_int = 19;
pub const REG_COLOR_ENHANCE: c_int = 20;
pub const REG_TRIGGER_INVERT: c_int = 21;
pub const REG_RECONNECTIONS: c_int = 22;
pub const REG_MEASURE_FIELD_AE: c_int = 23;
pub const REG_AVI_STATE: c_int = 24;
pub const REG_FOCUS: c_int = 25;
pub const REG_SHUTTER: c_int = 26;
pub const REG_ROI_ID: c_int = 27;
pub const REG_ROI_CYCLE: c_int = 28;

pub const REG_DEFECT_COR: c_int = 43;

pub const REG_BAD_FRAME: c_int = 81;
pub const REG_GOOD_FRAME: c_int = 82;

pub const REG_SW_TRIG_MODE: c_int = 94;

pub const REG_ROI_FPGA_ONE_FRAME: c_int = 96;
pub const REG_CALLBACK_BR_FRAMES: c_int = 97;
pub const REG_FGPA_VBLANKING: c_int = 98;
pub const REG_FGPA_HBLANKING: c_int = 99;
pub const REG_FGPA_CLK_DIVIDER: c_int = 100;
pub const REG_FGPA_ON_BOARD: c_int = 101;
pub const REG_SET_GPIO: c_int = 102;
pub const REG_GET_GPIO: c_int = 103;
pub const REG_SET_GPIO_MODE: c_int = 104;
pub const REG_MASK_ROI_ID: c_int = 105;
pub const REG_RED_OFFSET: c_int = 106;
pub const REG_GREEN_OFFSET: c_int = 107;
pub const REG_BLUE_OFFSET: c_int = 108;
pub const REG_HUE: c_int = 109;
pub const REG_COLOR_CORRECTION: c_int = 110;
pub const REG_GAMMA_ENABLE: c_int = 111;
pub const REG_GAMMA_MODE: c_int = 112;
pub const REG_INVERT_PIXEL: c_int = 113;
pub const REG_TNR: c_int = 115;
pub const REG_BAYER_CONVERSION: c_int = 116;
pub const REG_COLOR_PROCESSING: c_int = 117;
pub const REG_USB_SPEED: c_int = 118;
pub const REG_DEVICE_SPEED: c_int = 119;
pub const REG_DATA_TRANSMISSION: c_int = 120;

pub const REG_SIGNIFICANT_BITS: c_int = 125;
pub const REG_GRAPHIC_MODE: c_int = 126;
pub const REG_EDGE_ENHANCEMENT: c_int = 128;
pub const REG_EDGE_ENHANCEMENT_GAIN: c_int = 129;

pub const REG_SENSOR_STROBE: c_int = 132;

pub const REG_TRIG_TIMEOUT: c_int = 138;
pub const REG_PIPE_TIMEOUT_MODE: c_int = 139;
pub const REG_RESET_CAMERA: c_int = 140;
pub const REG_DISCONNECTIONS: c_int = 141;
pub const REG_XACT_RECOVER_MODE: c_int = 142;
pub const REG_RESET_TO_DEFAULT: c_int = 143;

pub const REG_TRANSFER_BUF_CNT: c_int = 145;
pub const REG_SW_TRIG_WD_MODE: c_int = 146;
pub const REG_AQU_FRAMERATE: c_int = 147;
pub const REG_SENSOR_STROBE_DELAY: c_int = 148;
pub const REG_SENSOR_OVERLAPPED: c_int = 149;
pub const REG_TRIG_DELAY_DIVIDER: c_int = 150;

pub const DEFAULT_GAMMA: c_int = 64;

pub const ROI_ID_1_2: c_int = 0;
pub const ROI_ID_1_1: c_int = 1;
pub const ROI_ID_2: c_int = 2;
pub const ROI_ID_3: c_int = 3;
pub const ROI_ID_4: c_int = 4;

pub const ROI_ID_0: c_int = 0;
pub const ROI_ID_1: c_int = 1;
//pub const ROI_ID_2: c_int = 2;
//pub const ROI_ID_3: c_int = 3;
//pub const ROI_ID_4: c_int = 4;
pub const ROI_ID_5: c_int = 5;

pub const ROI_STEP_SIZE: c_int = 4;
pub const ROI_STEP_SIZE_11000: c_int = 8;

pub const TRIG_SW_START: c_int = 0;
pub const TRIG_SW_DO: c_int = 1;
pub const TRIG_HW_START: c_int = 2;
pub const TRIG_STOP: c_int = 3;
pub const TRIG_SW_START_2: c_int = 4;
pub const TRIG_HW_START_2: c_int = 5;

pub const DLL_VERSION: c_int = 0;
pub const API_VERSION: c_int = 1;

pub const CALLBACK_RAW: c_int = 0;
pub const CALLBACK_RGB: c_int = 1;

pub const DISPLAY_NORMAL: c_int = 0;
pub const DISPLAY_FIT_TO_WINDOW: c_int = 1;
pub const DISPLAY_RECT: c_int = 2;

pub const AVI_DIB: c_int = 0;

pub const NETCAM_NAME_LENGTH: c_int = 14;
pub const NETCAM_SERIAL_LENGTH: c_int = 10;
pub const NETCAM_VERSION_LENGTH: c_int = 9;

pub const IS_CLOSED: c_int = 0;
pub const IS_OPEN: c_int = 1;

pub const IS_STOPPED: c_int = 0;
pub const IS_STARTED: c_int = 1;

pub const EVENT_NEW_FRAME: c_int = 0;
pub const EVENT_DEV_DISCONNECTED: c_int = 1;
pub const EVENT_DEV_RECONNECTED: c_int = 2;
pub const EVENT_USB_TRANSFER_FAILED: c_int = 3;

pub const MODE_TIMEOUT_NONE: c_int = 0;
pub const MODE_TIMEOUT_ALL: c_int = 1;
pub const MODE_TIMEOUT_PART: c_int = 2;

pub const PARAMETER_LOAD_TO_FILE: c_int = 0;
pub const PARAMETER_LOAD_FROM_FILE: c_int = 1;

#[repr(C)]
pub struct PARAM_PROPERTY {
    pub bEnabled: c_int,
    pub bAuto: c_int,
    pub bOnePush: c_int,
    pub nDef: c_uint,
    pub nMin: c_uint,
    pub nMax: c_uint,
}

#[repr(C)]
pub struct PARAM_PROPERTY_f {
    pub bEnabled: c_int,
    pub bAuto: c_int,
    pub bOnePush: c_int,
    pub nDef: c_float,
    pub nMin: c_float,
    pub nMax: c_float,
}

#[repr(C)]
pub struct ROI_PROPERTY {
    pub bEnabled: c_int,
    pub nXRes: c_int,
    pub nYRes: c_int,
    pub nXPos: c_int,
    pub nYPos: c_int,
}

#[repr(C)]
pub struct ROI_RANGE_PROPERTY {
    pub nXMin: c_int,
    pub nXMax: c_int,
    pub nYMin: c_int,
    pub nYMax: c_int,
}

#[repr(C)]
pub struct DISP_PROPERTY {
    pub left: c_int,
    pub top: c_int,
    pub right: c_int,
    pub bottom: c_int,
}

// define HWND as () if not on windows
#[cfg(not(windows))]
pub type HWND = *mut std::ffi::c_void;
#[cfg(windows)]
pub type HWND = *mut std::ffi::c_void; // TODO verify, but should not matter

decl_api! {
    ICubeSDK {
        fn Init() -> c_int;
        fn Open(nCamIndex: c_int) -> c_int;
        fn Close(nCamIndex: c_int) -> c_int;
        fn IsOpen(nCamIndex: c_int) -> c_int;
        fn IsOpenEx(nCamIndex: c_int) -> c_int;

        fn SetCallback(
            nCamIndex: c_int,
            nMode: c_int,
            pCallbackFunc: extern "C" fn(pBuf: *mut u8, lBufferSize: c_long, pContext: *mut c_void) -> c_long,
            pCBContext: *mut c_void,
        ) -> c_int;

        fn SetCallbackEx(
            nCamIndex: c_int,
            nMode: c_int,
            pCallbackFunc: extern "C" fn(event_type: c_int, pBuf: *mut u8, lBufferSize: c_long, pContext: *mut c_void) -> c_long,
            pCBContext: *mut c_void,
        ) -> c_int;

        fn Start(nCamIndex: c_int, ImgHandle: HWND, Preview: c_int, Callback: c_int) -> c_int;
        fn IsStarted(nCamIndex: c_int) -> c_int;
        fn Stop(nCamIndex: c_int) -> c_int;
        fn GetSize(nCamIndex: c_int, pnXRes: *mut c_int, pnYRes: *mut c_int) -> c_int;
        fn GetName(nCamIndex: c_int, Name: *mut c_char) -> c_int;

        fn GetBrokenFrames(nCamIndex: c_int, pnFrames: *mut c_int) -> c_int;
        fn GetGoodFrames(nCamIndex: c_int, pnFrames: *mut c_int) -> c_int;
        
        fn SetDisplayMode(nCamIndex: c_int, nMode: c_int, property: DISP_PROPERTY) -> c_int;

        //fn GetVersion(nType: c_int, pVersion: *mut c_char) -> c_void; sse https://github.com/LucaCiucci/icube-sdk-sys/issues/1
        fn GetFWVersion(nCamIndex: c_int, pVersion: *mut c_char) -> c_int;
        fn GetSerialNum(nCamIndex: c_int, pVersion: *mut c_char) -> c_int;
        fn GetFGPAVersion(nCamIndex: c_int, pVersion: *mut c_char) -> c_int;

        fn SetResolution(nCamIndex: c_int, property: *mut ROI_PROPERTY) -> c_int; // mut???
        fn GetResolution(nCamIndex: c_int, property: *mut ROI_PROPERTY) -> c_int;
        fn GetResolutionRange(nCamIndex: c_int, property: *mut ROI_RANGE_PROPERTY) -> c_int;
        
        fn SetResolutionParam(nCamIndex: c_int, ImgHandle: HWND, Preview: c_int, Callback: c_int) -> c_int;


        fn SetMode(nCamIndex: c_int, nMode: c_int) -> c_int;
        fn GetMode(nCamIndex: c_int, pMode: *mut c_int) -> c_int;
        fn GetModeList(nCamIndex: c_int, pLength: *mut c_int, pList: *mut c_int) -> c_int;

        fn SetBinSkip(nCamIndex: c_int, nParameter: c_int, nMode: c_int) -> c_int;
        fn GetBinSkip(nCamIndex: c_int, nParameter: *mut c_int, nMode: c_int) -> c_int;
        fn GetBinSkipList(nCamIndex: c_int, nMode: c_int, pLenght: *mut c_int, pList: *mut c_int) -> c_int;

        fn SaveToFile(nCamIndex: c_int, Name: *const c_char) -> c_int;
        fn SaveAvi(nCamIndex: c_int, Name: *const c_char, nMode: c_int, nTimeInSecs: c_int) -> c_int;

        fn SetTrigger(nCamIndex: c_int, nMode: c_int) -> c_int;
        fn GetTrigger(nCamIndex: c_int, nMode: *mut c_int) -> c_int;

        fn SetCamParameter(nCamIndex: c_int, Type: c_int, Value: c_ulong) -> c_int;
        fn GetCamParameter(nCamIndex: c_int, Type: c_int, Value: *mut c_ulong) -> c_int;
        fn GetCamParameterRange(nCamIndex: c_int, Type: c_int, property: *mut PARAM_PROPERTY) -> c_int;
        fn CamParameterSet(nCamIndex: c_int, nCmd: c_int, nParam: *mut c_void) -> c_int; // mut???

        fn SetExposure(nCamIndex: c_int, Value: c_float) -> c_int;
        fn GetExposure(nCamIndex: c_int, Value: *mut c_float) -> c_int;
        fn GetExposureRange(nCamIndex: c_int, property: *mut PARAM_PROPERTY_f) -> c_int;

        fn GetParamAuto(nCamIndex: c_int, Type: c_int, bAuto: *mut c_int) -> c_int;
        fn SetParamAuto(nCamIndex: c_int, Type: c_int, bAuto: c_int) -> c_int;
        fn SetParamAutoDef(nCamIndex: c_int, Type: c_int) -> c_int;
        fn SetParamOnePush(nCamIndex: c_int, Type: c_int) -> c_int;
    }
}

impl API {

    /// Check that the version of the SDK is correct.
    ///
    /// This ffi were provided for the 2.0.x.x version of the SDK
    pub fn check_version(&self) -> bool {
        // This check is disabled since GetVersion is broken and crashes the program
        // see https://github.com/LucaCiucci/icube-sdk-sys/issues/1

        /*
        let mut version = [0; 32 + 100];
        unsafe { (self.GetVersion)((version.len() as i32 - 10) as _, version.as_mut_ptr() as *mut _) };

        let version = unsafe { CStr::from_ptr(version.as_ptr()) }.to_string_lossy();

        if !version.starts_with("2.0") {
            log::error!("The version of the SDK is not 2.0.x.x: {}", version);
            return false;
        }
        */

        true
    }
}