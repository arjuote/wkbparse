// Python bindings
#[cfg(feature = "proj")]
extern crate proj;
extern crate pyo3;
use crate::error;
use crate::ewkb::{
    AsEwkbLineString, AsEwkbMultiLineString, AsEwkbMultiPoint, AsEwkbMultiPolygon, AsEwkbPoint,
    AsEwkbPolygon, EwkbRead, EwkbWrite,
};
use crate::geojson::{GeoJSONEncode, GeoJSONGeometry, GeometryType};

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

fn pydict_to_geojson(data: &PyDict) -> Result<GeoJSONGeometry, PyErr> {
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

    let data: GeoJSONGeometry = match type_name {
        "Point" => {
            let crds: Vec<f64> = crds_result.extract()?;
            GeoJSONGeometry::Point(geojson::Point {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        "LineString" => {
            let crds: Vec<Vec<f64>> = crds_result.extract()?;
            GeoJSONGeometry::LineString(geojson::LineString {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        "Polygon" => {
            let crds: Vec<Vec<Vec<f64>>> = crds_result.extract()?;
            GeoJSONGeometry::Polygon(geojson::Polygon {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        "MultiPoint" => {
            let crds: Vec<Vec<f64>> = crds_result.extract()?;
            GeoJSONGeometry::MultiPoint(geojson::MultiPoint {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        "MultiLineString" => {
            let crds: Vec<Vec<Vec<f64>>> = crds_result.extract()?;
            GeoJSONGeometry::MultiLineString(geojson::MultiLineString {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        "MultiPolygon" => {
            let crds: Vec<Vec<Vec<Vec<f64>>>> = crds_result.extract()?;
            GeoJSONGeometry::MultiPolygon(geojson::MultiPolygon {
                type_name: type_name.to_owned(),
                crs,
                coordinates: crds,
            })
        }
        _ => return Result::Err(PyValueError::new_err("invalid geoemetry type".to_owned())),
    };
    Ok(data)
}

fn geojson_to_pydict<'a>(py: Python<'a>, geom: &GeoJSONGeometry) -> Result<&'a PyDict, PyErr> {
    let key_vals: Vec<(&str, PyObject)> = match geom {
        GeoJSONGeometry::Point(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
        GeoJSONGeometry::LineString(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
        GeoJSONGeometry::Polygon(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
        GeoJSONGeometry::MultiPoint(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
        GeoJSONGeometry::MultiLineString(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
        GeoJSONGeometry::MultiPolygon(g) => {
            vec![
                ("type", g.type_name.to_object(py)),
                ("crs", g.crs.to_object(py)),
                ("coordinates", g.coordinates.to_object(py)),
            ]
        }
    };
    Ok(key_vals.into_py_dict(py))
}

fn parse_twkb_to_geojson(mut data: &[u8]) -> Result<GeoJSONGeometry, error::Error> {
    let geom_type = twkb::get_geom_type(&[data[0]]);
    match geom_type {
        GeometryType::Point => {
            let geom = twkb::Point::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::Point(geom.to_geojson()))
        }
        GeometryType::LineString => {
            let geom = twkb::LineString::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::LineString(geom.to_geojson()))
        }
        GeometryType::Polygon => {
            let geom = twkb::Polygon::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::Polygon(geom.to_geojson()))
        }
        GeometryType::MultiPoint => {
            let geom = twkb::MultiPoint::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiPoint(geom.to_geojson()))
        }
        GeometryType::MultiLineString => {
            let geom = twkb::MultiLineString::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiLineString(geom.to_geojson()))
        }
        GeometryType::MultiPolygon => {
            let geom = twkb::MultiPolygon::read_twkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiPolygon(geom.to_geojson()))
        }
        GeometryType::GeometryCollection => Err(WKBError::Other(
            "not implemented for GeometryCollection".to_owned(),
        )),
        GeometryType::None => Err(WKBError::Read("invalid geometry type".to_owned())),
    }
}

fn parse_ewkb_to_geojson(mut data: &[u8]) -> Result<GeoJSONGeometry, error::Error> {
    let geom_type = ewkb::get_geom_type(data);
    match geom_type {
        GeometryType::Point => {
            let geom = ewkb::Point::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::Point(geom.to_geojson()))
        }
        GeometryType::LineString => {
            let geom = ewkb::LineString::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::LineString(geom.to_geojson()))
        }
        GeometryType::Polygon => {
            let geom = ewkb::Polygon::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::Polygon(geom.to_geojson()))
        }
        GeometryType::MultiPoint => {
            let geom = ewkb::MultiPoint::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiPoint(geom.to_geojson()))
        }
        GeometryType::MultiLineString => {
            let geom = ewkb::MultiLineString::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiLineString(geom.to_geojson()))
        }
        GeometryType::MultiPolygon => {
            let geom = ewkb::MultiPolygon::read_ewkb(&mut data)?;
            Ok(GeoJSONGeometry::MultiPolygon(geom.to_geojson()))
        }
        GeometryType::GeometryCollection => Err(WKBError::Other(
            "not implemented for GeometryCollection".to_owned(),
        )),
        GeometryType::None => Err(WKBError::Read("invalid geometry type".to_owned())),
    }
}

/// TWKB parse
#[pyfunction]
fn twkb_to_geojson<'a>(
    py: Python<'a>,
    data: &[u8],
    from_srid: Option<i32>,
    to_srid: Option<i32>,
) -> PyResult<&'a PyDict> {
    let mut geojson_geom = parse_twkb_to_geojson(data)?;

    #[cfg(feature = "proj")]
    {
        use self::proj::Proj;
        match (from_srid, to_srid) {
            (None, None) => (),
            (None, Some(_)) => {
                return Err(error::Error::Other("missing from_srid".to_string()).into())
            }
            (Some(_), None) => {
                return Err(error::Error::Other("missing to_srid".to_string()).into())
            }
            (Some(from_srid), Some(to_srid)) => {
                let xform = Proj::new_known_crs(
                    &format!("EPSG:{}", from_srid),
                    &format!("EPSG:{}", to_srid),
                    None,
                )
                .map_err(|err| {
                    error::Error::Other(format!("failed to create transform: {}", err))
                })?;
                geojson_geom.transform(xform)?;
                geojson_geom.set_srid(to_srid);
            }
        };
    }

    let crds = match &geojson_geom {
        GeoJSONGeometry::Point(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::LineString(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::Polygon(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiPoint(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiLineString(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiPolygon(g) => ("coordinates", g.coordinates.to_object(py)),
    };

    let key_vals: Vec<(&str, PyObject)> = vec![
        ("type", geojson_geom.geom_type().to_string().to_object(py)),
        ("crs", geojson_geom.srid().to_object(py)),
        crds,
    ];
    Ok(key_vals.into_py_dict(py))
}

/// EWKB parse
#[pyfunction]
fn ewkb_to_geojson<'a>(
    py: Python<'a>,
    data: &[u8],
    from_srid: Option<i32>,
    to_srid: Option<i32>,
) -> PyResult<&'a PyDict> {
    let mut geojson_geom = parse_ewkb_to_geojson(data)?;

    #[cfg(feature = "proj")]
    {
        use self::proj::Proj;

        let from_srid = {
            if to_srid.is_some() {
                if let Some(from_srid) = from_srid {
                    Some(from_srid)
                } else {
                    geojson_geom.srid()
                }
            } else {
                None
            }
        };

        match (from_srid, to_srid) {
            (None, None) => (),
            (None, Some(_)) => {
                return Err(error::Error::Other("missing from_srid".to_string()).into())
            }
            (Some(_), None) => {
                return Err(error::Error::Other("missing to_srid".to_string()).into())
            }
            (Some(from_srid), Some(to_srid)) => {
                let xform = Proj::new_known_crs(
                    &format!("EPSG:{}", from_srid),
                    &format!("EPSG:{}", to_srid),
                    None,
                )
                .map_err(|err| {
                    error::Error::Other(format!("failed to create transform: {}", err))
                })?;
                geojson_geom.transform(xform)?;
                geojson_geom.set_srid(to_srid);
            }
        };
    }

    let crds = match &geojson_geom {
        GeoJSONGeometry::Point(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::LineString(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::Polygon(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiPoint(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiLineString(g) => ("coordinates", g.coordinates.to_object(py)),
        GeoJSONGeometry::MultiPolygon(g) => ("coordinates", g.coordinates.to_object(py)),
    };

    let key_vals: Vec<(&str, PyObject)> = vec![
        ("type", geojson_geom.geom_type().to_string().to_object(py)),
        ("crs", geojson_geom.srid().to_object(py)),
        crds,
    ];
    Ok(key_vals.into_py_dict(py))
}

#[cfg(not(feature = "proj"))]
#[pyfunction]
fn reproject_not_implemented() -> PyResult<()> {
    Err(PyValueError::new_err(
        "reproject_geojson not implemented - use wkbparse-proj package instead".to_owned(),
    ))
}

#[pyfunction]
fn geojson_to_ewkb<'a>(
    py: Python<'a>,
    data: &PyDict,
    from_srid: Option<i32>,
    to_srid: Option<i32>,
) -> PyResult<&'a PyBytes> {
    let mut geom = pydict_to_geojson(data)?;

    #[cfg(feature = "proj")]
    {
        use self::proj::Proj;

        let from_srid = {
            if to_srid.is_some() {
                if let Some(from_srid) = from_srid {
                    Some(from_srid)
                } else {
                    geom.srid()
                }
            } else {
                None
            }
        };

        match (from_srid, to_srid) {
            (None, None) => (),
            (None, Some(_)) => {
                return Err(error::Error::Other("missing from_srid".to_string()).into())
            }
            (Some(_), None) => {
                return Err(error::Error::Other("missing to_srid".to_string()).into())
            }
            (Some(from_srid), Some(to_srid)) => {
                let xform = Proj::new_known_crs(
                    &format!("EPSG:{}", from_srid),
                    &format!("EPSG:{}", to_srid),
                    None,
                )
                .map_err(|err| {
                    error::Error::Other(format!("failed to create transform: {}", err))
                })?;
                geom.transform(xform)?
            }
        };
    }

    let data = match geom {
        GeoJSONGeometry::Point(geom) => geom.to_ewkb()?,
        GeoJSONGeometry::LineString(geom) => geom.to_ewkb()?,
        GeoJSONGeometry::Polygon(geom) => geom.to_ewkb()?,
        GeoJSONGeometry::MultiPoint(geom) => geom.to_ewkb()?,
        GeoJSONGeometry::MultiLineString(geom) => geom.to_ewkb()?,
        GeoJSONGeometry::MultiPolygon(geom) => geom.to_ewkb()?,
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

#[cfg(all(feature = "proj", feature = "python", feature = "extension-module"))]
mod reproject {
    extern crate proj;
    use self::proj::Proj;
    use super::geojson_to_pydict;
    use crate::error::Error;
    use crate::geojson::GeoJSONEncode;
    use crate::pyo::pydict_to_geojson;
    use pyo::pyo3::pyfunction;
    use pyo::pyo3::types::PyDict;
    use pyo::pyo3::Python;
    use pyo::PyResult;

    #[pyfunction]
    pub(crate) fn reproject_geojson<'a>(
        py: Python<'a>,
        data: &PyDict,
        to_srid: i32,
        from_srid: Option<i32>,
    ) -> PyResult<&'a PyDict> {
        let mut geom = pydict_to_geojson(data)?;
        let from_srid = {
            if let Some(from_srid) = from_srid {
                format!("EPSG:{}", from_srid)
            } else if let Some(srid) = geom.srid() {
                format!("EPSG:{}", srid)
            } else {
                return Err(Error::Other(
                    "from_srid not provided and data does not have srid".to_string(),
                )
                .into());
            }
        };
        let xform = Proj::new_known_crs(from_srid.as_str(), &format!("EPSG:{}", to_srid), None)
            .map_err(|err| Error::Other(format!("failed to create transform: {}", err)))?;
        geom.transform(xform)?;
        geom.set_srid(to_srid);
        geojson_to_pydict(py, &geom)
    }
}

/// Conversions between EWKB, TWKB and GeoJSON geometries.
#[pymodule]
fn wkbparse(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyo::twkb_to_geojson, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::ewkb_to_geojson, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::geojson_to_ewkb, m)?)?;
    m.add_function(wrap_pyfunction!(pyo::twkb_to_ewkb, m)?)?;
    #[cfg(feature = "proj")]
    m.add_function(wrap_pyfunction!(pyo::reproject::reproject_geojson, m)?)?;
    #[cfg(not(feature = "proj"))]
    m.add_function(wrap_pyfunction!(pyo::reproject_not_implemented, m)?)?;
    Ok(())
}
