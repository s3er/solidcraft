//! Multi-body features (`Combine`, `Move`, `Scale`, `Remove`, `Align`, `BoundingSolid`,
//! `BoundaryFill`, `Stitch`, `Thicken`) and boolean target lists (`Extrude`, `Revolve`, `Loft`,
//! `Sweep`, `Pipe`, `Emboss`, `Coil`, `CellGrid`, `Thicken`) can reference upstream features by
//! id (`body_features`, `tool_features`, `target_features`) so evaluation dynamically resolves
//! every live body created by those features when upstream counts change. Older designs omit
//! these fields and default them to empty, keeping static body-name selection unchanged. The last
//! release opens a design carrying these fields with a warning and preserves what it cannot read.
