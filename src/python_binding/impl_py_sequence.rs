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
                        std::iter::Skip<std::slice::Iter<'_, $value_type>>,
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

        #[pyo3::pymethods]
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
                start: Option<&pyo3::Bound<'_, pyo3::types::PyInt>>,
                stop: Option<&pyo3::Bound<'_, pyo3::types::PyInt>>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<usize> {
                use pyo3::types::PyAnyMethods;
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
                            None => pyo3::intern!(py, "0").clone(),
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

            fn __getitem__<'py>(
                &self,
                item: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                use pyo3::types::PyAnyMethods;
                if item.is_instance(&<pyo3::types::PySlice as pyo3::type_object::PyTypeInfo>::type_object(py))? {
                    let (start, stop, step) = super::slicing::to_next_slice_indices(
                        self.start,
                        self.step,
                        self.len(),
                        item.extract::<pyo3::Bound<'_, pyo3::types::PySlice>>()?,
                    )?;
                    pyo3::IntoPyObject::into_pyobject(
                        Self {
                            $container_field: self.$container_field.clone(),
                            start,
                            stop,
                            step,
                        },
                        py,
                    ).map(pyo3::Bound::<'_, _>::into_any)
                } else {
                    let maybe_index =
                        unsafe { pyo3::ffi::PyNumber_Index(pyo3::AsPyPointer::as_ptr(item)) };
                    if maybe_index.is_null() {
                        Err(pyo3::PyErr::fetch(item.py()))
                    } else {
                        let index = super::slicing::py_long_to_valid_index(
                            unsafe {
                                pyo3::Bound::<'_, pyo3::PyAny>::from_owned_ptr(
                                    item.py(),
                                    maybe_index,
                                )
                                .extract::<pyo3::Bound<'_, pyo3::types::PyInt>>()?
                            },
                            self.len(),
                        )?;
                        pyo3::IntoPyObject::into_pyobject(
                            <$py_value_type>::from(
                                unsafe {
                                    self.iter().nth(index).unwrap_unchecked()
                                }
                                .clone(),
                            ),
                            py,
                        ).map(pyo3::Bound::<'_, _>::into_any)
                    }
                }
            }

            fn __hash__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<isize> {
                pyo3::types::PyAnyMethods::hash(
                    <pyo3::types::PyTuple>::new(
                        py,
                        self.iter()
                            .cloned()
                            .map(|element| <$py_value_type>::from(element))
                            .collect::<Vec<_>>(),
                    )?
                    .as_ref(),
                )
            }

            fn __len__(&self) -> usize {
                self.len()
            }

            fn __richcmp__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    match op {
                        pyo3::basic::CompareOp::Eq => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(
                                    self.iter().eq(other.iter()),
                                    py,
                                )
                                .unwrap(),
                            )
                            .into_any()
                            .unbind())
                        }
                        pyo3::basic::CompareOp::Ne => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(
                                    self.iter().ne(other.iter()),
                                    py,
                                )
                                .unwrap(),
                            )
                            .into_any()
                            .unbind())
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
