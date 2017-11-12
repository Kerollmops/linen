// extern crate opencl_sys;
extern crate cl_sys as opencl_sys;

pub mod platform;
pub mod device;
pub mod extensions;
pub mod built_in_kernels;
pub mod profile;

pub use platform::Platform;
pub use device::Device;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
