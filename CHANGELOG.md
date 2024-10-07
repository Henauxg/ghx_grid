# Changelog

## Version 0.4.0 (TBD)

- Update to use Bevy 0.14
- Ignore `PhantomData` with the `reflect` and `serde` features
- Add a `serde` feature to derive `Serialize` and `Deserialize` on most types
- Derive `Hash` on `CartesianPosition`
- Set `index_from_pos` and `pos_from_index` to `pub` on `CartesianGrid`
- Add `get_from_pos` and `get_from_pos_mut` to Cartesian `GridData`


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
