use std::ptr;
use opencl_sys::{cl_mem, c_void};
use opencl_sys::CL_SUCCESS;
use opencl_sys::clCreateBuffer;
use context::Context;

#[derive(Debug)]
pub struct Buffer {
    inner: cl_mem,
}

impl Buffer {
    pub fn uninitialized(context: &Context, size: usize, flags: ()) -> Self {
        assert!(size > 0);

        let mut error = 0;

        let buffer = unsafe {
            clCreateBuffer(
                context.inner,
                unimplemented!(),
                size,
                ptr::null_mut(),
                &mut error,
            )
        };
        // TODO: CL_MEM_OBJECT_ALLOCATION_FAILURE, CL_OUT_OF_RESOURCES
        assert_eq!(error, CL_SUCCESS);

        Self {
            inner: buffer
        }
    }

    pub fn from_slice(context: &Context, slice: &[u8], flags: ()) -> Self {
        assert!(slice.len() > 0);

        let mut error = 0;

        let buffer = unsafe {
            clCreateBuffer(
                context.inner,
                unimplemented!(),
                slice.len(),
                slice.as_ptr() as *mut c_void,
                &mut error,
            )
        };
        // TODO: CL_MEM_OBJECT_ALLOCATION_FAILURE, CL_OUT_OF_RESOURCES
        assert_eq!(error, CL_SUCCESS);

        Self {
            inner: buffer
        }
    }
}
