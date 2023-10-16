use crate::constants::{
    MIN_CONTOUR_VERTICES_COUNT, MIN_MULTIPOLYGON_POLYGONS_COUNT,
    MIN_MULTISEGMENT_SEGMENTS_COUNT,
};

pub(super) fn try_pack_polygons<Multipolygon: From<Vec<Polygon>>, Polygon>(
    polygons: Vec<Polygon>,
) -> pyo3::PyResult<Multipolygon> {
    if polygons.len() < MIN_MULTIPOLYGON_POLYGONS_COUNT {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Multipolygon should have at least {} polygons, but found {}.",
            MIN_MULTIPOLYGON_POLYGONS_COUNT,
            polygons.len()
        )))
    } else {
        Ok(Multipolygon::from(polygons))
    }
}

pub(super) fn try_pack_segments<Multisegment: From<Vec<Segment>>, Segment>(
    segments: Vec<Segment>,
) -> pyo3::PyResult<Multisegment> {
    if segments.len() < MIN_MULTISEGMENT_SEGMENTS_COUNT {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Multisegment should have at least {} segments, but found {}.",
            MIN_MULTISEGMENT_SEGMENTS_COUNT,
            segments.len()
        )))
    } else {
        Ok(Multisegment::from(segments))
    }
}

pub(super) fn try_pack_vertices<Contour: From<Vec<Point>>, Point>(
    vertices: Vec<Point>,
) -> pyo3::PyResult<Contour> {
    if vertices.len() < MIN_CONTOUR_VERTICES_COUNT {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Contour should have at least {} vertices, but found {}.",
            MIN_CONTOUR_VERTICES_COUNT,
            vertices.len()
        )))
    } else {
        Ok(Contour::from(vertices))
    }
}
