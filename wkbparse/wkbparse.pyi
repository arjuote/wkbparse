from typing import Any, Dict, Optional

def twkb_to_geojson(
    data: bytes, from_srid: Optional[int] = None, to_srid: Optional[int] = None
) -> Dict[str, Any]:
    """Convert data containing TWKB-bytes into a GeoJSON-like dictionary.
    Optionally reproject from `from_srid` to `to_srid` by providing integers matching EPSG codes.
    NOTE: Reprojection requires using package `wkbparse-proj` and a recent Proj installed on the system.
    """

def ewkb_to_geojson(
    data: bytes, from_srid: Optional[int] = None, to_srid: Optional[int] = None
) -> Dict[str, Any]:
    """Convert data containing EWKB-bytes into a GeoJSON-like dictionary.
    Optionally reproject from `from_srid` to `to_srid` by providing integers matching EPSG codes.
    NOTE: Reprojection requires using package `wkbparse-proj` and a recent Proj installed on the system.
    """

def geojson_to_ewkb(
    data: Dict[str, Any], from_srid: Optional[int] = None, to_srid: Optional[int] = None
) -> bytes:
    """Convert GeoJSON-like dictionary into EWKB-bytes.
    Optionally reproject from `from_srid` to `to_srid` by providing integers matching EPSG codes.
    NOTE: Reprojection requires using package `wkbparse-proj` and a recent Proj installed on the system.
    """

def twkb_to_ewkb(data: bytes) -> bytes:
    """Convert TWKB-bytes into EWKB-bytes"""

def reproject_geojson(
    data: Dict[str, Any], to_srid: int, from_srid: Optional[int] = None
) -> Dict[str, Any]:
    """Transform a GeoJSON geometry into another coordinate system.
    Provide from_srid and to_srid as integers that match EPSG-codes.
    `from_srid` may be omitted if the input geometry already contains SRID definition.
    NOTE: Reprojection requires using package `wkbparse-proj` and a recent Proj installed on the system.
    """
