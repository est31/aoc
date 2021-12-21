use std::collections::HashMap;

const INPUT :&str = include_str!("input");

#[cfg(test)]
mod test;

fn main() {
	let sps = obtain_starting_positions(INPUT);
	println!("Product: {}", play_game(sps));
	println!("Larger universe count: {}", play_dirac_game(sps));
}

fn obtain_starting_positions(input :&str) -> (u16, u16) {
	let mut l = input.lines()
		.map(|l| l.trim())
		.filter(|l| !l.is_empty());
	let p = l.next().unwrap().as_bytes()[28];
	let q = l.next().unwrap().as_bytes()[28];
	((p - b'0') as _, (q - b'0') as _)
}

fn play_move(dice_amount :u16, pos :&mut u16) {
	*pos = ((dice_amount + *pos - 1) % 10) + 1;
}

fn play_game(positions :(u16, u16)) -> u32 {
	let (die_rolls, (pl1_score, pl2_score)) = find_end_state(positions);
	let loser_score = pl1_score.min(pl2_score) as u32;
	loser_score * (die_rolls as u32)
}

fn find_end_state(positions :(u16, u16)) -> (u16, (u16, u16)) {
	let mut pl1_score = 0;
	let mut pl2_score = 0;

	let mut pl1_pos = positions.0;
	let mut pl2_pos = positions.1;

	let mut die_rolls = 0;
	while pl2_score < 1000 {
		let mut roll = || {
			let die_val = ((die_rolls % 100) + 1) as u16;
			die_rolls += 1;
			die_val
		};
		let die_val = roll() + roll() + roll();
		play_move(die_val, &mut pl1_pos);
		pl1_score += pl1_pos;

		if pl1_score >= 1000 {
			break;
		}

		let die_val = roll() + roll() + roll();
		play_move(die_val, &mut pl2_pos);
		pl2_score += pl2_pos;
	}
	(die_rolls, (pl1_score, pl2_score))
}

fn play_dirac_game(positions :(u16, u16)) -> u128 {
	let (u1, u2) = dirac_universes(positions);
	u1.max(u2)
}

fn die_rolls() -> Vec<u16> {
	let mut rolls = Vec::new();
	for r1 in 1..=3 {
		for r2 in 1..=3 {
			for r3 in 1..=3 {
				rolls.push(r1 + r2 + r3);
			}
		}
	}
	rolls
}

fn dirac_universes(positions :(u16, u16)) -> (u128, u128) {
	let mut universe_map = HashMap::new();
	universe_map.insert((positions, (0u16, 0u16, true)), 1u128);
	let mut player_1_won = 0u128;
	let mut player_2_won = 0u128;
	let die_rolls = die_rolls();
	while !universe_map.is_empty() {
		let mut new_universe_map = HashMap::new();
		for ((p, (sc1, sc2, pl1_moving)), cnt) in universe_map.iter() {
			let (pl_pos, sc, won) = if *pl1_moving {
				(p.0, sc1, &mut player_1_won)
			} else {
				(p.1, sc2, &mut player_2_won)
			};
			for roll in &die_rolls {
				let mut pl_pos = pl_pos;
				play_move(*roll, &mut pl_pos);
				let sc = sc + pl_pos;
				if sc >= 21 {
					*won += cnt;
					continue;
				}
				let key = if *pl1_moving {
					((pl_pos, p.1), (sc, *sc2, !pl1_moving))
				} else {
					((p.0, pl_pos), (*sc1, sc, !pl1_moving))
				};
				*new_universe_map.entry(key).or_insert(0) += cnt;
			}
		}
		universe_map = new_universe_map;
	}
	(player_1_won, player_2_won)
}
