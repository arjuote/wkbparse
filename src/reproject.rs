extern crate proj;
use self::proj::Coord;
use self::proj::Proj;
use crate::error::Error;
use crate::geojson::MultiLineString;
use crate::geojson::MultiPoint;
use crate::geojson::MultiPolygon;
use crate::geojson::Polygon;
use crate::geojson::{LineString, Point};

pub struct Crd<'a>(&'a mut Vec<f64>);

pub fn xform_crds(crds: &mut [Crd], xform: Proj) -> Result<(), Error> {
    for crd in crds {
        let res = xform
            .convert((crd.0[0], crd.0[1]))
            .map_err(|err| Error::Other(format!("reprojection failed: {}", err)))?;
        crd.0[0] = res.0;
        crd.0[1] = res.1;
    }
    Ok(())
}

impl Coord<f64> for &mut Crd<'_> {
    fn x(&self) -> f64 {
        self.0[0]
    }

    fn y(&self) -> f64 {
        self.0[1]
    }

    fn from_xy(_x: f64, _y: f64) -> Self {
        panic!("not allowed")
    }
}

pub trait AsCrds<'a> {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>>;
}

impl<'a> AsCrds<'a> for Point {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        vec![Crd(self.coordinates.as_mut())]
    }
}

impl<'a> AsCrds<'a> for LineString {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        self.coordinates.iter_mut().map(Crd).collect::<Vec<Crd>>()
    }
}

impl<'a> AsCrds<'a> for Polygon {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        self.coordinates
            .iter_mut()
            .flat_map(|ring| ring.iter_mut().map(Crd))
            .collect::<Vec<Crd>>()
    }
}
impl<'a> AsCrds<'a> for MultiPoint {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        self.coordinates.iter_mut().map(Crd).collect::<Vec<Crd>>()
    }
}

impl<'a> AsCrds<'a> for MultiLineString {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        self.coordinates
            .iter_mut()
            .flat_map(|line| line.iter_mut().map(Crd))
            .collect::<Vec<Crd>>()
    }
}

impl<'a> AsCrds<'a> for MultiPolygon {
    fn as_crds(&'a mut self) -> Vec<Crd<'a>> {
        self.coordinates
            .iter_mut()
            .flat_map(|polygon| polygon.iter_mut().flat_map(|ring| ring.iter_mut().map(Crd)))
            .collect::<Vec<Crd>>()
    }
}

#[cfg(test)]
mod test {
    extern crate approx;
    use self::approx::assert_relative_eq;
    use super::proj::Proj;
    use crate::geojson::{LineString, MultiLineString, MultiPoint, MultiPolygon, Polygon};

    use super::{xform_crds, AsCrds};
    #[test]
    fn test_reproject_linestring() {
        let mut ls = LineString {
            coordinates: vec![
                vec![24.97793, 60.33016],
                vec![24.94841, 60.31733],
                vec![24.92764, 60.30755],
            ],
            type_name: "LineString".to_string(),
            crs: None,
        };
        let xform = Proj::new_known_crs("EPSG:4326", "EPSG:3067", None).unwrap();
        let mut crds = ls.as_crds();
        xform_crds(&mut crds, xform).unwrap();

        let expected = [
            [388351.126, 6689893.389],
            [386677.069, 6688515.295],
            [385495.829, 6687462.308],
        ];
        for (i, vtx) in ls.coordinates.iter().enumerate() {
            let expected_vtx = expected[i];
            assert_relative_eq!(vtx[0], expected_vtx[0], epsilon = 0.001);
            assert_relative_eq!(vtx[1], expected_vtx[1], epsilon = 0.001);
        }
    }

    #[test]
    fn test_reproject_polygon() {
        let mut polygon = Polygon {
            // What a beautiful polygon, thanks ChatGPT.
            coordinates: vec![
                vec![
                    vec![24.97793, 60.33016],
                    vec![24.94841, 60.31733],
                    vec![24.92764, 60.30755],
                    vec![24.97793, 60.33016],
                ],
                vec![
                    vec![24.95000, 60.32000],
                    vec![24.94000, 60.31500],
                    vec![24.93000, 60.31000],
                    vec![24.95000, 60.32000],
                ],
            ],
            type_name: "Polygon".to_string(),
            crs: None,
        };

        let xform = Proj::new_known_crs("EPSG:4326", "EPSG:3067", None).unwrap();
        let mut crds = polygon.as_crds();
        xform_crds(&mut crds, xform).unwrap();

        let expected = [
            [
                [388351.126, 6689893.389],
                [386677.069, 6688515.295],
                [385495.829, 6687462.308],
                [388351.126, 6689893.389],
            ],
            [
                [386774.121, 6688809.833],
                [386204.525, 6688270.365],
                [385634.762, 6687730.985],
                [386774.121, 6688809.833],
            ],
        ];

        for (ring_idx, ring) in polygon.coordinates.iter().enumerate() {
            for (i, vtx) in ring.iter().enumerate() {
                let expected_vtx = expected[ring_idx][i];
                assert_relative_eq!(vtx[0], expected_vtx[0], epsilon = 0.001);
                assert_relative_eq!(vtx[1], expected_vtx[1], epsilon = 0.001);
            }
        }
    }

    #[test]
    fn test_reproject_multipoint() {
        let mut multipoint = MultiPoint {
            coordinates: vec![
                vec![24.97793, 60.33016],
                vec![24.94841, 60.31733],
                vec![24.92764, 60.30755],
            ],
            type_name: "MultiPoint".to_string(),
            crs: None,
        };

        let xform = Proj::new_known_crs("EPSG:4326", "EPSG:3067", None).unwrap();
        let mut crds = multipoint.as_crds();
        xform_crds(&mut crds, xform).unwrap();

        let expected = [
            [388351.126, 6689893.389],
            [386677.069, 6688515.295],
            [385495.829, 6687462.308],
        ];

        for (i, vtx) in multipoint.coordinates.iter().enumerate() {
            let expected_vtx = expected[i];
            assert_relative_eq!(vtx[0], expected_vtx[0], epsilon = 0.001);
            assert_relative_eq!(vtx[1], expected_vtx[1], epsilon = 0.001);
        }
    }

    #[test]
    fn test_reproject_multilinestring() {
        let mut multilinestring = MultiLineString {
            coordinates: vec![
                vec![vec![24.97793, 60.33016], vec![24.94841, 60.31733]],
                vec![vec![24.92764, 60.30755], vec![24.90000, 60.30000]],
            ],
            type_name: "MultiLineString".to_string(),
            crs: None,
        };

        let xform = Proj::new_known_crs("EPSG:4326", "EPSG:3067", None).unwrap();
        let mut crds = multilinestring.as_crds();
        xform_crds(&mut crds, xform).unwrap();

        let expected = [
            [[388351.126, 6689893.389], [386677.069, 6688515.295]],
            [[385495.829, 6687462.308], [383942.206, 6686670.045]],
        ];

        for (line_idx, line) in multilinestring.coordinates.iter().enumerate() {
            for (i, vtx) in line.iter().enumerate() {
                let expected_vtx = expected[line_idx][i];
                assert_relative_eq!(vtx[0], expected_vtx[0], epsilon = 0.001);
                assert_relative_eq!(vtx[1], expected_vtx[1], epsilon = 0.001);
            }
        }
    }

    #[test]
    fn test_reproject_multipolygon() {
        let mut multipolygon = MultiPolygon {
            coordinates: vec![
                vec![vec![
                    vec![24.97793, 60.33016],
                    vec![24.94841, 60.31733],
                    vec![24.92764, 60.30755],
                    vec![24.97793, 60.33016],
                ]],
                vec![vec![
                    vec![24.95000, 60.32000],
                    vec![24.94000, 60.31500],
                    vec![24.93000, 60.31000],
                    vec![24.95000, 60.32000],
                ]],
            ],
            type_name: "MultiPolygon".to_string(),
            crs: None,
        };

        let xform = Proj::new_known_crs("EPSG:4326", "EPSG:3067", None).unwrap();
        let mut crds = multipolygon.as_crds();
        xform_crds(&mut crds, xform).unwrap();

        let expected = [
            [[
                [388351.126, 6689893.389],
                [386677.069, 6688515.295],
                [385495.829, 6687462.308],
                [388351.126, 6689893.389],
            ]],
            [[
                [386774.121, 6688809.833],
                [386204.525, 6688270.365],
                [385634.762, 6687730.985],
                [386774.121, 6688809.833],
            ]],
        ];

        for (polygon_idx, polygon) in multipolygon.coordinates.iter().enumerate() {
            for (ring_idx, ring) in polygon.iter().enumerate() {
                for (i, vtx) in ring.iter().enumerate() {
                    let expected_vtx = expected[polygon_idx][ring_idx][i];
                    assert_relative_eq!(vtx[0], expected_vtx[0], epsilon = 0.001);
                    assert_relative_eq!(vtx[1], expected_vtx[1], epsilon = 0.001);
                }
            }
        }
    }
}
