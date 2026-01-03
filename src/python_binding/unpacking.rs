pub(super) fn try_unpack_maybe_empty_polygons<
    'py,
    Empty: Default + pyo3::IntoPyObject<'py, Error = Error>,
    Multipolygon: From<Vec<Polygon>> + pyo3::IntoPyObject<'py, Error = Error>,
    Polygon: pyo3::IntoPyObject<'py, Error = Error>,
    Error,
>(
    polygons: Vec<Polygon>,
    py: pyo3::Python<'py>,
) -> Result<pyo3::Py<pyo3::PyAny>, Error> {
    match polygons.len() {
        0 => pyo3::IntoPyObject::into_pyobject(Empty::default(), py)
            .map(pyo3::BoundObject::into_bound)
            .map(pyo3::Bound::into_any),
        1 => pyo3::IntoPyObject::into_pyobject(
            unsafe { polygons.into_iter().next().unwrap_unchecked() },
            py,
        )
        .map(pyo3::BoundObject::into_bound)
        .map(pyo3::Bound::into_any),
        _ => {
            pyo3::IntoPyObject::into_pyobject(Multipolygon::from(polygons), py)
                .map(pyo3::BoundObject::into_bound)
                .map(pyo3::Bound::into_any)
        }
    }
    .map(pyo3::Bound::unbind)
}

pub(super) fn try_unpack_maybe_empty_segments<
    'py,
    Empty: Default + pyo3::IntoPyObject<'py, Error = Error>,
    Multisegment: From<Vec<Segment>> + pyo3::IntoPyObject<'py, Error = Error>,
    Segment: pyo3::IntoPyObject<'py, Error = Error>,
    Error,
>(
    segments: Vec<Segment>,
    py: pyo3::Python<'py>,
) -> Result<pyo3::Py<pyo3::PyAny>, Error> {
    match segments.len() {
        0 => pyo3::IntoPyObject::into_pyobject(Empty::default(), py)
            .map(pyo3::BoundObject::into_bound)
            .map(pyo3::Bound::into_any),
        1 => pyo3::IntoPyObject::into_pyobject(
            unsafe { segments.into_iter().next().unwrap_unchecked() },
            py,
        )
        .map(pyo3::BoundObject::into_bound)
        .map(pyo3::Bound::into_any),
        _ => {
            pyo3::IntoPyObject::into_pyobject(Multisegment::from(segments), py)
                .map(pyo3::BoundObject::into_bound)
                .map(pyo3::Bound::into_any)
        }
    }
    .map(pyo3::Bound::unbind)
}

pub(super) fn try_unpack_non_empty_polygons<
    'py,
    Multipolygon: From<Vec<Polygon>> + pyo3::IntoPyObject<'py, Error = Error>,
    Polygon: pyo3::IntoPyObject<'py, Error = Error>,
    Error,
>(
    polygons: Vec<Polygon>,
    py: pyo3::Python<'py>,
) -> Result<pyo3::Py<pyo3::PyAny>, Error> {
    match polygons.len() {
        0 => unreachable!("Expected to be non-empty."),
        1 => pyo3::IntoPyObject::into_pyobject(
            unsafe { polygons.into_iter().next().unwrap_unchecked() },
            py,
        )
        .map(pyo3::BoundObject::into_bound)
        .map(pyo3::Bound::into_any),
        _ => {
            pyo3::IntoPyObject::into_pyobject(Multipolygon::from(polygons), py)
                .map(pyo3::BoundObject::into_bound)
                .map(pyo3::Bound::into_any)
        }
    }
    .map(pyo3::Bound::unbind)
}

pub(super) fn try_unpack_non_empty_segments<
    'py,
    Multisegment: From<Vec<Segment>> + pyo3::IntoPyObject<'py, Error = Error>,
    Segment: pyo3::IntoPyObject<'py, Error = Error>,
    Error,
>(
    segments: Vec<Segment>,
    py: pyo3::Python<'py>,
) -> Result<pyo3::Py<pyo3::PyAny>, Error> {
    match segments.len() {
        0 => unreachable!("Expected to be non-empty."),
        1 => pyo3::IntoPyObject::into_pyobject(
            unsafe { segments.into_iter().next().unwrap_unchecked() },
            py,
        )
        .map(pyo3::BoundObject::into_bound)
        .map(pyo3::Bound::into_any),
        _ => {
            pyo3::IntoPyObject::into_pyobject(Multisegment::from(segments), py)
                .map(pyo3::BoundObject::into_bound)
                .map(pyo3::Bound::into_any)
        }
    }
    .map(pyo3::Bound::unbind)
}
