from typing import Any, Dict

def twkb_to_geojson(data: bytes) -> Dict[str, Any]:
    """Convert data containing TWKB-bytes into a GeoJSON-like dictionary"""

def ewkb_to_geojson(data: bytes) -> Dict[str, Any]:
    """Convert data containing EWKB-bytes into a GeoJSON-like dictionary"""

def geojson_to_ewkb(data: Dict[str, Any]) -> bytes:
    """Convert GeoJSON-like dictionary into EWKB-bytes"""

def twkb_to_ewkb(data: bytes) -> bytes:
    """Convert TWKB-bytes into EWKB-bytes"""
