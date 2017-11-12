use std::ptr;
use std::ffi::CString;
use opencl_sys::{cl_platform_id, cl_platform_info, c_void};
use opencl_sys::{
    CL_SUCCESS,
    CL_PLATFORM_PROFILE,
    CL_PLATFORM_VERSION,
    CL_PLATFORM_NAME,
    CL_PLATFORM_VENDOR,
    CL_PLATFORM_EXTENSIONS,
};
use opencl_sys::{clGetPlatformIDs, clGetPlatformInfo};
use extensions::Extensions;
use profile::Profile;
use device::Device;

fn all_platform_ids() -> Vec<cl_platform_id> {
    let mut num_platforms = 0;

    let ret = unsafe { clGetPlatformIDs(0, ptr::null_mut(), &mut num_platforms) };
    assert_eq!(ret, CL_SUCCESS);

    if num_platforms == 0 {
        return Vec::new()
    }

    let mut platforms = Vec::with_capacity(num_platforms as usize);

    let ret = unsafe {
        clGetPlatformIDs(
            num_platforms,
            platforms.as_mut_ptr(),
            ptr::null_mut()
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    unsafe { platforms.set_len(num_platforms as usize) };

    platforms
}

fn first_platform_id() -> Option<cl_platform_id> {
    let mut platform_id = ptr::null_mut();
    let mut num_platforms = 0;

    let ret = unsafe {
        clGetPlatformIDs(
            1,
            &mut platform_id,
            &mut num_platforms
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    if num_platforms == 1 {
        Some(platform_id)
    } else {
        None
    }
}

fn platform_info(platform_id: cl_platform_id, info: cl_platform_info) -> CString {
    let mut value_size = 0;

    let ret = unsafe {
        clGetPlatformInfo(
            platform_id,
            info,
            0,
            ptr::null_mut(),
            &mut value_size,
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    let mut value: Vec<u8> = Vec::with_capacity(value_size);

    let ret = unsafe {
        clGetPlatformInfo(
            platform_id,
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
pub struct Platform {
    pub(crate) id: cl_platform_id,
}

impl Platform {
    pub fn all() -> Vec<Platform> {
        all_platform_ids().into_iter().map(|id| Platform { id }).collect()
    }

    pub fn first() -> Option<Platform> {
        first_platform_id().map(|id| Platform { id })
    }

    pub fn all_devices(&self) -> Vec<Device> {
        Device::all(self)
    }

    pub fn default_device(&self) -> Option<Device> {
        Device::default(self)
    }

    pub fn profile(&self) -> Profile {
        let value = platform_info(self.id, CL_PLATFORM_PROFILE);
        let value = value.into_string().unwrap();
        Profile::from(value.as_str())
    }

    pub fn version() -> () {
        // OpenCL<space><major_version.minor_version><space><platform-specific information>
        unimplemented!()
    }

    pub fn name(&self) -> String {
        let value = platform_info(self.id, CL_PLATFORM_NAME);
        value.into_string().unwrap()
    }

    pub fn vendor(&self) -> String {
        let value = platform_info(self.id, CL_PLATFORM_VENDOR);
        value.into_string().unwrap()
    }

    pub fn extensions(&self) -> Extensions {
        let value = platform_info(self.id, CL_PLATFORM_EXTENSIONS);
        Extensions {
            inner: value.into_string().unwrap()
        }
    }
}
