# CHANGELOG

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/) and [Keep a Changelog](http://keepachangelog.com/).



## Unreleased
---

### New

### Changes

### Fixes

### Breaks


## 0.2.1 - (2025-02-09)
---

### Changes
* Proj transform objects are now reused between python calls in order to cut down the rather large overhead of creating one


## 0.2.0 - (2025-01-27)

### New
* Added reprojection support with proj behind `proj` feature flag
* Publish separate package wkbparse-proj with the reprojection feature enabled


## 0.1.1 - (2024-10-28)
---

### Fixes
* Add support for M-coordinate if present alongside with Z


## 0.1.0 - (2023-07-02)
---

### New
* Initial fork
