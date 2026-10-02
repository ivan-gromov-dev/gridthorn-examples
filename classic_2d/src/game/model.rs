use gridthorn::prelude::*;

pub(crate) const GEMS: [[i16; 2]; 4] = [[-240, 120], [240, 120], [240, -120], [-240, -120]];
pub(crate) const WALLS: [[i16; 2]; 2] = [[-100, 0], [100, 0]];

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Arena {
    pub position: [i16; 2],
    pub collected: [bool; 4],
}

impl Arena {
    pub fn step(&mut self, movement: [i16; 2]) -> usize {
        let next = [
            (self.position[0] + movement[0] * 3).clamp(-360, 360),
            (self.position[1] + movement[1] * 3).clamp(-200, 200),
        ];
        let player = collider(next, [12.0, 12.0]);
        if !WALLS
            .iter()
            .any(|wall| gridthorn::overlaps(player, collider(*wall, [24.0, 65.0])))
        {
            self.position = next;
        }
        let mut count = 0;
        for (index, gem) in GEMS.iter().enumerate() {
            if !self.collected[index]
                && gridthorn::overlaps(
                    collider(self.position, [12.0, 12.0]),
                    collider(*gem, [12.0, 12.0]),
                )
            {
                self.collected[index] = true;
                count += 1;
            }
        }
        count
    }
}

fn collider(position: [i16; 2], half: [f32; 2]) -> Collider2d {
    Aabb2d::new(
        Vec2::new(f32::from(position[0]), f32::from(position[1])),
        Vec2::new(half[0], half[1]),
    )
    .expect("finite game geometry")
    .into()
}

pub(crate) struct Session {
    pub entity: Option<gridthorn::EntityId>,
    pub movement: [i16; 2],
    pub button: UiButton,
    pub hovered: bool,
    pub frames: u32,
}

pub(crate) fn state(name: &str) -> GameStateId {
    GameStateId::new(name).expect("named game state")
}
pub(crate) fn scene(name: &str) -> SceneId {
    SceneId::new(name).expect("named game scene")
}

#[cfg(test)]
mod test;
