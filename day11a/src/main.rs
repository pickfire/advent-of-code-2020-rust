use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i8,
    y: i8,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let seats: HashSet<_> = input
        .lines()
        .zip(0..)
        .flat_map(|(line, y)| line.chars().zip(0..).filter_map(move |(tile, x)| {
            if tile == 'L' {
                Some(Pos {x: x, y: y})
            } else {
                None
            }
        }))
        .collect();

    let adjacent: Vec<_> = (-1..=1)
        .flat_map(|x| (-1..=1)
            .filter(move |&y| (x, y) != (0, 0))
            .map(move |y| Pos {x: x, y: y})
        )
        .collect();

    let seats: Vec<(_, Vec<_>)> = seats.iter().map(|&pos| (pos, adjacent.iter().filter_map(|&delta| {
        let adjacent_seat = pos + delta;
        if seats.contains(&adjacent_seat) {
            Some(adjacent_seat)
        } else {
            None
        }
    }).collect())).collect();

    let mut filled_seats: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|_| false).collect())
        .collect();

    let mut changed = true;
    let mut to_remove = Vec::with_capacity(seats.len());
    let mut to_add = Vec::with_capacity(seats.len());

    while changed {
        for (pos, adjacent) in seats.iter() {
            let adjacent_filled = adjacent.iter().filter(|&x| filled_seats[x.y as usize][x.x as usize]).count();
            let filled = filled_seats[pos.y as usize][pos.x as usize];
            if adjacent_filled >= 4 && filled {
                to_remove.push(pos);
            } else if adjacent_filled == 0 && !filled {
                to_add.push(pos);
            }
        }
        changed = to_remove.len() > 0 || to_add.len() > 0;
        for &i in &to_remove {
            filled_seats[i.y as usize][i.x as usize] = false;
        }
        for &i in &to_add {
            filled_seats[i.y as usize][i.x as usize] = true;
        }
        to_remove.clear();
        to_add.clear();
    }

    println!("{}", filled_seats.iter().map(|x| x.iter().filter(|&&y| y).count()).sum::<usize>());
}
