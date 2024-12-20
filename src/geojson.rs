// GeoJSON structs
extern crate serde;
extern crate serde_json;

use error::Error;
use ewkb;
use ewkb::AsEwkbPoint;
use ewkb::*;
use ewkb::{
    AsEwkbLineString, AsEwkbMultiLineString, AsEwkbMultiPoint, AsEwkbMultiPolygon, AsEwkbPolygon,
};
use twkb;
use types::{
    LineString as LineStringTrait, MultiPolygon as MultiPolygonTrait, Point as PointTrait,
    Polygon as PolygonTrait,
};

use self::serde::ser::{SerializeStruct, Serializer};
use self::serde::{Deserialize, Serialize};

#[derive(PartialEq)]
pub enum GeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
    None,
}

pub trait GeoJSONPoint: Send + Sync {
    fn to_geojson(&self) -> Point;
}

pub trait GeoJSONLineString: Send + Sync {
    fn to_geojson(&self) -> LineString;
}

pub trait GeoJSONPolygon: Send + Sync {
    fn to_geojson(&self) -> Polygon;
}

pub trait GeoJSONMultiPoint: Send + Sync {
    fn to_geojson(&self) -> MultiPoint;
}

pub trait GeoJSONMultiLineString: Send + Sync {
    fn to_geojson(&self) -> MultiLineString;
}
pub trait GeoJSONMultiPolygon: Send + Sync {
    fn to_geojson(&self) -> MultiPolygon;
}

pub trait GeoJSONEncode: Send + Sync {
    fn as_str(&self) -> String;
    fn has_z(&self) -> bool;
    fn has_zm(&self) -> bool;
    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error>;
}

#[derive(Serialize)]
struct CrsProps {
    name: String,
}

fn crs_serializer<S>(srid: &Option<i32>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(srid) = srid {
        let crs_str = format!("EPSG:{}", srid);
        let mut crs_def = s.serialize_struct("crs", 2)?;
        let crs_props = CrsProps { name: crs_str };
        crs_def.serialize_field("type", "name")?;
        crs_def.serialize_field::<CrsProps>("properties", &crs_props)?;
        return crs_def.end();
    }
    s.serialize_none()
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<f64>,
}

fn to_ewkb_point(crds: &[f64], srid: Option<i32>) -> ewkb::Point {
    ewkb::Point::new(crds[0], crds[1], None, None, srid)
}

fn to_ewkb_pointz(crds: &[f64], srid: Option<i32>) -> ewkb::PointZ {
    ewkb::PointZ::new(crds[0], crds[1], crds[2], None, srid)
}

fn to_ewkb_pointzm(crds: &[f64], srid: Option<i32>) -> ewkb::PointZM {
    ewkb::PointZM::new(crds[0], crds[1], crds[2], crds[3], srid)
}

impl GeoJSONEncode for Point {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        self.coordinates.len() == 3
    }
    fn has_zm(&self) -> bool {
        self.coordinates.len() == 4
    }

    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = Vec::with_capacity(9 + 8 * 3);
        if self.has_zm() {
            let geom = to_ewkb_pointzm(&self.coordinates, srid);
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let geom = to_ewkb_pointz(&self.coordinates, srid);
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let geom = to_ewkb_point(&self.coordinates, srid);
            geom.as_ewkb().write_ewkb(&mut data)?;
        }
        Ok(data)
    }
}

impl GeoJSONPoint for twkb::Point {
    fn to_geojson(&self) -> Point {
        Point {
            type_name: "Point".to_owned(),
            crs: None,
            coordinates: self.crds(),
        }
    }
}

impl GeoJSONPoint for ewkb::Point {
    fn to_geojson(&self) -> Point {
        Point {
            type_name: "Point".to_owned(),
            crs: self.srid,
            coordinates: self.crds(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct LineString {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<Vec<f64>>,
}

impl GeoJSONLineString for twkb::LineString {
    fn to_geojson(&self) -> LineString {
        return LineString {
            type_name: "LineString".to_owned(),
            crs: None,
            coordinates: self.points().map(|x| x.crds()).collect(),
        };
    }
}

impl GeoJSONLineString for ewkb::LineString {
    fn to_geojson(&self) -> LineString {
        return LineString {
            type_name: "LineString".to_owned(),
            crs: self.srid,
            coordinates: self.points().map(|x| x.crds()).collect(),
        };
    }
}

impl GeoJSONEncode for LineString {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        !self.coordinates.is_empty() && self.coordinates[0].len() == 3
    }
    fn has_zm(&self) -> bool {
        !self.coordinates.is_empty() && self.coordinates[0].len() == 4
    }

    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = Vec::with_capacity(9 + 8 * 3 * self.coordinates.len());
        if self.has_zm() {
            let mut geom = ewkb::LineStringZM::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_pointzm(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let mut geom = ewkb::LineStringZ::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_pointz(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let mut geom = ewkb::LineString::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_point(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        }

        Ok(data)
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Polygon {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

impl GeoJSONEncode for Polygon {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        if !self.coordinates.is_empty() {
            let ring = &self.coordinates[0];
            !ring.is_empty() && ring[0].len() == 3
        } else {
            false
        }
    }
    fn has_zm(&self) -> bool {
        if !self.coordinates.is_empty() {
            let ring = &self.coordinates[0];
            !ring.is_empty() && ring[0].len() == 4
        } else {
            false
        }
    }
    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = vec![];
        if self.has_zm() {
            let mut geom = ewkb::PolygonZM::new();
            let rings = self
                .coordinates
                .iter()
                .map(|ring| {
                    ring.iter()
                        .map(|crds| to_ewkb_pointzm(crds, srid))
                        .collect()
                })
                .collect();
            geom.rings = rings;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let mut geom = ewkb::PolygonZ::new();
            let rings = self
                .coordinates
                .iter()
                .map(|ring| ring.iter().map(|crds| to_ewkb_pointz(crds, srid)).collect())
                .collect();
            geom.rings = rings;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let mut geom = ewkb::Polygon::new();
            let rings = self
                .coordinates
                .iter()
                .map(|ring| ring.iter().map(|crds| to_ewkb_point(crds, srid)).collect())
                .collect();
            geom.rings = rings;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        }
        Ok(data)
    }
}

impl GeoJSONPolygon for twkb::Polygon {
    fn to_geojson(&self) -> Polygon {
        let mut rings: Vec<Vec<Vec<f64>>> = Vec::new();
        rings.reserve(self.rings().len());
        for ring in &self.rings {
            let crds = &ring.points;
            let crds_vec: Vec<Vec<f64>> = crds.iter().map(|crd| crd.crds()).collect();
            rings.push(crds_vec);
        }

        Polygon {
            type_name: "Polygon".to_owned(),
            crs: None,
            coordinates: rings,
        }
    }
}

impl GeoJSONPolygon for ewkb::Polygon {
    fn to_geojson(&self) -> Polygon {
        let mut rings: Vec<Vec<Vec<f64>>> = vec![];
        for ring in &self.rings {
            let crds = &ring.points;
            let crds_vec: Vec<Vec<f64>> = crds.iter().map(|crd| crd.crds()).collect();
            rings.push(crds_vec);
        }

        Polygon {
            type_name: "Polygon".to_owned(),
            crs: self.srid,
            coordinates: rings,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MultiPoint {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<Vec<f64>>,
}
impl GeoJSONMultiPoint for twkb::MultiPoint {
    fn to_geojson(&self) -> MultiPoint {
        MultiPoint {
            type_name: "MultiPoint".to_owned(),
            crs: None,
            coordinates: self.points.iter().map(|pnt| pnt.crds()).collect(),
        }
    }
}

impl GeoJSONMultiPoint for ewkb::MultiPoint {
    fn to_geojson(&self) -> MultiPoint {
        MultiPoint {
            type_name: "MultiPoint".to_owned(),
            crs: self.srid,
            coordinates: self.points.iter().map(|pnt| pnt.crds()).collect(),
        }
    }
}

impl GeoJSONEncode for MultiPoint {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        !self.coordinates.is_empty() && self.coordinates[0].len() == 3
    }
    fn has_zm(&self) -> bool {
        !self.coordinates.is_empty() && self.coordinates[0].len() == 4
    }
    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = vec![];
        if self.has_zm() {
            let mut geom = ewkb::MultiPointZM::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_pointzm(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let mut geom = ewkb::MultiPointZ::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_pointz(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let mut geom = ewkb::MultiPoint::new();
            let pnts = self
                .coordinates
                .iter()
                .map(|crds| to_ewkb_point(crds, srid))
                .collect();
            geom.points = pnts;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        }
        Ok(data)
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MultiLineString {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<Vec<Vec<f64>>>,
}
impl GeoJSONMultiLineString for twkb::MultiLineString {
    fn to_geojson(&self) -> MultiLineString {
        return MultiLineString {
            type_name: "MultiLineString".to_owned(),
            crs: None,
            coordinates: self
                .lines
                .iter()
                .map(|line| line.points.iter().map(|pnt| pnt.crds()).collect())
                .collect(),
        };
    }
}

impl GeoJSONMultiLineString for ewkb::MultiLineString {
    fn to_geojson(&self) -> MultiLineString {
        return MultiLineString {
            type_name: "MultiLineString".to_owned(),
            crs: self.srid,
            coordinates: self
                .lines
                .iter()
                .map(|line| line.points.iter().map(|pnt| pnt.crds()).collect())
                .collect(),
        };
    }
}

impl GeoJSONEncode for MultiLineString {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        if !self.coordinates.is_empty() {
            let line = &self.coordinates[0];
            !line.is_empty() && line[0].len() == 3
        } else {
            false
        }
    }
    fn has_zm(&self) -> bool {
        if !self.coordinates.is_empty() {
            let line = &self.coordinates[0];
            !line.is_empty() && line[0].len() == 4
        } else {
            false
        }
    }

    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = Vec::with_capacity(9 + 8 * 3 * self.coordinates.len());
        if self.has_zm() {
            let mut geom = ewkb::MultiLineStringZM::new();
            let lines = self
                .coordinates
                .iter()
                .map(|lines| {
                    lines
                        .iter()
                        .map(|crds| to_ewkb_pointzm(crds, srid))
                        .collect()
                })
                .collect();
            geom.lines = lines;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let mut geom = ewkb::MultiLineStringZ::new();
            let lines = self
                .coordinates
                .iter()
                .map(|lines| {
                    lines
                        .iter()
                        .map(|crds| to_ewkb_pointz(crds, srid))
                        .collect()
                })
                .collect();
            geom.lines = lines;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let mut geom = ewkb::MultiLineString::new();
            let lines = self
                .coordinates
                .iter()
                .map(|lines| lines.iter().map(|crds| to_ewkb_point(crds, srid)).collect())
                .collect();
            geom.lines = lines;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        }
        Ok(data)
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MultiPolygon {
    #[serde(rename(serialize = "type"))]
    pub type_name: String,
    #[serde(serialize_with = "crs_serializer")]
    pub crs: Option<i32>,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}
impl GeoJSONEncode for MultiPolygon {
    fn as_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn has_z(&self) -> bool {
        if !self.coordinates.is_empty() {
            let poly = &self.coordinates[0];
            if !poly.is_empty() {
                let ring = &poly[0];
                !ring.is_empty() && ring[0].len() == 3
            } else {
                false
            }
        } else {
            false
        }
    }
    fn has_zm(&self) -> bool {
        if !self.coordinates.is_empty() {
            let poly = &self.coordinates[0];
            if !poly.is_empty() {
                let ring = &poly[0];
                !ring.is_empty() && ring[0].len() == 4
            } else {
                false
            }
        } else {
            false
        }
    }
    fn to_ewkb(&self, srid: Option<i32>) -> Result<Vec<u8>, Error> {
        let mut data = vec![];
        if self.has_zm() {
            let mut geom = ewkb::MultiPolygonZM::new();
            let polys = self
                .coordinates
                .iter()
                .map(|poly| {
                    poly.iter()
                        .map(|ring| {
                            ring.iter()
                                .map(|crds| to_ewkb_pointzm(crds, srid))
                                .collect()
                        })
                        .collect()
                })
                .collect();
            geom.polygons = polys;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else if self.has_z() {
            let mut geom = ewkb::MultiPolygonZ::new();
            let polys = self
                .coordinates
                .iter()
                .map(|poly| {
                    poly.iter()
                        .map(|ring| ring.iter().map(|crds| to_ewkb_pointz(crds, srid)).collect())
                        .collect()
                })
                .collect();
            geom.polygons = polys;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        } else {
            let mut geom = ewkb::MultiPolygon::new();
            let polys = self
                .coordinates
                .iter()
                .map(|poly| {
                    poly.iter()
                        .map(|ring| ring.iter().map(|crds| to_ewkb_point(crds, srid)).collect())
                        .collect()
                })
                .collect();
            geom.polygons = polys;
            geom.srid = srid;
            geom.as_ewkb().write_ewkb(&mut data)?;
        }
        Ok(data)
    }
}

impl GeoJSONMultiPolygon for twkb::MultiPolygon {
    fn to_geojson(&self) -> MultiPolygon {
        let mut polygons: Vec<Vec<Vec<Vec<f64>>>> = Vec::new();
        polygons.reserve(self.polygons().len());
        for polygon in &self.polygons {
            let mut poly_crds = Vec::new();
            poly_crds.reserve(polygon.rings.len());
            for ring in polygon.rings() {
                let crds = &ring.points;
                let crds_vec: Vec<Vec<f64>> = crds.iter().map(|crd| crd.crds()).collect();
                poly_crds.push(crds_vec);
            }
            polygons.push(poly_crds);
        }

        MultiPolygon {
            type_name: "MultiPolygon".to_owned(),
            crs: None,
            coordinates: polygons,
        }
    }
}

impl GeoJSONMultiPolygon for ewkb::MultiPolygon {
    fn to_geojson(&self) -> MultiPolygon {
        let mut polygons: Vec<Vec<Vec<Vec<f64>>>> = Vec::new();
        polygons.reserve(self.polygons().len());
        for polygon in &self.polygons {
            let mut poly_crds = Vec::new();
            poly_crds.reserve(polygon.rings.len());
            for ring in polygon.rings() {
                let crds = &ring.points;
                let crds_vec: Vec<Vec<f64>> = crds.iter().map(|crd| crd.crds()).collect();
                poly_crds.push(crds_vec);
            }
            polygons.push(poly_crds);
        }

        MultiPolygon {
            type_name: "MultiPolygon".to_owned(),
            crs: self.srid,
            coordinates: polygons,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ewkb::{self, EwkbRead},
        geojson::{
            GeoJSONEncode, GeoJSONLineString, GeoJSONMultiLineString, GeoJSONMultiPoint,
            GeoJSONMultiPolygon, GeoJSONPoint, GeoJSONPolygon,
        },
        twkb::{self, TwkbGeom},
    };

    fn hex_to_vec(hexstr: &str) -> Vec<u8> {
        hexstr
            .as_bytes()
            .chunks(2)
            .map(|chars| {
                let hb = if chars[0] <= 57 {
                    chars[0] - 48
                } else {
                    chars[0] - 87
                };
                let lb = if chars[1] <= 57 {
                    chars[1] - 48
                } else {
                    chars[1] - 87
                };
                hb * 16 + lb
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_ewkb_readwrite_point() {
        let ewkb_data = hex_to_vec("0101000080000000000000f03f00000000000000400000000000001040"); // 3D Point
        let point = ewkb::Point::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_point = point.to_geojson();
        assert_eq!(
            format!("{:.0?}", geojson_point),
            "Point { type_name: \"Point\", crs: None, coordinates: [1, 2, 4] }"
        );
        assert_eq!(
            format!("{:.0?}", geojson_point.as_str()),
            "\"{\\\"type\\\":\\\"Point\\\",\\\"crs\\\":null,\\\"coordinates\\\":[1.0,2.0,4.0]}\""
        );
        let encoded = geojson_point.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_line() {
        let ewkb_data = hex_to_vec("010200008002000000000000000000f03f0000000000000040000000000000144000000000000024400000000000002e400000000000003640"); // 3D LineString
        let line = ewkb::LineString::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_line = line.to_geojson();

        assert_eq!(
        format!("{:.0?}", geojson_line),
        "LineString { type_name: \"LineString\", crs: None, coordinates: [[1, 2, 5], [10, 15, 22]] }"
    );
        assert_eq!(
        format!("{:.0?}", geojson_line.as_str()),
        "\"{\\\"type\\\":\\\"LineString\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[1.0,2.0,5.0],[10.0,15.0,22.0]]}\""
    );
        let encoded = geojson_line.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_polygon() {
        let ewkb_data = hex_to_vec("01030000a0e610000001000000070000003333333333f33840295c8fc2f5284e400000000000000840ae47e17a14ee384048e17a14ae274e4000000000000008403333333333f3384048e17a14ae274e4000000000000008407b14ae47e1fa384048e17a14ae274e4000000000000008403d0ad7a370fd3840295c8fc2f5284e4000000000000008407b14ae47e1fa38400ad7a3703d2a4e4000000000000008403333333333f33840295c8fc2f5284e400000000000000840"); // 3D Polygon
        let poly = ewkb::Polygon::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_poly = poly.to_geojson();

        assert_eq!(
        format!("{:.2?}", geojson_poly),
        "Polygon { type_name: \"Polygon\", crs: Some(4326), coordinates: [[[24.95, 60.32, 3.00], [24.93, 60.31, 3.00], [24.95, 60.31, 3.00], [24.98, 60.31, 3.00], [24.99, 60.32, 3.00], [24.98, 60.33, 3.00], [24.95, 60.32, 3.00]]] }"
    );
        assert_eq!(
        format!("{:.2?}", geojson_poly.as_str()),
        "\"{\\\"type\\\":\\\"Polygon\\\",\\\"crs\\\":{\\\"type\\\":\\\"name\\\",\\\"properties\\\":{\\\"name\\\":\\\"EPSG:4326\\\"}},\\\"coordinates\\\":[[[24.95,60.32,3.0],[24.93,60.31,3.0],[24.95,60.31,3.0],[24.98,60.31,3.0],[24.99,60.32,3.0],[24.98,60.33,3.0],[24.95,60.32,3.0]]]}\""
    );
        let encoded = geojson_poly.to_ewkb(Some(4326)).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_multipoint() {
        let ewkb_data = hex_to_vec("010400008003000000010100008000000000000024400000000000003440000000000000000001010000800000000000002e4000000000000039400000000000001440010100008000000000000034400000000000003e400000000000002440"); // 3D MultiPoint
        let geom = ewkb::MultiPoint::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "MultiPoint { type_name: \"MultiPoint\", crs: None, coordinates: [[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"MultiPoint\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0]]}\""
    );
        let encoded = geojson_geom.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_multilinestring() {
        let ewkb_data =
        hex_to_vec("0105000080020000000102000080030000000000000000002440000000000000344000000000000000000000000000002e400000000000003940000000000000144000000000000034400000000000003e4000000000000024400102000080020000000000000000003e4000000000000044400000000000000000000000000080414000000000008046400000000000001440"); // 3D MultiLineString
        let geom = ewkb::MultiLineString::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "MultiLineString { type_name: \"MultiLineString\", crs: None, coordinates: [[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]], [[30.0, 40.0, 0.0], [35.0, 45.0, 5.0]]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"MultiLineString\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0]],[[30.0,40.0,0.0],[35.0,45.0,5.0]]]}\""
    );
        let encoded = geojson_geom.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_tewkb_readwrite_multipolygon() {
        let ewkb_data = hex_to_vec("01060000800100000001030000800100000004000000a01a2fdd1e67114191ed7cff238f5941000000000000000052b81e0517671141931804ce228f594100000000000000009cc420b0036711417b14ae1f238f59410000000000000000a01a2fdd1e67114191ed7cff238f59410000000000000000"); // 2D MultiPolygon
        let poly = ewkb::MultiPolygon::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_geom = poly.to_geojson();
        assert_eq!(format!("{:.1?}", geojson_geom), "MultiPolygon { type_name: \"MultiPolygon\", crs: None, coordinates: [[[[285127.7, 6700176.0, 0.0], [285125.8, 6700171.2, 0.0], [285120.9, 6700172.5, 0.0], [285127.7, 6700176.0, 0.0]]]] }");
        assert_eq!(format!("{:.1?}", geojson_geom.as_str()), "\"{\\\"type\\\":\\\"MultiPolygon\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[[[285127.716,6700175.992,0.0],[285125.755,6700171.219,0.0],[285120.922,6700172.495,0.0],[285127.716,6700175.992,0.0]]]]}\"");
        let encoded = geojson_geom.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_twkb_read_point() {
        let twkb = hex_to_vec("410809d00fa01fe807"); // 3D Point
        let geom = twkb::Point::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
            format!("{:.1?}", geojson_geom),
            "Point { type_name: \"Point\", crs: None, coordinates: [10.0, 20.0, 5.0] }"
        );
        assert_eq!(
            format!("{:.1?}", geojson_geom.as_str()),
            "\"{\\\"type\\\":\\\"Point\\\",\\\"crs\\\":null,\\\"coordinates\\\":[10.0,20.0,5.0]}\""
        );
    }

    #[test]
    fn test_twkb_read_linestring() {
        let twkb = hex_to_vec("42080903d00fa01f00e807e807e807e807e807e807"); // 3D LineString
        let geom = twkb::LineString::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "LineString { type_name: \"LineString\", crs: None, coordinates: [[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"LineString\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0]]}\""
    );
    }

    #[test]
    fn test_twkb_read_polygon() {
        let twkb = hex_to_vec("4308090104d00fa01f00e807e807e807e807e807e807cf0fcf0fcf0f"); // 3D Polygon
        let geom = twkb::Polygon::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "Polygon { type_name: \"Polygon\", crs: None, coordinates: [[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0], [10.0, 20.0, 0.0]]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"Polygon\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0],[10.0,20.0,0.0]]]}\""
    );
    }

    #[test]
    fn test_twkb_read_multipoint() {
        let twkb = hex_to_vec("44080903d00fa01f00e807e807e807e807e807e807"); // 3D MultiPoint
        let geom = twkb::MultiPoint::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "MultiPoint { type_name: \"MultiPoint\", crs: None, coordinates: [[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"MultiPoint\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0]]}\""
    );
    }

    #[test]
    fn test_twkb_read_multilinestring() {
        let twkb =
            hex_to_vec("4508090203d00fa01f00e807e807e807e807e807e80702d00fd00fcf0fe807e807e807"); // 3D MultiLineString
        let geom = twkb::MultiLineString::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_geom = geom.to_geojson();
        assert_eq!(
        format!("{:.1?}", geojson_geom),
        "MultiLineString { type_name: \"MultiLineString\", crs: None, coordinates: [[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]], [[30.0, 40.0, 0.0], [35.0, 45.0, 5.0]]] }"
    );
        assert_eq!(
        format!("{:.1?}", geojson_geom.as_str()),
        "\"{\\\"type\\\":\\\"MultiLineString\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[[10.0,20.0,0.0],[15.0,25.0,5.0],[20.0,30.0,10.0]],[[30.0,40.0,0.0],[35.0,45.0,5.0]]]}\""
    );
    }

    #[test]
    fn test_twkb_read_multipolygon() {
        let twkb = hex_to_vec("660801010104c8d0f58f02f0c9e4f53100d11ec94a00c14bf81300946ad23600"); // 2D MultiPolygon
        let poly = twkb::MultiPolygon::read_twkb(&mut twkb.as_slice()).unwrap();
        let geojson_poly = poly.to_geojson();
        assert_eq!(format!("{:.1?}", geojson_poly), "MultiPolygon { type_name: \"MultiPolygon\", crs: None, coordinates: [[[[285127.7, 6700176.0, 0.0], [285125.8, 6700171.2, 0.0], [285120.9, 6700172.5, 0.0], [285127.7, 6700176.0, 0.0]]]] }");
        assert_eq!(format!("{:.1?}", geojson_poly.as_str()), "\"{\\\"type\\\":\\\"MultiPolygon\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[[[285127.716,6700175.992,0.0],[285125.755,6700171.219,0.0],[285120.922,6700172.494999999,0.0],[285127.716,6700175.992,0.0]]]]}\"");
    }

    #[test]
    fn test_ewkb_readwrite_pointzm() {
        let ewkb_data = hex_to_vec(
            "01010000c0000000000000f03f000000000000004000000000000010400000000000001440",
        ); // ZM Point
        let point = ewkb::Point::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_point = point.to_geojson();
        assert_eq!(
            format!("{:.0?}", geojson_point),
            "Point { type_name: \"Point\", crs: None, coordinates: [1, 2, 4, 5] }"
        );
        assert_eq!(
        format!("{:.0?}", geojson_point.as_str()),
        "\"{\\\"type\\\":\\\"Point\\\",\\\"crs\\\":null,\\\"coordinates\\\":[1.0,2.0,4.0,5.0]}\""
    );
        let encoded = geojson_point.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_linezm() {
        let ewkb_data = hex_to_vec("01020000c002000000000000000000f03f00000000000000400000000000001040000000000000144000000000000024400000000000002e4000000000000034400000000000003840"); // 3D LineString
        let line = ewkb::LineString::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_line = line.to_geojson();

        assert_eq!(
        format!("{:.0?}", geojson_line),
        "LineString { type_name: \"LineString\", crs: None, coordinates: [[1, 2, 4, 5], [10, 15, 20, 24]] }"
    );
        assert_eq!(
        format!("{:.0?}", geojson_line.as_str()),
        "\"{\\\"type\\\":\\\"LineString\\\",\\\"crs\\\":null,\\\"coordinates\\\":[[1.0,2.0,4.0,5.0],[10.0,15.0,20.0,24.0]]}\""
    );
        let encoded = geojson_line.to_ewkb(None).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_polygonzm() {
        let ewkb_data = hex_to_vec("01030000e0e610000001000000070000003333333333f33840295c8fc2f5284e4000000000000008400000000000002440ae47e17a14ee384048e17a14ae274e40000000000000084000000000000034403333333333f3384048e17a14ae274e4000000000000008400000000000003e407b14ae47e1fa384048e17a14ae274e40000000000000084000000000000044403d0ad7a370fd3840295c8fc2f5284e40000000000000084000000000000049407b14ae47e1fa38400ad7a3703d2a4e4000000000000008400000000000004e403333333333f33840295c8fc2f5284e4000000000000008400000000000002440"); // ZM Polygon
        let poly = ewkb::Polygon::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_poly = poly.to_geojson();

        assert_eq!(
        format!("{:.2?}", geojson_poly),
        "Polygon { type_name: \"Polygon\", crs: Some(4326), coordinates: [[[24.95, 60.32, 3.00, 10.00], [24.93, 60.31, 3.00, 20.00], [24.95, 60.31, 3.00, 30.00], [24.98, 60.31, 3.00, 40.00], [24.99, 60.32, 3.00, 50.00], [24.98, 60.33, 3.00, 60.00], [24.95, 60.32, 3.00, 10.00]]] }"
    );
        assert_eq!(
        format!("{:.2?}", geojson_poly.as_str()),
        "\"{\\\"type\\\":\\\"Polygon\\\",\\\"crs\\\":{\\\"type\\\":\\\"name\\\",\\\"properties\\\":{\\\"name\\\":\\\"EPSG:4326\\\"}},\\\"coordinates\\\":[[[24.95,60.32,3.0,10.0],[24.93,60.31,3.0,20.0],[24.95,60.31,3.0,30.0],[24.98,60.31,3.0,40.0],[24.99,60.32,3.0,50.0],[24.98,60.33,3.0,60.0],[24.95,60.32,3.0,10.0]]]}\""
    );
        let encoded = geojson_poly.to_ewkb(Some(4326)).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_multipointzm() {
        let ewkb_data = hex_to_vec("01040000e0e61000000300000001010000c0000000000000244000000000000034400000000000000000000000000000144001010000c00000000000002e4000000000000039400000000000001440000000000000244001010000c000000000000034400000000000003e4000000000000024400000000000002e40"); // ZM MultiPoint
        let multipoint = ewkb::MultiPoint::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_poly = multipoint.to_geojson();

        assert_eq!(
        format!("{:.2?}", geojson_poly),
        "MultiPoint { type_name: \"MultiPoint\", crs: Some(4326), coordinates: [[10.00, 20.00, 0.00, 5.00], [15.00, 25.00, 5.00, 10.00], [20.00, 30.00, 10.00, 15.00]] }"
    );
        assert_eq!(
        format!("{:.2?}", geojson_poly.as_str()),
        "\"{\\\"type\\\":\\\"MultiPoint\\\",\\\"crs\\\":{\\\"type\\\":\\\"name\\\",\\\"properties\\\":{\\\"name\\\":\\\"EPSG:4326\\\"}},\\\"coordinates\\\":[[10.0,20.0,0.0,5.0],[15.0,25.0,5.0,10.0],[20.0,30.0,10.0,15.0]]}\""
    );
        let encoded = geojson_poly.to_ewkb(Some(4326)).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_multilinestringzm() {
        let ewkb_data = hex_to_vec("01050000e0e61000000200000001020000c00200000000000000000024400000000000003440000000000000000000000000000014400000000000002e4000000000000039400000000000001440000000000000244001020000c00200000000000000000034400000000000003e4000000000000024400000000000002e40000000000000394000000000008041400000000000002e400000000000003440"); // ZM MultiLineString
        let multiline = ewkb::MultiLineString::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_multiline = multiline.to_geojson();

        assert_eq!(
        format!("{:.2?}", geojson_multiline),
        "MultiLineString { type_name: \"MultiLineString\", crs: Some(4326), coordinates: [[[10.00, 20.00, 0.00, 5.00], [15.00, 25.00, 5.00, 10.00]], [[20.00, 30.00, 10.00, 15.00], [25.00, 35.00, 15.00, 20.00]]] }"
    );
        assert_eq!(
        format!("{:.2?}", geojson_multiline.as_str()),
        "\"{\\\"type\\\":\\\"MultiLineString\\\",\\\"crs\\\":{\\\"type\\\":\\\"name\\\",\\\"properties\\\":{\\\"name\\\":\\\"EPSG:4326\\\"}},\\\"coordinates\\\":[[[10.0,20.0,0.0,5.0],[15.0,25.0,5.0,10.0]],[[20.0,30.0,10.0,15.0],[25.0,35.0,15.0,20.0]]]}\""
    );
        let encoded = geojson_multiline.to_ewkb(Some(4326)).unwrap();
        assert_eq!(encoded, ewkb_data);
    }

    #[test]
    fn test_ewkb_readwrite_multipolygonzm() {
        let ewkb_data = hex_to_vec("01060000e0e61000000200000001030000c0010000000400000000000000000024400000000000003440000000000000000000000000000014400000000000002e4000000000000039400000000000001440000000000000244000000000000034400000000000003e4000000000000024400000000000002e40000000000000244000000000000034400000000000000000000000000000144001030000c0010000000400000000000000000034400000000000003e4000000000000024400000000000002e40000000000000394000000000008041400000000000002e4000000000000034400000000000003e4000000000000044400000000000003440000000000000394000000000000034400000000000003e4000000000000024400000000000002e40"); // ZM MultiPolygon
        let multipolygon = ewkb::MultiPolygon::read_ewkb(&mut ewkb_data.as_slice()).unwrap();
        let geojson_multipolygon = multipolygon.to_geojson();

        assert_eq!(
        format!("{:.2?}", geojson_multipolygon),
        "MultiPolygon { type_name: \"MultiPolygon\", crs: Some(4326), coordinates: [[[[10.00, 20.00, 0.00, 5.00], [15.00, 25.00, 5.00, 10.00], [20.00, 30.00, 10.00, 15.00], [10.00, 20.00, 0.00, 5.00]]], [[[20.00, 30.00, 10.00, 15.00], [25.00, 35.00, 15.00, 20.00], [30.00, 40.00, 20.00, 25.00], [20.00, 30.00, 10.00, 15.00]]]] }"
    );
        assert_eq!(
        format!("{:.2?}", geojson_multipolygon.as_str()),
        "\"{\\\"type\\\":\\\"MultiPolygon\\\",\\\"crs\\\":{\\\"type\\\":\\\"name\\\",\\\"properties\\\":{\\\"name\\\":\\\"EPSG:4326\\\"}},\\\"coordinates\\\":[[[[10.0,20.0,0.0,5.0],[15.0,25.0,5.0,10.0],[20.0,30.0,10.0,15.0],[10.0,20.0,0.0,5.0]]],[[[20.0,30.0,10.0,15.0],[25.0,35.0,15.0,20.0],[30.0,40.0,20.0,25.0],[20.0,30.0,10.0,15.0]]]]}\""
    );
        let encoded = geojson_multipolygon.to_ewkb(Some(4326)).unwrap();
        assert_eq!(encoded, ewkb_data);
    }
}
