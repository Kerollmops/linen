use opencl_sys::cl_context;
use opencl_sys::clCreateContext;
use device::Device;

#[derive(Debug)]
pub struct Context {
    pub(crate) inner: cl_context,
}

impl Context {
    pub fn simple(device: &Device) -> Self {
        unimplemented!()
    }

    pub fn new<'a, I>(devices: I) -> Self
        where I: Iterator<Item=&'a Device>
    {
        unimplemented!()
    }
}
