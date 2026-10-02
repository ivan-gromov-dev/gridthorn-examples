# Grid placement

Headless example of the provisional `gridthorn::grid` placement API. Run from
the examples repository:

```console
cargo run -p gridthorn_example_grid_placement
```

The example validates a two-cell building against caller-owned terrain, previews
without reservation, places at a negative anchor, selects through a non-anchor
cell in square and isometric projections, rejects a conflicting move while
preserving occupancy, moves with self-overlap, and removes the building.
Object identities belong to the game; this service does not spawn ECS entities.
Games must revalidate terrain and commit authoritative placement at a fixed tick.
