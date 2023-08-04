"""Tests for python side EWKB-parsing"""

import json
from pathlib import Path
from time import time
import wkbparse


def test_parse_ewkb_point():
    """Test point parsing"""
    hex_string = "0101000080000000000000F03F00000000000000400000000000001040"
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "Point"
    crds = result.get("coordinates", [])
    assert crds[0] == 1.0
    assert crds[1] == 2.0
    assert crds[2] == 4.0

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_ewkb_line():
    """Test linestring parsing"""
    hex_string = "010200008002000000000000000000f03f0000000000000040000000000000144000000000000024400000000000002e400000000000003640"
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
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

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_ewkb_polygon():
    """Test polygon parsing"""
    hex_string = "01030000a0e610000001000000070000003333333333f33840295c8fc2f5284e400000000000000840ae47e17a14ee384048e17a14ae274e4000000000000008403333333333f3384048e17a14ae274e4000000000000008407b14ae47e1fa384048e17a14ae274e4000000000000008403d0ad7a370fd3840295c8fc2f5284e4000000000000008407b14ae47e1fa38400ad7a3703d2a4e4000000000000008403333333333f33840295c8fc2f5284e400000000000000840"  # pragma: allowlist secret
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "Polygon"
    assert result.get("crs") == 4326
    crds = result.get("coordinates", [])
    expected_crds = "[[[24.95, 60.32, 3.0], [24.93, 60.31, 3.0], [24.95, 60.31, 3.0], [24.98, 60.31, 3.0], [24.99, 60.32, 3.0], [24.98, 60.33, 3.0], [24.95, 60.32, 3.0]]]"
    print(json.dumps(crds))
    assert json.dumps(crds) == expected_crds

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_ewkb_multipoint():
    """Test multipoint parsing"""
    hex_string = "010400008003000000010100008000000000000024400000000000003440000000000000000001010000800000000000002e4000000000000039400000000000001440010100008000000000000034400000000000003e400000000000002440"
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPoint"
    crds = result.get("coordinates", [])
    expected_crds = "[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]]"
    assert json.dumps(crds) == expected_crds

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_ewkb_multilinestring():
    """Test multilinestring parsing"""
    hex_string = "0105000080020000000102000080030000000000000000002440000000000000344000000000000000000000000000002e400000000000003940000000000000144000000000000034400000000000003e4000000000000024400102000080020000000000000000003e4000000000000044400000000000000000000000000080414000000000008046400000000000001440"
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiLineString"
    crds = result.get("coordinates", [])
    expected_crds = "[[[10.0, 20.0, 0.0], [15.0, 25.0, 5.0], [20.0, 30.0, 10.0]], [[30.0, 40.0, 0.0], [35.0, 45.0, 5.0]]]"
    assert json.dumps(crds) == expected_crds

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_ewkb_multipolygon():
    """Test multipolygon parsing"""
    hex_string = "01060000800100000001030000800100000004000000a01a2fdd1e67114191ed7cff238f5941000000000000000052b81e0517671141931804ce228f594100000000000000009cc420b0036711417b14ae1f238f59410000000000000000a01a2fdd1e67114191ed7cff238f59410000000000000000"
    result = wkbparse.ewkb_to_geojson(bytes.fromhex(hex_string))
    assert isinstance(result, dict)
    assert result.get("type") == "MultiPolygon"
    crds = result.get("coordinates", [])
    expected_crds = "[[[[285127.716, 6700175.992, 0.0], [285125.755, 6700171.219, 0.0], [285120.922, 6700172.495, 0.0], [285127.716, 6700175.992, 0.0]]]]"
    assert json.dumps(crds) == expected_crds

    encoded = wkbparse.geojson_to_ewkb(result)
    hex_result = bytes.hex(encoded)
    assert hex_string.lower() == hex_result.lower()


def test_parse_medium_polygon():
    """Test medium sized real world polygon parsing parsing"""
    hex_string = Path("./tests/data/medium-ewkb-polygon-example.ewkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.ewkb_to_geojson(data)
    print("done in {:2f} us".format((time() - time_start) * 1_000_000))
    assert result.get("type") == "Polygon"
    rings = result.get("coordinates", [])
    crds = [crd for crds in rings for crd in crds]
    assert len(crds) == 32


def test_parse_large_polygon():
    """Test large sized real world polygon parsing parsing"""
    hex_string = Path("./tests/data/large-ewkb-polygon-example.ewkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.ewkb_to_geojson(data)
    print("done in {:2f} ms".format((time() - time_start) * 1000))
    assert result.get("type") == "Polygon"
    rings = result.get("coordinates", [])
    crds = [crd for crds in rings for crd in crds]
    assert len(crds) > 250000


def test_parse_large_multipolygon():
    """Test large sized real world multipolygon parsing parsing"""
    hex_string = Path("./tests/data/large-ewkb-multipolygon-example.ewkb").read_text(
        encoding="utf8"
    )
    data = bytes.fromhex(hex_string)
    time_start = time()
    result = wkbparse.ewkb_to_geojson(data)
    print("done in {:2f} ms".format((time() - time_start) * 1000))
    assert result.get("type") == "MultiPolygon"
    polygons = result.get("coordinates", [])
    crds = [crd for rings in polygons for crds in rings for crd in crds]
    assert len(crds) > 300000


if __name__ == "__main__":
    test_parse_large_multipolygon()
