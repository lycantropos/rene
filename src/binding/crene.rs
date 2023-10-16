use pyo3::exceptions::PyValueError;
use pyo3::sync::GILOnceCell;
use pyo3::type_object::PyTypeInfo;
use pyo3::types::{PyModule, PyTuple};
use pyo3::{
    intern, pyclass, pymethods, pymodule, IntoPy, Py, PyAny, PyCell, PyObject,
    PyResult, Python, ToPyObject,
};

use crate::constants::{
    MIN_CONTOUR_VERTICES_COUNT, MIN_MULTIPOLYGON_POLYGONS_COUNT,
    MIN_MULTISEGMENT_SEGMENTS_COUNT,
};
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::Relation;

use super::traits::TryToPyAny;

#[pymodule]
fn _crene(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyLocation>()?;
    module.add_class::<PyOrientation>()?;
    module.add_class::<PyRelation>()?;
    module.add("MIN_CONTOUR_VERTICES_COUNT", MIN_CONTOUR_VERTICES_COUNT)?;
    module.add(
        "MIN_MULTIPOLYGON_POLYGONS_COUNT",
        MIN_MULTIPOLYGON_POLYGONS_COUNT,
    )?;
    module.add(
        "MIN_MULTISEGMENT_SEGMENTS_COUNT",
        MIN_MULTISEGMENT_SEGMENTS_COUNT,
    )?;
    Ok(())
}

impl IntoPy<PyObject> for Relation {
    fn into_py(self, py: Python<'_>) -> PyObject {
        IntoPy::into_py(PyRelation(self), py)
    }
}

impl ToPyObject for Relation {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        IntoPy::into_py(*self, py)
    }
}

impl TryToPyAny for Location {
    fn try_to_py_any(self, py: Python) -> PyResult<&PyAny> {
        static LOCATION_CLS: GILOnceCell<PyObject> = GILOnceCell::new();
        LOCATION_CLS
            .get_or_try_init(py, || {
                py.import("rene")?
                    .getattr(intern!(py, "Location"))
                    .map(|value| IntoPy::into_py(value, py))
            })?
            .getattr(
                py,
                match self {
                    Location::Boundary => intern!(py, "BOUNDARY"),
                    Location::Exterior => intern!(py, "EXTERIOR"),
                    Location::Interior => intern!(py, "INTERIOR"),
                },
            )
            .map(|value| value.into_ref(py))
    }
}

impl TryToPyAny for Orientation {
    fn try_to_py_any(self, py: Python) -> PyResult<&PyAny> {
        static ORIENTATION_CLS: GILOnceCell<PyObject> = GILOnceCell::new();
        ORIENTATION_CLS
            .get_or_try_init(py, || {
                py.import("rene")?
                    .getattr(intern!(py, "Orientation"))
                    .map(|value| IntoPy::into_py(value, py))
            })?
            .getattr(
                py,
                match self {
                    Orientation::Clockwise => {
                        intern!(py, "CLOCKWISE")
                    }
                    Orientation::Collinear => {
                        intern!(py, "COLLINEAR")
                    }
                    Orientation::Counterclockwise => {
                        intern!(py, "COUNTERCLOCKWISE")
                    }
                },
            )
            .map(|value| value.into_ref(py))
    }
}

impl TryToPyAny for Relation {
    fn try_to_py_any(self, py: Python) -> PyResult<&PyAny> {
        static RELATION_CLS: GILOnceCell<PyObject> = GILOnceCell::new();
        RELATION_CLS
            .get_or_try_init(py, || {
                py.import("rene")?
                    .getattr(intern!(py, "Relation"))
                    .map(|value| IntoPy::into_py(value, py))
            })?
            .getattr(
                py,
                match self {
                    Relation::Component => intern!(py, "COMPONENT"),
                    Relation::Composite => intern!(py, "COMPOSITE"),
                    Relation::Cover => intern!(py, "COVER"),
                    Relation::Cross => intern!(py, "CROSS"),
                    Relation::Disjoint => intern!(py, "DISJOINT"),
                    Relation::Enclosed => intern!(py, "ENCLOSED"),
                    Relation::Encloses => intern!(py, "ENCLOSES"),
                    Relation::Equal => intern!(py, "EQUAL"),
                    Relation::Overlap => intern!(py, "OVERLAP"),
                    Relation::Touch => intern!(py, "TOUCH"),
                    Relation::Within => intern!(py, "WITHIN"),
                },
            )
            .map(|value| value.into_ref(py))
    }
}

#[pyclass(name = "Location", module = "rene")]
struct PyLocation(Location);

#[pyclass(name = "Orientation", module = "rene")]
struct PyOrientation(Orientation);

#[pyclass(name = "Relation", module = "rene")]
struct PyRelation(Relation);

#[pymethods]
impl PyLocation {
    #[classattr]
    const BOUNDARY: PyLocation = PyLocation(Location::Boundary);

    #[classattr]
    const EXTERIOR: PyLocation = PyLocation(Location::Exterior);

    #[classattr]
    const INTERIOR: PyLocation = PyLocation(Location::Interior);

    fn __repr__(&self, _py: Python) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Location::Boundary => "BOUNDARY",
                Location::Exterior => "EXTERIOR",
                Location::Interior => "INTERIOR",
            }
        )
    }
}

#[pymethods]
impl PyOrientation {
    #[classattr]
    const CLOCKWISE: PyOrientation = PyOrientation(Orientation::Clockwise);

    #[classattr]
    const COLLINEAR: PyOrientation = PyOrientation(Orientation::Collinear);

    #[classattr]
    const COUNTERCLOCKWISE: PyOrientation =
        PyOrientation(Orientation::Counterclockwise);

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Orientation::Clockwise => "CLOCKWISE",
                Orientation::Collinear => "COLLINEAR",
                Orientation::Counterclockwise => "COUNTERCLOCKWISE",
            }
        )
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl PyRelation {
    #[classattr]
    fn COMPONENT(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[0].clone_ref(py)
    }

    #[classattr]
    fn COMPOSITE(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[1].clone_ref(py)
    }

    #[classattr]
    fn COVER(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[2].clone_ref(py)
    }

    #[classattr]
    fn CROSS(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[3].clone_ref(py)
    }

    #[classattr]
    fn DISJOINT(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[4].clone_ref(py)
    }

    #[classattr]
    fn ENCLOSED(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[5].clone_ref(py)
    }

    #[classattr]
    fn ENCLOSES(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[6].clone_ref(py)
    }

    #[classattr]
    fn EQUAL(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[7].clone_ref(py)
    }

    #[classattr]
    fn OVERLAP(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[8].clone_ref(py)
    }

    #[classattr]
    fn TOUCH(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[9].clone_ref(py)
    }

    #[classattr]
    fn WITHIN(py: Python) -> Py<PyRelation> {
        to_py_relation_values(py)[10].clone_ref(py)
    }

    #[new]
    #[pyo3(signature = (value, /))]
    fn new(value: &PyAny, py: Python) -> PyResult<Py<Self>> {
        let values = to_py_relation_values(py);
        match value.extract::<usize>() {
            Ok(value) if 1 <= value && value <= values.len() => {
                Ok(values[value - 1].clone_ref(py))
            }
            _ => Err(PyValueError::new_err(format!(
                "{} is not a valid {}",
                value.repr()?,
                Self::NAME
            ))),
        }
    }

    #[getter]
    fn complement(&self, py: Python) -> Py<PyRelation> {
        match self.0 {
            Relation::Component => Self::COMPOSITE(py),
            Relation::Composite => Self::COMPONENT(py),
            Relation::Cover => Self::WITHIN(py),
            Relation::Cross => Self::CROSS(py),
            Relation::Disjoint => Self::DISJOINT(py),
            Relation::Enclosed => Self::ENCLOSES(py),
            Relation::Encloses => Self::ENCLOSED(py),
            Relation::Equal => Self::EQUAL(py),
            Relation::Overlap => Self::OVERLAP(py),
            Relation::Touch => Self::TOUCH(py),
            Relation::Within => Self::COVER(py),
        }
    }

    #[getter]
    fn value(&self) -> u8 {
        match self.0 {
            Relation::Component => 1,
            Relation::Composite => 2,
            Relation::Cover => 3,
            Relation::Cross => 4,
            Relation::Disjoint => 5,
            Relation::Enclosed => 6,
            Relation::Encloses => 7,
            Relation::Equal => 8,
            Relation::Overlap => 9,
            Relation::Touch => 10,
            Relation::Within => 11,
        }
    }

    fn __getnewargs__<'a>(&self, py: Python<'a>) -> &'a PyTuple {
        PyTuple::new(py, [self.value()])
    }

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Relation::Component => "COMPONENT",
                Relation::Composite => "COMPOSITE",
                Relation::Cover => "COVER",
                Relation::Cross => "CROSS",
                Relation::Disjoint => "DISJOINT",
                Relation::Enclosed => "ENCLOSED",
                Relation::Encloses => "ENCLOSES",
                Relation::Equal => "EQUAL",
                Relation::Overlap => "OVERLAP",
                Relation::Touch => "TOUCH",
                Relation::Within => "WITHIN",
            }
        )
    }
}

fn to_py_relation_values(py: Python) -> &[Py<PyRelation>; 11] {
    static VALUES: GILOnceCell<[Py<PyRelation>; 11]> = GILOnceCell::new();
    VALUES.get_or_init(py, || {
        [
            PyCell::new(py, PyRelation(Relation::Component))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Composite))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Cover)).unwrap().into(),
            PyCell::new(py, PyRelation(Relation::Cross)).unwrap().into(),
            PyCell::new(py, PyRelation(Relation::Disjoint))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Enclosed))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Encloses))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Equal)).unwrap().into(),
            PyCell::new(py, PyRelation(Relation::Overlap))
                .unwrap()
                .into(),
            PyCell::new(py, PyRelation(Relation::Touch)).unwrap().into(),
            PyCell::new(py, PyRelation(Relation::Within))
                .unwrap()
                .into(),
        ]
    })
}
