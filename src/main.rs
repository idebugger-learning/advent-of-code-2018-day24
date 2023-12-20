use crate::group::{Group, GroupKind};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

mod group;

#[derive(Debug)]
struct Fight {
    groups: Vec<(usize, Group)>,
}

impl Fight {
    fn plan_attack(&mut self) -> HashMap<usize, usize> {
        self.groups.sort_unstable_by(|(_, group_a), (_, group_b)| {
            match group_a
                .get_effective_power()
                .cmp(&group_b.get_effective_power())
            {
                Ordering::Equal => {}
                Ordering::Less => return Ordering::Greater,
                Ordering::Greater => return Ordering::Less,
            }

            group_a.initiative.cmp(&group_b.initiative)
        });

        let mut attack_plan: HashMap<usize, usize> = HashMap::new();
        let mut targets: HashSet<usize> = HashSet::new();
        for (from, attacker) in &self.groups {
            let enemy_type = attacker.kind.get_enemy();
            let enemy = self
                .groups
                .iter()
                .filter(|(_, group)| group.kind == enemy_type)
                .filter(|(to, _)| !targets.contains(to))
                .max_by(|(_, group_a), (_, group_b)| {
                    let damage_to_a = attacker.get_damage_to(group_a);
                    let damage_to_b = attacker.get_damage_to(group_b);
                    match damage_to_a.cmp(&damage_to_b) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }

                    let power_a = group_a.get_effective_power();
                    let power_b = group_b.get_effective_power();
                    match power_a.cmp(&power_b) {
                        Ordering::Equal => {}
                        ord => return ord,
                    }

                    group_a.initiative.cmp(&group_b.initiative)
                });

            if let Some((to, target)) = enemy {
                if attacker.get_damage_to(target) > 0 {
                    attack_plan.insert(*from, *to);
                    targets.insert(*to);
                }
            };
        }

        attack_plan
    }

    fn attack(&mut self, plan: &HashMap<usize, usize>) {
        self.groups.sort_unstable_by(|(_, group_a), (_, group_b)| {
            match group_a.initiative.cmp(&group_b.initiative) {
                Ordering::Equal => Ordering::Equal,
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
            }
        });

        let attackers: Vec<_> = self.groups.clone().into_iter().map(|(i, _)| i).collect();
        for ref attacker_id in attackers {
            let Some(target_id) = plan.get(attacker_id) else {
                continue;
            };
            let Some((_, target_group)) = self.groups.iter().find(|(i, _)| target_id == i) else {
                panic!();
            };
            let Some((_, attack_group)) = self.groups.iter().find(|(i, _)| attacker_id == i) else {
                continue;
            };

            let damage = attack_group.get_damage_to(target_group);
            let Some((_, target_group)) = self.groups.iter_mut().find(|(i, _)| target_id == i)
            else {
                continue;
            };
            target_group.damage(damage);

            self.clear_dead_groups();
        }
    }

    fn clear_dead_groups(&mut self) {
        self.groups = self
            .groups
            .clone()
            .into_iter()
            .filter(|(_, group)| group.units_count != 0)
            .collect()
    }

    fn run_fight(&mut self) {
        loop {
            let count_of_immune = self
                .groups
                .iter()
                .filter(|(_, group)| group.kind == GroupKind::Immune)
                .count();
            let count_of_infection = self
                .groups
                .iter()
                .filter(|(_, group)| group.kind == GroupKind::Infection)
                .count();
            if count_of_immune == 0 || count_of_infection == 0 {
                return;
            }

            let plan = self.plan_attack();
            self.attack(&plan);
        }
    }
}

fn main() {
    // let mut fight = Fight {
    //     groups: vec![
    //         (
    //             0,
    //             Group::new(
    //                 GroupKind::Immune,
    //                 17,
    //                 5390,
    //                 4507,
    //                 String::from("fire"),
    //                 2,
    //                 HashSet::from([String::from("radiation"), String::from("bludgeoning")]),
    //                 HashSet::from([]),
    //             ),
    //         ),
    //         (
    //             1,
    //             Group::new(
    //                 GroupKind::Immune,
    //                 989,
    //                 1274,
    //                 25,
    //                 String::from("slashing"),
    //                 3,
    //                 HashSet::from([String::from("bludgeoning"), String::from("slashing")]),
    //                 HashSet::from([String::from("fire")]),
    //             ),
    //         ),
    //         (
    //             2,
    //             Group::new(
    //                 GroupKind::Infection,
    //                 801,
    //                 4706,
    //                 116,
    //                 String::from("bludgeoning"),
    //                 1,
    //                 HashSet::from([String::from("radiation")]),
    //                 HashSet::from([]),
    //             ),
    //         ),
    //         (
    //             3,
    //             Group::new(
    //                 GroupKind::Infection,
    //                 4485,
    //                 2961,
    //                 12,
    //                 String::from("slashing"),
    //                 4,
    //                 HashSet::from([String::from("fire"), String::from("cold")]),
    //                 HashSet::from([String::from("radiation")]),
    //             ),
    //         ),
    //     ],
    // };

    let mut fight = Fight {
        groups: vec![
            (
                0,
                Group::new(
                    GroupKind::Immune,
                    554,
                    8034,
                    124,
                    String::from("bludgeoning"),
                    2,
                    HashSet::from([String::from("cold")]),
                    HashSet::from([String::from("slashing")]),
                ),
            ),
            (
                1,
                Group::new(
                    GroupKind::Immune,
                    285,
                    3942,
                    107,
                    String::from("bludgeoning"),
                    6,
                    HashSet::from([String::from("cold")]),
                    HashSet::from([]),
                ),
            ),
            (
                2,
                Group::new(
                    GroupKind::Immune,
                    4470,
                    7895,
                    17,
                    String::from("bludgeoning"),
                    1,
                    HashSet::from([String::from("radiation")]),
                    HashSet::from([String::from("bludgeoning")]),
                ),
            ),
            (
                3,
                Group::new(
                    GroupKind::Immune,
                    4705,
                    8128,
                    14,
                    String::from("bludgeoning"),
                    8,
                    HashSet::from([String::from("slashing")]),
                    HashSet::from([]),
                ),
            ),
            (
                4,
                Group::new(
                    GroupKind::Immune,
                    3788,
                    7504,
                    17,
                    String::from("cold"),
                    3,
                    HashSet::from([String::from("cold"), String::from("slashing")]),
                    HashSet::from([]),
                ),
            ),
            (
                5,
                Group::new(
                    GroupKind::Immune,
                    7087,
                    2733,
                    3,
                    String::from("slashing"),
                    14,
                    HashSet::from([String::from("bludgeoning")]),
                    HashSet::from([]),
                ),
            ),
            (
                6,
                Group::new(
                    GroupKind::Immune,
                    23,
                    7234,
                    3132,
                    String::from("fire"),
                    15,
                    HashSet::from([]),
                    HashSet::from([]),
                ),
            ),
            (
                7,
                Group::new(
                    GroupKind::Immune,
                    818,
                    7188,
                    80,
                    String::from("fire"),
                    19,
                    HashSet::from([String::from("fire")]),
                    HashSet::from([String::from("radiation"), String::from("slashing")]),
                ),
            ),
            (
                8,
                Group::new(
                    GroupKind::Immune,
                    3233,
                    3713,
                    10,
                    String::from("radiation"),
                    20,
                    HashSet::from([String::from("radiation")]),
                    HashSet::from([String::from("cold")]),
                ),
            ),
            (
                9,
                Group::new(
                    GroupKind::Immune,
                    1011,
                    8135,
                    75,
                    String::from("radiation"),
                    12,
                    HashSet::from([String::from("fire")]),
                    HashSet::from([
                        String::from("slashing"),
                        String::from("cold"),
                        String::from("bludgeoning"),
                    ]),
                ),
            ),
            (
                10,
                Group::new(
                    GroupKind::Infection,
                    136,
                    37513,
                    492,
                    String::from("cold"),
                    18,
                    HashSet::from([String::from("radiation")]),
                    HashSet::from([]),
                ),
            ),
            (
                11,
                Group::new(
                    GroupKind::Infection,
                    4811,
                    5863,
                    2,
                    String::from("radiation"),
                    17,
                    HashSet::from([String::from("radiation"), String::from("cold")]),
                    HashSet::from([String::from("slashing")]),
                ),
            ),
            (
                12,
                Group::new(
                    GroupKind::Infection,
                    4057,
                    9812,
                    4,
                    String::from("bludgeoning"),
                    11,
                    HashSet::from([String::from("slashing")]),
                    HashSet::from([]),
                ),
            ),
            (
                13,
                Group::new(
                    GroupKind::Infection,
                    2828,
                    30926,
                    19,
                    String::from("cold"),
                    7,
                    HashSet::from([String::from("cold")]),
                    HashSet::from([String::from("bludgeoning")]),
                ),
            ),
            (
                14,
                Group::new(
                    GroupKind::Infection,
                    2311,
                    20627,
                    17,
                    String::from("slashing"),
                    5,
                    HashSet::from([]),
                    HashSet::from([String::from("slashing")]),
                ),
            ),
            (
                15,
                Group::new(
                    GroupKind::Infection,
                    1622,
                    30824,
                    34,
                    String::from("bludgeoning"),
                    4,
                    HashSet::from([String::from("slashing"), String::from("bludgeoning")]),
                    HashSet::from([]),
                ),
            ),
            (
                16,
                Group::new(
                    GroupKind::Infection,
                    108,
                    8628,
                    139,
                    String::from("slashing"),
                    13,
                    HashSet::from([]),
                    HashSet::from([]),
                ),
            ),
            (
                17,
                Group::new(
                    GroupKind::Infection,
                    1256,
                    51819,
                    63,
                    String::from("radiation"),
                    16,
                    HashSet::from([]),
                    HashSet::from([String::from("slashing"), String::from("cold")]),
                ),
            ),
            (
                18,
                Group::new(
                    GroupKind::Infection,
                    3681,
                    21469,
                    11,
                    String::from("cold"),
                    9,
                    HashSet::from([String::from("slashing")]),
                    HashSet::from([String::from("cold"), String::from("bludgeoning")]),
                ),
            ),
            (
                19,
                Group::new(
                    GroupKind::Infection,
                    7289,
                    6935,
                    1,
                    String::from("fire"),
                    10,
                    HashSet::from([String::from("slashing"), String::from("bludgeoning")]),
                    HashSet::from([]),
                ),
            ),
        ],
    };

    fight.run_fight();

    println!("{:#?}", fight);

    let win_units: usize = fight
        .groups
        .iter()
        .map(|(_, group)| group.units_count)
        .sum();
    println!("Sum of winner units: {}", win_units);
}
