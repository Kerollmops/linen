use std::ptr;
use std::ffi::CString;
use opencl_sys::{cl_device_id, cl_device_type, cl_device_info, cl_platform_id, c_void};
use opencl_sys::{CL_SUCCESS, CL_DEVICE_NOT_FOUND};
use opencl_sys::{
    CL_DEVICE_TYPE_CPU,
    CL_DEVICE_TYPE_GPU,
    CL_DEVICE_TYPE_ACCELERATOR,
    CL_DEVICE_TYPE_CUSTOM,
    CL_DEVICE_TYPE_DEFAULT,
    CL_DEVICE_TYPE_ALL,
};
use opencl_sys::{
    // CL_DEVICE_ADDRESS_BITS,
    // CL_DEVICE_AVAILABLE,
    // CL_DEVICE_BUILT_IN_KERNELS,
    // CL_DEVICE_COMPILER_AVAILABLE,
    // CL_DEVICE_DOUBLE_FP_CONFIG,
    // CL_DEVICE_ENDIAN_LITTLE,
    // CL_DEVICE_ERROR_CORRECTION_SUPPORT,
    // CL_DEVICE_EXECUTION_CAPABILITIES,
    CL_DEVICE_EXTENSIONS,
    // CL_DEVICE_GLOBAL_MEM_CACHE_SIZE,
    // CL_DEVICE_GLOBAL_MEM_CACHE_TYPE,
    // CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE,
    // CL_DEVICE_GLOBAL_MEM_SIZE,
    // CL_DEVICE_HALF_FP_CONFIG,
    // CL_DEVICE_HOST_UNIFIED_MEMORY,
    // CL_DEVICE_IMAGE_SUPPORT,
    // CL_DEVICE_IMAGE2D_MAX_HEIGHT,
    // CL_DEVICE_IMAGE2D_MAX_WIDTH,
    // CL_DEVICE_IMAGE3D_MAX_DEPTH,
    // CL_DEVICE_IMAGE3D_MAX_HEIGHT,
    // CL_DEVICE_IMAGE3D_MAX_WIDTH,
    // CL_DEVICE_IMAGE_MAX_BUFFER_SIZE,
    // CL_DEVICE_IMAGE_MAX_ARRAY_SIZE,
    // CL_DEVICE_LINKER_AVAILABLE,
    // CL_DEVICE_LOCAL_MEM_SIZE,
    // CL_DEVICE_LOCAL_MEM_TYPE,
    // CL_DEVICE_MAX_CLOCK_FREQUENCY,
    // CL_DEVICE_MAX_COMPUTE_UNITS,
    // CL_DEVICE_MAX_CONSTANT_ARGS,
    // CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE,
    // CL_DEVICE_MAX_MEM_ALLOC_SIZE,
    // CL_DEVICE_MAX_PARAMETER_SIZE,
    // CL_DEVICE_MAX_READ_IMAGE_ARGS,
    // CL_DEVICE_MAX_SAMPLERS,
    // CL_DEVICE_MAX_WORK_GROUP_SIZE,
    // CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS,
    // CL_DEVICE_MAX_WORK_ITEM_SIZES,
    // CL_DEVICE_MAX_WRITE_IMAGE_ARGS,
    // CL_DEVICE_MEM_BASE_ADDR_ALIGN,
    // CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE,
    CL_DEVICE_NAME,
    // CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT,
    // CL_DEVICE_OPENCL_C_VERSION,
    // CL_DEVICE_PARENT_DEVICE,
    // CL_DEVICE_PARTITION_MAX_SUB_DEVICES,
    // CL_DEVICE_PARTITION_PROPERTIES,
    // CL_DEVICE_PARTITION_AFFINITY_DOMAIN,
    // CL_DEVICE_PARTITION_TYPE,
    // CL_DEVICE_PLATFORM,
    // CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT,
    // CL_DEVICE_PRINTF_BUFFER_SIZE,
    // CL_DEVICE_PREFERRED_INTEROP_USER_SYNC,
    // CL_DEVICE_PROFILE,
    // CL_DEVICE_PROFILING_TIMER_RESOLUTION,
    // CL_DEVICE_QUEUE_PROPERTIES,
    // CL_DEVICE_REFERENCE_COUNT,
    // CL_DEVICE_SINGLE_FP_CONFIG,
    // CL_DEVICE_TYPE,
    CL_DEVICE_VENDOR,
    // CL_DEVICE_VENDOR_ID,
    // CL_DEVICE_VERSION,
    // CL_DRIVER_VERSION,
};
use opencl_sys::{clGetDeviceIDs, clGetDeviceInfo};
use platform::Platform;
use extensions::Extensions;

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

fn device_info_as_cstring(device_id: cl_device_id, info: cl_device_info) -> CString {
    let mut value_size = 0;

    let ret = unsafe {
        clGetDeviceInfo(
            device_id,
            info,
            0,
            ptr::null_mut(),
            &mut value_size,
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    let mut value: Vec<u8> = Vec::with_capacity(value_size);

    let ret = unsafe {
        clGetDeviceInfo(
            device_id,
            info,
            value_size,
            value.as_mut_ptr() as *mut c_void,
            ptr::null_mut(),
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    unsafe { value.set_len(value_size - 1) }; // ignore trailing '\0'

    CString::new(value).unwrap()
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

    pub fn version(&self) -> () {
        unimplemented!()
    }

    pub fn name(&self) -> String {
        let value = device_info_as_cstring(self.id, CL_DEVICE_NAME);
        value.into_string().unwrap()
    }

    pub fn vendor(&self) -> String {
        let value = device_info_as_cstring(self.id, CL_DEVICE_VENDOR);
        value.into_string().unwrap()
    }

    pub fn extensions(&self) -> Extensions {
        let value = device_info_as_cstring(self.id, CL_DEVICE_EXTENSIONS);
        Extensions {
            inner: value.into_string().unwrap()
        }
    }
}
