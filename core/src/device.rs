use std::ptr;
use opencl_sys::{cl_device_id, cl_device_type, cl_device_info, cl_platform_id};
use opencl_sys::{CL_SUCCESS, CL_DEVICE_NOT_FOUND};
use opencl_sys::{
    CL_DEVICE_TYPE_CPU,
    CL_DEVICE_TYPE_GPU,
    CL_DEVICE_TYPE_ACCELERATOR,
    CL_DEVICE_TYPE_CUSTOM,
    CL_DEVICE_TYPE_DEFAULT,
    CL_DEVICE_TYPE_ALL,
};
use opencl_sys::clGetDeviceIDs;
use platform::Platform;

fn all_device_ids(platform_id: cl_platform_id, type_: cl_device_type) -> Vec<cl_device_id> {
    let mut num_devices = 0;

    let ret = unsafe {
        clGetDeviceIDs(
            platform_id,
            type_,
            0,
            ptr::null_mut(),
            &mut num_devices)
    };

    if num_devices == 0 || ret == CL_DEVICE_NOT_FOUND {
        return Vec::new()
    }

    // TODO: manage CL_OUT_OF_RESOURCES
    assert_eq!(ret, CL_SUCCESS);

    let mut devices = Vec::with_capacity(num_devices as usize);

    let ret = unsafe {
        clGetDeviceIDs(
            platform_id,
            type_,
            num_devices,
            devices.as_mut_ptr(),
            ptr::null_mut()
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    unsafe { devices.set_len(num_devices as usize) };

    devices
}

fn first_device_id(platform_id: cl_platform_id, type_: cl_device_type) -> Option<cl_device_id> {
    let mut device_id = ptr::null_mut();
    let mut num_devices = 0;

    let ret = unsafe {
        clGetDeviceIDs(
            platform_id,
            type_,
            1,
            &mut device_id,
            &mut num_devices)
    };

    if num_devices == 0 || ret == CL_DEVICE_NOT_FOUND {
        None
    } else {
        assert_eq!(ret, CL_SUCCESS);
        Some(device_id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Cpu,
    Gpu,
    Accelerator,
    Custom,
    Default,
    All,
}

impl From<Type> for cl_device_type {
    fn from(value: Type) -> Self {
        match value {
            Type::Cpu => CL_DEVICE_TYPE_CPU,
            Type::Gpu => CL_DEVICE_TYPE_GPU,
            Type::Accelerator => CL_DEVICE_TYPE_ACCELERATOR,
            Type::Custom => CL_DEVICE_TYPE_CUSTOM,
            Type::Default => CL_DEVICE_TYPE_DEFAULT,
            Type::All => CL_DEVICE_TYPE_ALL,
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub(crate) id: cl_device_id,
}

impl Device {
    pub fn all(platform: &Platform) -> Vec<Device> {
        let devices = all_device_ids(platform.id, Type::All.into());
        devices.into_iter().map(|id| Device { id }).collect()
    }

    pub fn default(platform: &Platform) -> Option<Device> {
        let device = first_device_id(platform.id, Type::Default.into());
        device.map(|id| Device { id })
    }
}
