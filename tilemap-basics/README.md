# Tilemap basics

Provisional headless SDK example for sparse tilemaps, ordered layers, signed
chunks, square/isometric picking, and orthographic camera conversion.

Run `cargo run -p gridthorn_example_tilemap_basics` from the examples workspace.
Assertions verify the roof wins picking, hiding it exposes ground, and deleting
the last tile releases its chunk. No window, GPU, or tile rendering is used.
