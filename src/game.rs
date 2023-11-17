use crate::entity::{find, find_mut, Player};

/// play_game determines what kind of game to play, 0 or 1 will be a straight
/// round robin, anything more will be round robin with multiple rounds
pub fn play_game(players: &mut [impl Player], rounds: i32) {
    if rounds <= 1 {
        play_round_robin(players)
    } else {
        play_tournament(players, rounds)
    }
}

fn play_tournament(players: &mut [impl Player], rounds: i32) {
    for _ in 0..rounds {
        play_round_robin(players);
    }
}

fn play_round_robin(players: &mut [impl Player]) {
    let rounds = set_rounds(players);
    for (p1, p2) in &rounds {
        let (c1, c2);
        {
            let player_one = find(p1, players).unwrap();
            let player_two = find(p2, players).unwrap();
            c1 = player_one.choose(player_two.tag());
            c2 = player_two.choose(player_one.tag());
        }
        find_mut(p1, players).unwrap().add_memory(p2, (c1, c2));
        find_mut(p2, players).unwrap().add_memory(p1, (c2, c1));
    }
}

// set_rounds takes a Vec of players and produces the order they will compete
// against each other, it is called at the beginning of round_robin
fn set_rounds(players: &[impl Player]) -> Vec<(String, String)> {
    let opponents = players.to_owned();
    let round_list = players
        .iter()
        .enumerate()
        .flat_map(|(i, player)| {
            opponents
                .iter()
                .skip(i + 1)
                .map(move |opponent| (player.tag().to_string(), opponent.tag().to_string()))
        })
        .collect::<Vec<_>>();
    round_list
}
