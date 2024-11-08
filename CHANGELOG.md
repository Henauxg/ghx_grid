# Changelog

## Version 0.5.0 (TBD)

- Updated to use Bevy 0.15
- Changed `NodeRef` trait to be generic over the concrete `Grid` type
- Implemented `NodeRef` for `CartesianPosition`

## Version 0.4.1 (2024-11-07)

- Fixed the `total_size` implementation in `CartesianGrid`
- Fixed some code documentation/links

## Version 0.4.0 (2024-10-24)

- Updated to use Bevy 0.14
- Ignore `PhantomData` with the `reflect` and `serde` features
- Derive `Hash` on `CartesianPosition`
- Set `index_from_pos` and `pos_from_index` to `pub` on `CartesianGrid`
- Removed `nodes` method on `GridData` 
- Added `indexes` method to `GridData`
- Added `iter` and `iter_mut` to `GridData`
- Added `get_from_pos` and `get_from_pos_mut` to Cartesian `GridData`
- Added a `flood_fill` algorithm for `Cartesian2D` `GridData`
- Added a `serde` feature to derive `Serialize` and `Deserialize` on most types

## Version 0.3.2 (2024-11-07)

(backport of the 0.4.1 fix) 
- Fix the `total_size` implementation in `CartesianGrid`

## Version 0.3.1 (2024-09-21)

- Fix the `reflect` feature

## Version 0.3.0 (2024-06-13)

Support more grid types than just cartesian grids:

  - Added `DirectionTrait`
  - Added `DirectionIndex` type alias
  - Changed `CoordinateSystem` trait:
    - Added `directions_count` function
    - Added an associated type `Direction`
  - Added `Grid` generic trait to represent a generic topology
  - Added a generic `Grid` parameter to `GridData`
  - Added a `cartesian` module to contazin cartesian specific implementations:
    - Changed `GridDefinition` to `CartesianGrid`
    - Added `CartesianCoordinates` trait

## Version 0.2.0 (2024-03-06)

- Update to use Bevy 0.13
