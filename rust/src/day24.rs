use aoc2018::read_input;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    army: Army,
    units: i32,
    hp: i32,
    attack_damage: i32,
    attack_type: String,
    initiative: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Army {
    ImmuneSystem,
    Infection,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.units * self.attack_damage
    }
    
    fn damage_to(&self, target: &Group) -> i32 {
        if target.immunities.contains(&self.attack_type) {
            0
        } else if target.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

fn parse_input(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut current_army = Army::ImmuneSystem;
    let mut id = 0;
    
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        } else if line.starts_with("Immune System:") {
            current_army = Army::ImmuneSystem;
        } else if line.starts_with("Infection:") {
            current_army = Army::Infection;
        } else {
            // Parse group line
            let mut weaknesses = Vec::new();
            let mut immunities = Vec::new();
            
            // Extract special properties if present
            if let Some(start) = line.find('(') {
                if let Some(end) = line.find(')') {
                    let special = &line[start + 1..end];
                    for part in special.split("; ") {
                        if let Some(stripped) = part.strip_prefix("weak to ") {
                            weaknesses = stripped.split(", ").map(|s| s.to_string()).collect();
                        } else if let Some(stripped) = part.strip_prefix("immune to ") {
                            immunities = stripped.split(", ").map(|s| s.to_string()).collect();
                        }
                    }
                }
            }
            
            // Parse the main properties
            let parts: Vec<&str> = line.split_whitespace().collect();
            let units: i32 = parts[0].parse().unwrap();
            let hp: i32 = parts[4].parse().unwrap();
            
            // Find "does" keyword to locate attack damage
            let does_idx = parts.iter().position(|&s| s == "does").unwrap();
            let attack_damage: i32 = parts[does_idx + 1].parse().unwrap();
            let attack_type = parts[does_idx + 2].to_string();
            let initiative: i32 = parts[parts.len() - 1].parse().unwrap();
            
            groups.push(Group {
                id,
                army: current_army,
                units,
                hp,
                attack_damage,
                attack_type,
                initiative,
                weaknesses,
                immunities,
            });
            id += 1;
        }
    }
    
    groups
}

fn simulate_combat(mut groups: Vec<Group>, boost: i32) -> Option<(Army, i32)> {
    // Apply boost to immune system
    for group in &mut groups {
        if group.army == Army::ImmuneSystem {
            group.attack_damage += boost;
        }
    }
    
    loop {
        // Remove dead groups
        groups.retain(|g| g.units > 0);
        
        // Check if combat is over
        let immune_alive = groups.iter().any(|g| g.army == Army::ImmuneSystem);
        let infection_alive = groups.iter().any(|g| g.army == Army::Infection);
        
        if !immune_alive {
            let total = groups.iter().map(|g| g.units).sum();
            return Some((Army::Infection, total));
        }
        if !infection_alive {
            let total = groups.iter().map(|g| g.units).sum();
            return Some((Army::ImmuneSystem, total));
        }
        
        // Target selection phase
        let mut targets: Vec<(usize, usize)> = Vec::new();
        let mut targeted: HashSet<usize> = HashSet::new();
        
        // Sort by effective power (descending), then initiative (descending)
        let mut selection_order: Vec<usize> = (0..groups.len()).collect();
        selection_order.sort_by(|&a, &b| {
            groups[b].effective_power().cmp(&groups[a].effective_power())
                .then_with(|| groups[b].initiative.cmp(&groups[a].initiative))
        });
        
        for &attacker_idx in &selection_order {
            let attacker = &groups[attacker_idx];
            if attacker.units <= 0 {
                continue;
            }
            
            let mut best_target: Option<usize> = None;
            let mut best_damage = 0;
            
            for (defender_idx, defender) in groups.iter().enumerate() {
                if defender.army == attacker.army || targeted.contains(&defender_idx) || defender.units <= 0 {
                    continue;
                }
                
                let damage = attacker.damage_to(defender);
                if damage == 0 {
                    continue;
                }
                
                let is_better = if let Some(current_best) = best_target {
                    damage > best_damage || 
                    (damage == best_damage && defender.effective_power() > groups[current_best].effective_power()) ||
                    (damage == best_damage && defender.effective_power() == groups[current_best].effective_power() && defender.initiative > groups[current_best].initiative)
                } else {
                    true
                };
                
                if is_better {
                    best_target = Some(defender_idx);
                    best_damage = damage;
                }
            }
            
            if let Some(target_idx) = best_target {
                targets.push((attacker_idx, target_idx));
                targeted.insert(target_idx);
            }
        }
        
        // Attack phase - sort by initiative (descending)
        targets.sort_by(|&(a, _), &(b, _)| groups[b].initiative.cmp(&groups[a].initiative));
        
        let mut any_killed = false;
        for (attacker_idx, defender_idx) in targets {
            if groups[attacker_idx].units <= 0 {
                continue;
            }
            
            let damage = groups[attacker_idx].damage_to(&groups[defender_idx]);
            let units_killed = damage / groups[defender_idx].hp;
            
            if units_killed > 0 {
                any_killed = true;
            }
            
            groups[defender_idx].units = (groups[defender_idx].units - units_killed).max(0);
        }
        
        // Check for stalemate
        if !any_killed {
            return None;
        }
    }
}

fn part1(input: &str) -> i32 {
    let groups = parse_input(input);
    if let Some((_, units)) = simulate_combat(groups, 0) {
        units
    } else {
        0
    }
}

fn part2(input: &str) -> i32 {
    let groups = parse_input(input);
    
    for boost in 1.. {
        if let Some((winner, units)) = simulate_combat(groups.clone(), boost) {
            if winner == Army::ImmuneSystem {
                return units;
            }
        }
    }
    
    0
}

fn main() {
    let input = read_input(24);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(part1(input), 5216);
    }

    #[test]
    fn test_part2() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(part2(input), 51);
    }
}
