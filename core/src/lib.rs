// extern crate opencl_sys;
extern crate cl_sys as opencl_sys;

pub mod platform;

pub use platform::Platform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
