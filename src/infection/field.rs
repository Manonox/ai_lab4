pub const FIELD_WIDTH: usize = 6;
pub const FIELD_HEIGHT: usize = 6;
pub const FIELD_SIZE: usize = FIELD_WIDTH * FIELD_HEIGHT;


#[derive(Copy, Clone, PartialEq)]
pub struct Field {
    data: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    pub current_player_id: u8,
}



#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move {
    pub source: [i8; 2],
    pub destination: [i8; 2],
}


impl Move {
    pub fn from_string<S: AsRef<str>>(s: S) -> Option<Move> {
        let str = s.as_ref();
        let split: Vec<&str> = str.split(" ").collect();
        if split.len() != 2 { return None }
        let source_str = split[0];
        if source_str.len() != 2 { return None }
        let destination_str = split[1];
        if destination_str.len() != 2 { return None }

        let source = [
            (source_str.bytes().nth(0).unwrap() - b'a') as i8,
            (source_str.bytes().nth(1).unwrap() - b'1') as i8,
        ];

        let destination = [
            (destination_str.bytes().nth(0).unwrap() - b'a') as i8,
            (destination_str.bytes().nth(1).unwrap() - b'1') as i8,
        ];

        let m = Move { source, destination, };
        if !m.is_valid() { return None }

        Some(m)
    }
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(5);
        s.push((b'a' + self.source[0] as u8).into());
        s.push((b'1' + self.source[1] as u8).into());
        s.push(' ');
        s.push((b'a' + self.destination[0] as u8).into());
        s.push((b'1' + self.destination[1] as u8).into());
        s
    }

    const fn distance(&self) -> u8 {
        let x = self.destination[0].abs_diff(self.source[0]);
        let y = self.destination[1].abs_diff(self.source[1]);
        if x > y { x } else { y }
    }

    pub fn is_valid(&self) -> bool {
        self.distance() > 0 && self.distance() <= 2
    }

    pub fn is_cloning(&self) -> bool {
        self.distance() == 1
    }

    pub fn is_moving(&self) -> bool {
        self.distance() == 2
    }
}


impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move({}, {} -> {}, {})", self.source[0], self.source[1], self.destination[0], self.destination[1])
    }
}



impl Field {
    pub fn new() -> Field { Field { data: [[0_u8; FIELD_WIDTH]; FIELD_HEIGHT], current_player_id: 1 } }

    pub fn into_iter(&self) -> impl Iterator<Item = [i8; 2]> {
        (0..FIELD_SIZE).map(|i| {
            [(i % FIELD_WIDTH) as i8, (i / FIELD_WIDTH) as i8]
        })
    }

    pub fn is_valid_position(p: [i8; 2]) -> bool {
        p[0] >= 0 && p[0] < FIELD_WIDTH as i8 && p[1] >= 0 && p[1] < FIELD_HEIGHT as i8
    }

    pub fn get(&self, p: [i8; 2]) -> Option<u8> {
        if !Self::is_valid_position(p) { return None }
        Some(self.data[p[1] as usize][p[0] as usize])
    }

    pub fn set(&mut self, p: [i8; 2], v: u8) -> Result<(), ()> {
        if !Self::is_valid_position(p) { return Err(()) }
        self.data[p[1] as usize][p[0] as usize] = v;
        Ok(())
    }

    pub fn is_valid_move(&self, m: Move) -> bool {
        if !m.is_valid() { return false };
        let Some(cell) = self.get(m.source) else { return false };
        if cell != self.current_player_id { return false }
        let Some(rcell) = self.get(m.destination) else { return false };
        if rcell != 0 { return false }
        true
    }


    fn get_winner(&self) -> Option<u8> {
        if !self.valid_moves().is_empty() { return None }
        let mut p1_count = 0;
        let mut p2_count = 0;
        (0..FIELD_SIZE).for_each(|i| {
            let p = [(i % FIELD_WIDTH) as i8, (i / FIELD_WIDTH) as i8];
            match self.get(p).unwrap() {
                1 => { p1_count += 1},
                2 => { p2_count += 1},
                _ => {},
            }
        });

        if p1_count == p2_count { return Some(0) }
        Some(if p1_count > p2_count {1} else {2})
    }
}


impl Default for Field {
    fn default() -> Self {
        let mut f = Self::new();
        let _ = f.set([0, 0], 1); let _ = f.set([5, 0], 2);
        let _ = f.set([0, 5], 2); let _ = f.set([5, 5], 1);
        f
    }
}


use std::{fmt::Display, collections::HashSet};

use crate::bot::{self, Game};
impl bot::Game<Move> for Field {
    fn valid_moves(&self) -> Vec<Move> {
        let mut similar = HashSet::<Move>::default();
        let mut v = Vec::<Move>::default();
        (0..FIELD_SIZE).for_each(|i| {
            let p = [(i % FIELD_WIDTH) as i8, (i / FIELD_WIDTH) as i8];
            let cell = self.get(p).unwrap();
            if cell != self.current_player_id { return }
            
            (-2..3).for_each(|dx| {
                (-2..3).for_each(|dy| {
                    if dx == 0 && dy == 0 { return }
                    let rp: [i8; 2] = [p[0] + dx, p[1] + dy];
                    let m = Move { source: p, destination: rp };
                    if !self.is_valid_move(m) { return }
                    if m.is_cloning() {
                        if similar.contains(&m) { return }
                        similar.insert(m);
                    }
                    v.push(m);
                });
            });
        });
        v
    }

    fn make_move(&mut self, m: Move) -> Result<Option<u8>, ()> {
        if !self.is_valid_move(m) { return Err(()) }
        
        let player_id = self.get(m.source).unwrap();
        if m.is_moving() {
            self.set(m.source, 0)?;
        }
        self.set(m.destination, player_id)?;

        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 { continue }
                let capturep: [i8; 2] = [m.destination[0] + dx, m.destination[1] + dy];
                let Some(capturecell) = self.get(capturep) else { continue };
                if capturecell == 0 || capturecell == player_id { continue }
                self.set(capturep, player_id)?;
            }
        }

        self.current_player_id = 1 + self.current_player_id % 2;
        
        Ok(self.get_winner())
    }
}


impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                let cell = self.get([x as i8, y as i8]).unwrap();
                write!(f, "{} ", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
