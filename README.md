# wkbparse

`wkbparse` is a Python module written in Rust for parsing [EWKB](https://postgis.net/docs/using_postgis_dbmanagement.html#EWKB_EWKT) and [TWKB](https://github.com/TWKB/Specification/blob/master/twkb.md) geometries into GeoJSON strings or GeoJSON-like Python dictionaries in a performant fashion.

Original EWKB/TWKB encoding code forked from [https://github.com/Mortal/rust-ewkb](Mortal/rust-ewkb) which in turn originates from [https://github.com/andelf/rust-postgis](andelf/rust-postgis).

`wkbparse` is developed mainly for usage with PostGIS and has been tested with geometries generated with one. However, it has no explicit dependencies towards PostgreSQL or PostGIS and can be used with EWKB/TWKB geometries originating from any system. In such case it is advisable to validate results carefully before using for anything serious.

It supports reading and writing ZM geometries as well, even though GeoJSON specification doesn't really recognize the M coordinate. The M coordinate is simply output as the fourth coordinate in a vertex. Respectively, input GeoJSON dictionaries with four coordinates in a vertex are treated as ZM geometries.

## Motivation

The main rationale behind this library is to offload compute related to geometry encoding from the database to the application and to minimize data transfer between them. This can be achieved by favoring native EWKB geometries or better-yet the transfer-optimized TWKB-geometries instead of making the database encode the data in some text-based format such as WKT or GeoJSON and sending that over the wire.

The benefits may be especially noticeable when dealing with large geometries with lots of vertices. E.g. the size of a 300 000 vertex multipolygon as EWKB is ~10 MB while as TWKB (1 cm precision) it is ~2 MB. Letting the database encode such geometry as GeoJSON and transferring it over the wire takes a long time (anecdotally way longer than a typical API timeout). Deserializing such MultiPolygon using `wkbparse` takes ~150 ms on an AMD Ryzen 4900 HS laptop and the transfer of TWKB is much quicker than of the other formats.

## Installation

Pre-built wheels are available for the following platforms and python versions:

Python versions: `[3.8, 3.9, 3.10, 3.11, 3.12]`

Platforms: Linux `[x86_64, x86, aarch64, armv7, s390x, ppc64le]`, Windows: `[x64, x86]`, MacOS: `[x86_64, aarch64]`

Install by saying `pip install wkbparse`.

Supported python version is >=3.8.

Tested on Python versions 3.8, 3.9, 3.10, 3.11 on Linux x86_64.

## Reprojection

Install `wkbparse-proj` instead of `wkbparse` to enable coordinate reprojection using the [Proj](https://github.com/OSGeo/PROJ) project.

Using the reprojection-enabled package allows one to pass in `from_srid` and `to_srid` as integers corresponding to EPSG-codes to many of the functions. The `from_srid` argument may be omitted if the source EWKB bytes or GeoJSON-dictionary data already contains the SRID. TWKB data never contains the SRID.

Pre-installed proj library must be present on the system for this feature to work. See [Proj installation](https://proj.org/en/stable/install.html).

Using `wkbparse-proj` bumps up the package size from ~250 kilobytes to ~10 megabytes due to rather large size of the Proj C++-dependency.

NOTE: Separate package is used instead of python "extras" within a single package due to extras not interacting nicely with wheels built with different Rust feature flags. This approach allows us to have a single codebase and a surefire way of selecting the appropriate package to avoid downloading redundant large dependencies.

## Usage

This module implements the following functionalities:

- TWKB to GeoJSON dictionary: `twkb_to_geojson`
- TWKB to EWKB: `twkb_to_ewkb`
- EWKB to GeoJSON dictionary: `ewkb_to_geojson`
- GeoJSON dictionary to EWKB: `geojson_to_ewkb`
- Reproject geojson `reproject_geojson` (only with `wkbparse-proj`)

The following is not currently implemented:

- Support for GeometryCollection types
- Encoding any data in TWKB

Example:

```python
import wkbparse

twkb_bytes = bytes.fromhex("610805d00fa01f50")
geometry = wkbparse.twkb_to_geojson(twkb_bytes)
print(geometry)
```

The result dict has the following shape:

```
{
    type: str                # GeoJSON geometry type
    crs: Optional[int]       # Spatial reference system identifier
    coordinates: list[float] # nesting depth depending on geometry type
}
```

E.g.

```python
{'type': 'Point', 'crs': None, 'coordinates': [1.0, 2.0, 4.0]}
```

To reproject data when using `wkbparse-proj` we can additionally pass in `from_srid` and `to_srid`

```python
import wkbparse

twkb_bytes = bytes.fromhex("610805d00fa01f50")
geometry = wkbparse.twkb_to_geojson(twkb_bytes, from_srid=4326, to_srid=3857)
print(geometry)
```

```python
{'type': 'Point', 'crs': 3857, 'coordinates': [111319.49079327357, 222684.20850554405, 4.0]}
```

If we already have a dictionary as above, we can reproject it with `reproject_geojson`:

```python
import wkbparse

d = {"type": "Point", "crs": 3857, "coordinates": [111319.49079327357, 222684.20850554405, 4.0]}

reprojected = wkbparse.reproject_geojson(d, to_srid=4326)
print(reprojected)
```

```python
{'type': 'Point', 'crs': 4326, 'coordinates': [0.9999999999999998, 1.9999999999999996, 4.0]}
```

Note that `from_srid` was omitted in this case as the input geometry already had the `crs` field. One may provide it anyway to override the crs.
