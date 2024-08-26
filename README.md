# icube-sdk-sys
 Bindings for the iCube SDK C API

> ⚠️ This is not an official [https://net-gmbh.com](https://net-gmbh.com/) project

## Usage

See the `examples` directory for usage examples.

## Documentation

See the official documentation shipped with the SDK.

## Runtime and SDK

Download from: <https://net-gmbh.com/en/machine-vision/products/cameras/usb2-icube/>

Unpack the correct version and you will find:
- driver
- SDK
- documentation

## Linux

To install the driver, use dpkg, for example:
```bash
dpkg --install netusbcam_1.39-1_amd64_libudev.deb
```

When loading the library, you may encounter and error telling that `libudev.so.0` is missing. To fix this, install the `libudev0` package:
```bash
sudo apt-get install libudev0
```

### Troubleshooting

You may encounter a permission error, this because `libNETUSBCAM*.so` uses `libusb` to access the device directly.  
The easy way to fix this is to allow the current user to access the device (NETUSBCAM).

Create `/etc/udev/rules.d/99-netusbcam.rules` with the following content:
```plain
SUBSYSTEM=="usb", ATTR{idVendor}=="152a", ATTR{idProduct}=="*", MODE="0666", GROUP="plugdev"
```
You may want to verify the `idVendor` and `idProduct` by running `lsusb`.