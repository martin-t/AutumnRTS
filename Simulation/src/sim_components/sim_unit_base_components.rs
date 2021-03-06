use crate::common::*;
use std::collections::VecDeque;

use crate::sim_fix_math::*;

/// Unit type name
pub struct TypeNameComp {
    name: String,
}

impl TypeNameComp {
    pub fn new(name: &str) -> Self {
        TypeNameComp {
            name: String::from(name),
        }
    }
}

/// Unit id
/// Reason is that hecs ecs reuses id's, and
///this could cause some bugs in the future.
///Id component should be perfectly unique in game context
/// Also it should be owned by entities that get referenced by other entities (eg unit that gets attacked should have Id comp. Projectile that flies and ignores everything except its target doesnt need one)
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IdComp {
    id: UId,
    owner: PlayerId,
}

impl IdComp {
    pub fn new(id_counter: &mut u64, owner: &PlayerId) -> Self {
        let id = std::mem::replace(id_counter, *id_counter + 1);

        IdComp {
            id: id,
            owner: owner.clone(),
        }
    }

    #[deprecated(since = "yesterday", note = "Please use the get_id() function instead")]
    pub fn get(&self) -> &UId {
        &self.id
    }

    pub fn get_id(&self) -> &UId {
        &self.id
    }

    pub fn get_owner(&self) -> &PlayerId {
        &self.owner
    }
}

/// Location component
#[derive(Debug, PartialEq, Clone)]
pub struct PositionComp {
    pos: Pos,
}

impl PositionComp {
    pub fn new(pos: Pos) -> Self {
        PositionComp { pos: pos }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
}

/// Next position component
#[derive(Debug, PartialEq, Clone)]
pub struct NextPosComp {
    pos: Pos,
}

impl NextPosComp {
    pub fn new(pos: Pos) -> Self {
        NextPosComp { pos: pos }
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }
}

// Unit's destination component
pub struct DestinationComp {
    dest: Pos,
    updated_on: TickNum,
}

impl DestinationComp {
    pub fn new(pos: Pos) -> Self {
        DestinationComp {
            dest: pos,
            updated_on: 0,
        }
    }

    pub fn set_dest(&mut self, pos: Pos, tick: TickNum) {
        self.dest = pos;
        self.updated_on = tick;
    }

    pub fn get_dest(&self) -> &Pos {
        &self.dest
    }

    pub fn last_set(&self) -> TickNum {
        self.updated_on
    }
}

/// Unit's speed component
pub struct SpeedComponent {
    speed: FixF,
    cooldown: TickNum, // cooldown between steps.
}

impl SpeedComponent {
    pub fn new(s: FixF, cd: TickNum) -> Self {
        SpeedComponent {
            speed: s,
            cooldown: cd,
        }
    }

    pub fn get_speed(&self) -> &FixF {
        &self.speed
    }

    pub fn get_cooldown(&self) -> &TickNum {
        &self.cooldown
    }
}

// Square hitbox. W,H should be treadted as radius
pub struct CollComp {
    r: FixF,
}

impl CollComp {
    pub fn new(radius: FixF) -> Self {
        CollComp { r: radius }
    }

    pub fn get_r(&self) -> &FixF {
        &self.r
    }
}

// pathfinding pomponent. Holds positions that unit should walk to.
#[derive(Debug, PartialEq, Clone)]
pub struct PathComp {
    positions: VecDeque<Pos>,
}

impl PathComp {
    pub fn new() -> Self {
        PathComp {
            positions: VecDeque::new(),
        }
    }

    pub fn get_path(&self) -> &VecDeque<Pos> {
        &self.positions
    }

    pub fn _get_mut(&mut self) -> &mut VecDeque<Pos> {
        &mut self.positions
    }

    pub fn set(&mut self, path: VecDeque<Pos>) {
        self.positions = path;
    }

    pub fn _from_vec(&mut self, path: Vec<Pos>) {
        self.positions = VecDeque::from(path);
    }

    pub fn _is_empty(&self) -> bool {
        self.positions.len() == 0
    }

    pub fn get_next_pos(&mut self, current_pos: &Pos) -> Option<&Pos> {
        match self.positions.front() {
            None => return None,
            Some(front) => {
                if front == current_pos {
                    self.positions.pop_front();
                }
                return self.positions.front();
            }
        }
    }
}

pub struct UnitAIComp {}

impl UnitAIComp {
    pub fn new() -> Self {
        UnitAIComp {}
    }
}
