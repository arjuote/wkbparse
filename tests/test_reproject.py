"""Reprojection tests for wkbparse-proj"""

from typing import Optional
import pytest
import wkbparse


# On-the fly EWKB-GeoJSON reprojection
def test_reproject_ewkb_point():
    """Test point reprojection from WGS84 latlng to Webmercator"""
    hex_string = "0101000080000000000000F03F00000000000000400000000000001040"
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "Point"
    crds = result.get("coordinates", [])
    assert crds[0] == pytest.approx(111319.491, abs=0.001)
    assert crds[1] == pytest.approx(222684.209, abs=0.001)
    assert crds[2] == pytest.approx(4.0, abs=0.001)


def test_reproject_ewkb_line():
    """Test linestring reprojection from WGS84 latlng to Webmercator"""
    hex_string = "010200008002000000000000000000f03f0000000000000040000000000000144000000000000024400000000000002e400000000000003640"
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "LineString"
    crds = result.get("coordinates", [])
    pnt1 = crds[0]
    assert pnt1[0] == pytest.approx(111319.491, abs=0.001)
    assert pnt1[1] == pytest.approx(222684.209, abs=0.001)
    assert pnt1[2] == pytest.approx(5.0, abs=0.001)

    pnt2 = crds[1]
    assert pnt2[0] == pytest.approx(1113194.908, abs=0.001)
    assert pnt2[1] == pytest.approx(1689200.14, abs=0.001)
    assert pnt2[2] == pytest.approx(22.0, abs=0.001)


@pytest.mark.parametrize("from_srid", [None, 4326])
def test_reproject_ewkb_polygon(from_srid: Optional[int]):
    """Test polygon reprojection from WGS84 latlng to ETRS-GK25 (input geometry contains crs definition)"""
    hex_string = "01030000a0e610000001000000070000003333333333f33840295c8fc2f5284e400000000000000840ae47e17a14ee384048e17a14ae274e4000000000000008403333333333f3384048e17a14ae274e4000000000000008407b14ae47e1fa384048e17a14ae274e4000000000000008403d0ad7a370fd3840295c8fc2f5284e4000000000000008407b14ae47e1fa38400ad7a3703d2a4e4000000000000008403333333333f33840295c8fc2f5284e400000000000000840"  # pragma: allowlist secret
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=from_srid, to_srid=3879
    )
    assert isinstance(result, dict)
    assert result.get("type") == "Polygon"
    assert result.get("crs") == 3879
    crds = result.get("coordinates", [])
    expected_crds = [
        [
            [25497236.988, 6689726.667, 3],
            [25496130.601, 6688613.497, 3],
            [25497236.143, 6688612.491, 3],
            [25498894.457, 6688611.611, 3],
            [25499447.398, 6689725.662, 3],
            [25498895.133, 6690839.965, 3],
            [25497236.988, 6689726.667, 3],
        ]
    ]
    for i, ring in enumerate(crds):
        for j, vtx in enumerate(ring):
            expected_crd = expected_crds[i][j]
            assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
            assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
            assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_ewkb_multipoint():
    """Test multipoint reprojection"""
    hex_string = "010400008003000000010100008000000000000024400000000000003440000000000000000001010000800000000000002e4000000000000039400000000000001440010100008000000000000034400000000000003e400000000000002440"
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPoint"
    crds = result.get("coordinates", [])
    expected_crds = [
        [1113194.908, 2273030.927, 0],
        [1669792.362, 2875744.624, 5],
        [2226389.816, 3503549.844, 10],
    ]
    for i, vtx in enumerate(crds):
        expected_crd = expected_crds[i]
        assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
        assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
        assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_ewkb_multilinestring():
    """Test multilinestring reprojection"""
    hex_string = "0105000080020000000102000080030000000000000000002440000000000000344000000000000000000000000000002e400000000000003940000000000000144000000000000034400000000000003e4000000000000024400102000080020000000000000000003e4000000000000044400000000000000000000000000080414000000000008046400000000000001440"
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiLineString"
    crds = result.get("coordinates", [])
    expected_crds = [
        [
            [1113194.908, 2273030.927, 0],
            [1669792.362, 2875744.624, 5],
            [2226389.816, 3503549.844, 10],
        ],
        [[3339584.724, 4865942.28, 0], [3896182.178, 5621521.486, 5]],
    ]

    for i, line in enumerate(crds):
        for j, vtx in enumerate(line):
            expected_crd = expected_crds[i][j]
            assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
            assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
            assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_ewkb_multipolygon():
    """Test multipolygon reprojecting multipolygon from Webmercator to UTM N 31"""
    hex_string = "01060000800100000001030000800100000004000000a01a2fdd1e67114191ed7cff238f5941000000000000000052b81e0517671141931804ce228f594100000000000000009cc420b0036711417b14ae1f238f59410000000000000000a01a2fdd1e67114191ed7cff238f59410000000000000000"
    result = wkbparse.ewkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=3857, to_srid=32631
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPolygon"
    crds = result.get("coordinates", [])
    expected_crds = [
        [
            [
                [469514.448, 5699270.597, 0],
                [469513.206, 5699267.632, 0],
                [469510.193, 5699268.445, 0],
                [469514.448, 5699270.597, 0],
            ]
        ]
    ]

    for i, poly in enumerate(crds):
        for j, ring in enumerate(poly):
            for k, vtx in enumerate(ring):
                expected_crd = expected_crds[i][j][k]
                assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
                assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
                assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


# On-the-fly TWKB to GeoJSON reprojection
def test_reproject_twkb_point():
    """Test point reprojection from WGS84 latlng to Webmercator"""
    hex_string = "610805d00fa01f50"
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "Point"
    crds = result.get("coordinates", [])
    assert crds[0] == pytest.approx(111319.491, abs=0.001)
    assert crds[1] == pytest.approx(222684.209, abs=0.001)
    assert crds[2] == pytest.approx(4.0, abs=0.001)


def test_reproject_twkb_line():
    """Test linestring reprojection from WGS84 latlng to Webmercator"""
    hex_string = "42080902c8019003e807880ea814c81a"  # pragma: allowlist secret
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "LineString"
    crds = result.get("coordinates", [])
    pnt1 = crds[0]
    assert pnt1[0] == pytest.approx(111319.491, abs=0.001)
    assert pnt1[1] == pytest.approx(222684.209, abs=0.001)
    assert pnt1[2] == pytest.approx(5.0, abs=0.001)

    pnt2 = crds[1]
    assert pnt2[0] == pytest.approx(1113194.908, abs=0.001)
    assert pnt2[1] == pytest.approx(1689200.14, abs=0.001)
    assert pnt2[2] == pytest.approx(22.0, abs=0.001)


def test_reproject_twkb_polygon():
    """Test polygon reprojection from WGS84 latlng to Webmercator"""
    hex_string = "4308090104d00fa01f00e807e807e807e807e807e807cf0fcf0fcf0f"  # pragma: allowlist secret
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "Polygon"
    crds = result.get("coordinates", [])

    expected_crds = [
        [
            [1113194.908, 2273030.927, 0],
            [1669792.362, 2875744.624, 5],
            [2226389.816, 3503549.844, 10],
            [1113194.908, 2273030.927, 0],
        ]
    ]

    for i, ring in enumerate(crds):
        for j, vtx in enumerate(ring):
            expected_crd = expected_crds[i][j]
            assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
            assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
            assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_twkb_multipoint():
    """Test multipoint reprojection"""
    hex_string = "44080903d00fa01f00e807e807e807e807e807e807"
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPoint"
    crds = result.get("coordinates", [])
    expected_crds = [
        [1113194.908, 2273030.927, 0],
        [1669792.362, 2875744.624, 5],
        [2226389.816, 3503549.844, 10],
    ]
    for i, vtx in enumerate(crds):
        expected_crd = expected_crds[i]
        assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
        assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
        assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_twkb_multilinestring():
    """Test multilinestring reprojection"""
    hex_string = (
        "4508090203d00fa01f00e807e807e807e807e807e80702d00fd00fcf0fe807e807e807"
    )
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=4326, to_srid=3857
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiLineString"
    crds = result.get("coordinates", [])
    expected_crds = [
        [
            [1113194.908, 2273030.927, 0],
            [1669792.362, 2875744.624, 5],
            [2226389.816, 3503549.844, 10],
        ],
        [[3339584.724, 4865942.28, 0], [3896182.178, 5621521.486, 5]],
    ]

    for i, line in enumerate(crds):
        for j, vtx in enumerate(line):
            expected_crd = expected_crds[i][j]
            assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
            assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
            assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


def test_reproject_twkb_multipolygon():
    """Test multipolygon reprojecting multipolygon from Webmercator to UTM N 31"""
    hex_string = "660801010104c8d0f58f02f0c9e4f53100d11ec94a00c14bf81300946ad23600"  # pragma: allowlist secret
    result = wkbparse.twkb_to_geojson(
        bytes.fromhex(hex_string), from_srid=3857, to_srid=32631
    )
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPolygon"
    crds = result.get("coordinates", [])
    expected_crds = [
        [
            [
                [469514.448, 5699270.597, 0],
                [469513.206, 5699267.632, 0],
                [469510.193, 5699268.445, 0],
                [469514.448, 5699270.597, 0],
            ]
        ]
    ]

    for i, poly in enumerate(crds):
        for j, ring in enumerate(poly):
            for k, vtx in enumerate(ring):
                expected_crd = expected_crds[i][j][k]
                assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
                assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
                assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)


# GeoJSON-GeoJSON reprojection


def test_reproject_geojson_linestring():
    """Test reprojecting GeoJSON linestring"""
    linezm = {
        "type": "LineString",
        "coordinates": [
            [
                24.96866776733461,
                60.33517409488283,
                20.0,
                0.5,
            ],
            [
                24.942231915284196,
                60.33619359369882,
                20.0,
                0.5,
            ],
            [
                24.92025925931813,
                60.32880150560584,
                20.0,
                0.5,
            ],
            [
                24.899878718286544,
                60.31867284247099,
                20.0,
                0.5,
            ],
            [
                24.89910449044379,
                60.312668337214234,
                20.0,
                0.5,
            ],
            [
                24.90391100899734,
                60.30340004464932,
                20.0,
                0.5,
            ],
            [
                24.92193545357864,
                60.301529061178684,
                20.0,
                0.5,
            ],
        ],
    }

    line_3879 = wkbparse.reproject_geojson(linezm, from_srid=4326, to_srid=3879)
    crds = line_3879.get("coordinates", [])
    expected_crds = [
        [25498269.377, 6691416.696, 20, 0.5],
        [25496809.302, 6691531.273, 20, 0.5],
        [25495594.694, 6690708.927, 20, 0.5],
        [25494467.051, 6689581.951, 20, 0.5],
        [25494423.241, 6688913.009, 20, 0.5],
        [25494687.406, 6687879.963, 20, 0.5],
        [25495683.699, 6687670.187, 20, 0.5],
    ]
    for i, vtx in enumerate(crds):
        expected_crd = expected_crds[i]
        assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
        assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
        assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)
        assert vtx[3] == pytest.approx(expected_crd[3], abs=0.001)

    line_3067 = wkbparse.reproject_geojson(line_3879, to_srid=3067)
    crds = line_3067.get("coordinates", [])
    expected_crds = [
        [387857.025, 6690467.368, 20, 0.5],
        [386401.453, 6690626.138, 20, 0.5],
        [385162.759, 6689841.199, 20, 0.5],
        [384001.732, 6688749.197, 20, 0.5],
        [383937.673, 6688082.048, 20, 0.5],
        [384170.337, 6687041.713, 20, 0.5],
        [385159.577, 6686801.882, 20, 0.5],
    ]
    for i, vtx in enumerate(crds):
        expected_crd = expected_crds[i]
        assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
        assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
        assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)
        assert vtx[3] == pytest.approx(expected_crd[3], abs=0.001)

    line_4326 = wkbparse.reproject_geojson(line_3067, to_srid=4326)
    crds = line_4326.get("coordinates", [])
    expected_crds = linezm.get("coordinates", [])
    for i, vtx in enumerate(crds):
        expected_crd = expected_crds[i]
        assert vtx[0] == pytest.approx(expected_crd[0], abs=0.001)
        assert vtx[1] == pytest.approx(expected_crd[1], abs=0.001)
        assert vtx[2] == pytest.approx(expected_crd[2], abs=0.001)
        assert vtx[3] == pytest.approx(expected_crd[3], abs=0.001)
