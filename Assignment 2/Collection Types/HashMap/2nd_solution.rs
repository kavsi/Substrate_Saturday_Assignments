// Basic Operations problem solution
// 1st method:
use std::collections::HashMap;
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2: HashMap<_,_> = teams.into_iter().collect();  // Implemented team_map2 

    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

// 2nd method:
use std::{collections::HashMap};
fn main() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2 = HashMap::from(teams);   // Implemented team_map2
    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}