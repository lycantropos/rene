pub(super) struct Reference<T> {
    _python_value: pyo3::PyObject,
    rust_ptr: *const T,
}

unsafe impl<T> Send for Reference<T> {}

impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self {
            _python_value: unsafe {
                pyo3::Python::with_gil_unchecked(|py| {
                    self._python_value.clone_ref(py)
                })
            },
            rust_ptr: self.rust_ptr,
        }
    }
}

impl<T> std::ops::Deref for Reference<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rust_ptr }
    }
}

impl<T> Reference<T> {
    pub(super) fn from_py_ref(value: pyo3::PyRef<'_, T>) -> Self
    where
        T: pyo3::PyClass,
    {
        Reference {
            _python_value: unsafe {
                pyo3::Bound::from_borrowed_ptr(
                    value.py(),
                    pyo3::AsPyPointer::as_ptr(&value),
                )
            }
            .unbind(),
            rust_ptr: std::ops::Deref::deref(&value) as *const T,
        }
    }
}
