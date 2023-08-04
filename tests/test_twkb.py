"""Tests for python side TWKB-parsing"""

import json
from pathlib import Path
from time import time
import wkbparse


def test_parse_twkb_point():
    """Test point parsing"""
    hex_string = "610805d00fa01f50"
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "Point"
    crds = result.get("coordinates", [])
    assert crds[0] == 1.0
    assert crds[1] == 2.0
    assert crds[2] == 4.0


def test_parse_twkb_line():
    """Test linestring parsing"""
    hex_string = "42080902c8019003e807880ea814c81a"  # pragma: allowlist secret
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "LineString"
    crds = result.get("coordinates", [])
    pnt1 = crds[0]
    assert pnt1[0] == 1.0
    assert pnt1[1] == 2.0
    assert pnt1[2] == 5.0

    pnt2 = crds[1]
    assert pnt2[0] == 10.0
    assert pnt2[1] == 15.0
    assert pnt2[2] == 22.0


def test_parse_twkb_polygon():
    """Test polygon parsing"""
    hex_string = "4308090104d00fa01f00e807e807e807e807e807e807cf0fcf0fcf0f"
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "Polygon"
    crds = result.get("coordinates", [])
    expected_crds = "[[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0], [10.0, 20.0, 0.0]]]"
    assert json.dumps(crds) == expected_crds


def test_parse_twkb_multipoint():
    """Test multipoint parsing"""
    hex_string = "44080903d00fa01f00e807e807e807e807e807e807"
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPoint"
    crds = result.get("coordinates", [])
    expected_crds = "[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]]"
    assert json.dumps(crds) == expected_crds


def test_parse_twkb_multilinestring():
    """Test multilinestring parsing"""
    hex_string = (
        "4508090203d00fa01f00e807e807e807e807e807e80702d00fd00fcf0fe807e807e807"
    )
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiLineString"
    crds = result.get("coordinates", [])
    expected_crds = "[[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]], [[30.0, 40.0, 0.0], [35.0, 45.0, 5.0]]]"
    assert json.dumps(crds) == expected_crds


def test_parse_twkb_multipolygon():
    """Test multipolygon parsing"""
    hex_string = "660801010104c8d0f58f02f0c9e4f53100d11ec94a00c14bf81300946ad23600"  # pragma: allowlist secret
    result = wkbparse.twkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPolygon"
    crds = result.get("coordinates", [])
    expected_crds = "[[[[285127.716, 6700175.992, 0.0], [285125.755, 6700171.219, 0.0], [285120.922, 6700172.494999999, 0.0], [285127.716, 6700175.992, 0.0]]]]"
    assert json.dumps(crds) == expected_crds


def test_parse_medium_polygon():
    """Test medium sized real world polygon parsing parsing"""
    hex_string = Path("./tests/data/medium-twkb-polygon-example.twkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.twkb_to_geojson(data)
    print("done in {:2f} us".format((time() - time_start) * 1_000_000))
    assert result.get("type") == "Polygon"
    rings = result.get("coordinates", [])
    crds = [crd for crds in rings for crd in crds]
    assert len(crds) == 32


def test_parse_large_polygon():
    """Test large sized real world polygon parsing parsing"""
    hex_string = Path("./tests/data/large-twkb-polygon-example.twkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.twkb_to_geojson(data)
    print("done in {:2f} ms".format((time() - time_start) * 1000))
    assert result.get("type") == "Polygon"
    rings = result.get("coordinates", [])
    crds = [crd for crds in rings for crd in crds]
    assert len(crds) > 250000


def test_parse_large_multipolygon():
    """Test large sized real world multipolygon parsing parsing"""
    hex_string = Path("./tests/data/large-twkb-multipolygon-example.twkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.twkb_to_geojson(data)
    print("done in {:2f} ms".format((time() - time_start) * 1000))
    assert result.get("type") == "MultiPolygon"
    polygons = result.get("coordinates", [])
    crds = [crd for rings in polygons for crds in rings for crd in crds]
    assert len(crds) > 300000


if __name__ == "__main__":
    test_parse_twkb_multipolygon()
