use std::ptr;
use opencl_sys::cl_platform_id;
use opencl_sys::CL_SUCCESS;
use opencl_sys::clGetPlatformIDs;

fn all_platform_ids() -> Vec<cl_platform_id> {
    let mut num_platforms = 0;

    let ret = unsafe { clGetPlatformIDs(0, ptr::null_mut(), &mut num_platforms) };
    assert_eq!(ret, CL_SUCCESS);

    if num_platforms == 0 {
        return Vec::new()
    }

    let mut platforms: Vec<cl_platform_id> = Vec::with_capacity(num_platforms as usize);

    let ret = unsafe {
        clGetPlatformIDs(
            num_platforms,
            platforms.as_mut_ptr() as *mut cl_platform_id,
            ptr::null_mut()
        )
    };
    assert_eq!(ret, CL_SUCCESS);

    unsafe { platforms.set_len(num_platforms as usize) };

    platforms
}

#[derive(Debug)]
pub struct Platform {
    id: cl_platform_id,
}

impl Platform {
    pub fn all() -> Vec<Platform> {
        let platform_ids = all_platform_ids();
        platform_ids.into_iter().map(|id| Platform { id }).collect()
    }
}
