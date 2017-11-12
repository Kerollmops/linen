use std::str::Split;

#[derive(Debug, Clone)]
pub struct BuiltInKernels {
    pub(crate) inner: String
}

impl BuiltInKernels {
    pub fn iter(&self) -> Split<char> {
        self.inner.split(';')
    }
}
