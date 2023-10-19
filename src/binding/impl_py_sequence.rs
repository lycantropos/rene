macro_rules! impl_py_sequence {
    (
        $py_sequence: ident,
        $container_field:ident,
        $value_name: ident,
        $values_method: ident,
        $py_value_type: ty,
        $value_type: ty
    ) => {
        impl $py_sequence {
            fn iter(
                &self,
            ) -> super::generic_iterator::GenericIterator<
                std::iter::Take<
                    std::iter::StepBy<
                        std::iter::Skip<std::slice::Iter<$value_type>>,
                    >,
                >,
            > {
                if self.step > 0 {
                    super::generic_iterator::GenericIterator::Forward(
                        (&std::ops::Deref::deref(&self.$container_field).0)
                            .$values_method()
                            .into_iter()
                            .skip(self.start as usize)
                            .step_by(self.step as usize)
                            .take(self.len()),
                    )
                } else {
                    let elements_count = self.len();
                    super::generic_iterator::GenericIterator::Backward(
                        (&std::ops::Deref::deref(&self.$container_field).0)
                            .$values_method()
                            .into_iter()
                            .skip(
                                (self.start
                                    + ((elements_count as isize) - 1)
                                        * self.step)
                                    as usize,
                            )
                            .step_by((-self.step) as usize)
                            .take(elements_count)
                            .rev(),
                    )
                }
            }

            fn len(&self) -> usize {
                (if self.step > 0 && self.start < self.stop {
                    1 + (self.stop - self.start - 1) / self.step
                } else if self.step < 0 && self.start > self.stop {
                    1 + (self.start - self.stop - 1) / (-self.step)
                } else {
                    0
                }) as usize
            }
        }

        #[pyo3::prelude::pymethods]
        impl $py_sequence {
            #[pyo3(signature = ($value_name, /))]
            fn count(&self, $value_name: &$py_value_type) -> usize {
                self.iter()
                    .filter(|&candidate| candidate == &$value_name.0)
                    .count()
            }

            #[pyo3(signature = ($value_name, start = None, stop = None, /))]
            fn index(
                &self,
                $value_name: &$py_value_type,
                start: Option<&pyo3::types::PyLong>,
                stop: Option<&pyo3::types::PyLong>,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<usize> {
                match {
                    let elements_count = self.len();
                    let start = super::slicing::normalize_index_start(start, elements_count);
                    let stop = super::slicing::normalize_index_stop(stop, elements_count);
                    self.iter()
                        .skip(start)
                        .take(stop.saturating_sub(start))
                        .position(|candidate| candidate.eq(&$value_name.0))
                        .map(|offset| start + offset)
                }
                {
                    Some(result) => Ok(result),
                    None => Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "{} is not found among {} {} with indices from range({}, {}).",
                        $value_name.__repr__(py)?,
                        stringify!($container_field),
                        stringify!($values_method),
                        match start {
                            Some(start) => start.repr()?,
                            None => pyo3::intern!(py, "0"),
                        },
                        match stop {
                            Some(stop) => stop.repr()?,
                            None =>
                                pyo3::types::PyString::new(py, &self.len().to_string()),
                        }
                    ))),
                }
            }

            fn __contains__(&self, value: &$py_value_type) -> bool {
                self.iter().contains(&value.0)
            }

            fn __getitem__(
                &self,
                item: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                if item.is_instance(<pyo3::types::PySlice as pyo3::type_object::PyTypeInfo>::type_object(py))? {
                    let (start, stop, step) = super::slicing::to_next_slice_indices(
                        self.start,
                        self.step,
                        self.len(),
                        item.extract::<&pyo3::types::PySlice>()?,
                    )?;
                    Ok(pyo3::IntoPy::into_py(
                        Self {
                            $container_field: self.$container_field.clone(),
                            start,
                            stop,
                            step,
                        },
                        py,
                    ))
                } else {
                    let maybe_index =
                        unsafe { pyo3::ffi::PyNumber_Index(pyo3::AsPyPointer::as_ptr(item)) };
                    if maybe_index.is_null() {
                        Err(pyo3::PyErr::fetch(item.py()))
                    } else {
                        let index = super::slicing::py_long_to_valid_index(
                            unsafe {
                                <pyo3::types::PyLong as pyo3::FromPyPointer>::from_owned_ptr(item.py(), maybe_index)
                            },
                            self.len(),
                        )?;
                        Ok(pyo3::IntoPy::into_py(
                            unsafe {
                                self.iter().nth(index).unwrap_unchecked()
                            }
                            .clone(),
                            py,
                        ))
                    }
                }
            }

            fn __hash__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<pyo3::ffi::Py_hash_t> {
                <pyo3::types::PyTuple>::new(py, self.iter().collect::<Vec<_>>()).hash()
            }

            fn __len__(&self) -> usize {
                self.len()
            }

            fn __richcmp__(
                &self,
                other: &pyo3::PyAny,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(<Self as pyo3::type_object::PyTypeInfo>::type_object(py))? {
                    let other = other.extract::<pyo3::PyRef<Self>>()?;
                    match op {
                        pyo3::basic::CompareOp::Eq => {
                            Ok(pyo3::IntoPy::into_py(self.iter().eq(other.iter()), py))
                        }
                        pyo3::basic::CompareOp::Ne => {
                            Ok(pyo3::IntoPy::into_py(self.iter().ne(other.iter()), py))
                        }
                        _ => Ok(py.NotImplemented()),
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

pub(super) use impl_py_sequence;
