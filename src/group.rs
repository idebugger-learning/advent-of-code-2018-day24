use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupKind {
    Immune,
    Infection,
}

impl GroupKind {
    pub fn get_enemy(&self) -> Self {
        match self {
            GroupKind::Infection => GroupKind::Immune,
            GroupKind::Immune => GroupKind::Infection,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub kind: GroupKind,
    pub units_count: usize,
    pub hit_points: usize,
    pub attack_damage: usize,
    pub attack_type: String,
    pub initiative: usize,
    pub weaknesses: HashSet<String>,
    pub immunities: HashSet<String>,
}

impl Group {
    pub fn new(
        kind: GroupKind,
        units_count: usize,
        hit_points: usize,
        attack_damage: usize,
        attack_type: String,
        initiative: usize,
        weaknesses: HashSet<String>,
        immunities: HashSet<String>,
    ) -> Self {
        Group {
            kind,
            units_count,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        }
    }

    pub fn get_effective_power(&self, immune_boost: usize) -> usize {
        if self.kind == GroupKind::Immune {
            self.units_count * (self.attack_damage + immune_boost)
        } else {
            self.units_count * self.attack_damage
        }
    }

    pub fn get_damage_to(&self, target: &Group, immune_boost: usize) -> usize {
        if target.immunities.contains(&self.attack_type) {
            return 0;
        }

        let default_dmg = self.get_effective_power(immune_boost);
        if target.weaknesses.contains(&self.attack_type) {
            return default_dmg * 2;
        }

        return default_dmg;
    }

    pub fn damage(&mut self, damage: usize) {
        // let decreased_units = (damage as f32 / self.hit_points as f32).floor() as usize;
        let decreased_units = damage / self.hit_points;
        self.units_count -= decreased_units.min(self.units_count);
    }
}
