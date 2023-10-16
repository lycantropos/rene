#[derive(Clone)]
pub(super) struct Reference<T> {
    _python_value: pyo3::PyObject,
    rust_ptr: *const T,
}

unsafe impl<T> Send for Reference<T> {}

impl<T> std::ops::Deref for Reference<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rust_ptr }
    }
}

impl<T> Reference<T> {
    pub(super) fn from_py_ref(value: pyo3::PyRef<T>) -> Self
    where
        T: pyo3::PyClass,
    {
        Reference {
            _python_value: unsafe {
                pyo3::PyObject::from_borrowed_ptr(
                    value.py(),
                    pyo3::AsPyPointer::as_ptr(&value),
                )
            },
            rust_ptr: std::ops::Deref::deref(&value) as *const T,
        }
    }
}
