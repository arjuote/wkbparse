// Python bindings
extern crate pyo3;

use crate::error;
use crate::ewkb::{
    AsEwkbLineString, AsEwkbMultiLineString, AsEwkbMultiPoint, AsEwkbMultiPolygon, AsEwkbPoint,
    AsEwkbPolygon, EwkbRead, EwkbWrite,
};
use crate::geojson::{GeoJSONEncode, GeometryType};

use self::pyo3::prelude::*;
use self::pyo3::types::IntoPyDict;
use self::pyo3::types::{PyBytes, PyDict};
use error::Error as WKBError;
use ewkb;
use geojson;
use geojson::{
    GeoJSONLineString, GeoJSONMultiLineString, GeoJSONMultiPoint, GeoJSONMultiPolygon,
    GeoJSONPoint, GeoJSONPolygon,
};
use twkb;
use twkb::TwkbGeom;

impl From<WKBError> for PyErr {
    fn from(error: WKBError) -> Self {
        match error {
            WKBError::Read(_) => PyValueError::new_err(error.to_string()),
            WKBError::Write(_) => PyValueError::new_err(error.to_string()),
            WKBError::Other(_) => PyValueError::new_err(error.to_string()),
        }
    }
}

use self::pyo3::exceptions::PyValueError;

/// TWKB parse
#[pyfunction]
fn twkb_to_geojson<'a>(py: Python<'a>, mut data: &[u8]) -> PyResult<&'a PyDict> {
    let geom_type = twkb::get_geom_type(&[data[0]]);
    let pydict = match geom_type {
        GeometryType::Point => {
            let geom = twkb::Point::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::LineString => {
            let geom = twkb::LineString::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::Polygon => {
            let geom = twkb::Polygon::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiPoint => {
            let geom = twkb::MultiPoint::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiLineString => {
            let geom = twkb::MultiLineString::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiPolygon => {
            let geom = twkb::MultiPolygon::read_twkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::GeometryCollection => {
            return Err(
                WKBError::Other("not implemented for GeometryCollection".to_owned()).into(),
            );
        }
        GeometryType::None => {
            return Err(WKBError::Other("invalid geometry type".to_owned()).into());
        }
    };
    Ok(pydict)
}

/// EWKB parse
#[pyfunction]
fn ewkb_to_geojson<'a>(py: Python<'a>, mut data: &[u8]) -> PyResult<&'a PyDict> {
    let geom_type = ewkb::get_geom_type(data);
    let pydict = match geom_type {
        GeometryType::Point => {
            let geom = ewkb::Point::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::LineString => {
            let geom = ewkb::LineString::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::Polygon => {
            let geom = ewkb::Polygon::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiPoint => {
            let geom = ewkb::MultiPoint::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiLineString => {
            let geom = ewkb::MultiLineString::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::MultiPolygon => {
            let geom = ewkb::MultiPolygon::read_ewkb(&mut data)?;
            let geojson_geom = geom.to_geojson();
            let key_vals: Vec<(&str, PyObject)> = vec![
                ("type", geojson_geom.type_name.to_object(py)),
                ("crs", geojson_geom.crs.to_object(py)),
                ("coordinates", geojson_geom.coordinates.to_object(py)),
            ];
            key_vals.into_py_dict(py)
        }
        GeometryType::GeometryCollection => {
            return Err(
                error::Error::Other("not implemented for GeometryCollection".to_owned()).into(),
            );
        }
        GeometryType::None => {
            return Err(error::Error::Other("invalid geometry type".to_owned()).into());
        }
    };
    Ok(pydict)
}

#[pyfunction]
fn geojson_to_ewkb<'a>(py: Python<'a>, data: &PyDict) -> PyResult<&'a PyBytes> {
    let type_result = match data.get_item("type") {
        Some(type_name) => type_name,
        None => return Result::Err(PyValueError::new_err("invalid geojson".to_owned())),
    };
    let type_name: &str = type_result.extract()?;

    let crds_result = match data.get_item("coordinates") {
        Some(crds) => crds,
        None => return Result::Err(PyValueError::new_err("invalid geojson".to_owned())),
    };

    let crs = {
        let crs = data.get_item("crs");
        if let Some(crs) = crs {
            let crs: Option<i32> = crs.extract()?;
            crs
        } else {
            None
        }
    };

    let data = match type_name {
        "Point" => {
            let crds: Vec<f64> = crds_result.extract()?;
            let geom = geojson::Point {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        "LineString" => {
            let crds: Vec<Vec<f64>> = crds_result.extract()?;
            let geom = geojson::LineString {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        "Polygon" => {
            let crds: Vec<Vec<Vec<f64>>> = crds_result.extract()?;
            let geom = geojson::Polygon {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        "MultiPoint" => {
            let crds: Vec<Vec<f64>> = crds_result.extract()?;
            let geom = geojson::MultiPoint {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        "MultiLineString" => {
            let crds: Vec<Vec<Vec<f64>>> = crds_result.extract()?;
            let geom = geojson::MultiLineString {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        "MultiPolygon" => {
            let crds: Vec<Vec<Vec<Vec<f64>>>> = crds_result.extract()?;
            let geom = geojson::MultiPolygon {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            };
            geom.to_ewkb(crs)?
        }
        _ => return Result::Err(PyValueError::new_err("invalid geoemetry type".to_owned())),
    };
    Ok(PyBytes::new(py, &data))
}

#[pyfunction]
fn twkb_to_ewkb<'a>(py: Python<'a>, mut data: &[u8]) -> PyResult<&'a PyBytes> {
    let geom_type = twkb::get_geom_type(&[data[0]]);
    let result = match geom_type {
        GeometryType::Point => {
            let geom = twkb::Point::read_twkb(&mut data)?;
            let mut encoded = Vec::with_capacity(9 + 8 * 3);
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::LineString => {
            let geom = twkb::LineString::read_twkb(&mut data)?;
            let mut encoded = Vec::with_capacity(9 + 8 * 3 * geom.points.len());
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::Polygon => {
            let geom = twkb::Polygon::read_twkb(&mut data)?;
            let n_crds: usize = geom.rings.iter().map(|ring| ring.points.len()).sum();
            let mut encoded = Vec::with_capacity(9 + 8 * 3 * geom.rings.len() * n_crds);
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::MultiPoint => {
            let geom = twkb::MultiPoint::read_twkb(&mut data)?;
            let mut encoded = Vec::with_capacity(9 + 8 * 3 * geom.points.len());
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::MultiLineString => {
            let geom = twkb::MultiLineString::read_twkb(&mut data)?;
            let n_crds: usize = geom.lines.iter().map(|line| line.points.len()).sum();
            let mut encoded = Vec::with_capacity(9 + 8 * 3 * n_crds);
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::MultiPolygon => {
            let geom = twkb::MultiPolygon::read_twkb(&mut data)?;
            let n_crds: usize = geom
                .polygons
                .iter()
                .map(|poly| {
                    poly.rings
                        .iter()
                        .map(|ring| ring.points.len())
                        .sum::<usize>()
                })
                .sum();
            let mut encoded = Vec::with_capacity(9 + 8 * 3 * n_crds);
            geom.as_ewkb().write_ewkb(&mut encoded)?;
            encoded
        }
        GeometryType::GeometryCollection => {
            return Err(
                WKBError::Other("not implemented for GeometryCollection".to_owned()).into(),
            );
        }
        GeometryType::None => {
            return Err(WKBError::Other("invalid geometry type".to_owned()).into());
        }
    };
    Ok(PyBytes::new(py, &result))
}

/// A Python module implemented in Rust.
#[pymodule]
fn wkbparse(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyo::twkb_to_geojson, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::ewkb_to_geojson, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::geojson_to_ewkb, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::twkb_to_ewkb, m)?)?;
    Ok(())
}
