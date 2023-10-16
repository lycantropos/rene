pub(super) fn unpack_maybe_empty_polygons<
    Empty: Default + pyo3::IntoPy<pyo3::PyObject>,
    Multipolygon: From<Vec<Polygon>> + pyo3::IntoPy<pyo3::PyObject>,
    Polygon: pyo3::IntoPy<pyo3::PyObject>,
>(
    polygons: Vec<Polygon>,
    py: pyo3::Python,
) -> pyo3::PyObject {
    match polygons.len() {
        0 => pyo3::IntoPy::into_py(Empty::default(), py),
        1 => pyo3::IntoPy::into_py(
            unsafe { polygons.into_iter().next().unwrap_unchecked() },
            py,
        ),
        _ => pyo3::IntoPy::into_py(Multipolygon::from(polygons), py),
    }
}

pub(super) fn unpack_maybe_empty_segments<
    Empty: Default + pyo3::IntoPy<pyo3::PyObject>,
    Multisegment: From<Vec<Segment>> + pyo3::IntoPy<pyo3::PyObject>,
    Segment: pyo3::IntoPy<pyo3::PyObject>,
>(
    segments: Vec<Segment>,
    py: pyo3::Python,
) -> pyo3::PyObject {
    match segments.len() {
        0 => pyo3::IntoPy::into_py(Empty::default(), py),
        1 => pyo3::IntoPy::into_py(
            unsafe { segments.into_iter().next().unwrap_unchecked() },
            py,
        ),
        _ => pyo3::IntoPy::into_py(Multisegment::from(segments), py),
    }
}

pub(super) fn unpack_non_empty_polygons<
    Multipolygon: From<Vec<Polygon>> + pyo3::IntoPy<pyo3::PyObject>,
    Polygon: pyo3::IntoPy<pyo3::PyObject>,
>(
    polygons: Vec<Polygon>,
    py: pyo3::Python,
) -> pyo3::PyObject {
    match polygons.len() {
        0 => unreachable!("Expected to be non-empty."),
        1 => pyo3::IntoPy::into_py(
            unsafe { polygons.into_iter().next().unwrap_unchecked() },
            py,
        ),
        _ => pyo3::IntoPy::into_py(Multipolygon::from(polygons), py),
    }
}

pub(super) fn unpack_non_empty_segments<
    Multisegment: From<Vec<Segment>> + pyo3::IntoPy<pyo3::PyObject>,
    Segment: pyo3::IntoPy<pyo3::PyObject>,
>(
    segments: Vec<Segment>,
    py: pyo3::Python,
) -> pyo3::PyObject {
    match segments.len() {
        0 => unreachable!("Expected to be non-empty."),
        1 => pyo3::IntoPy::into_py(
            unsafe { segments.into_iter().next().unwrap_unchecked() },
            py,
        ),
        _ => pyo3::IntoPy::into_py(Multisegment::from(segments), py),
    }
}
