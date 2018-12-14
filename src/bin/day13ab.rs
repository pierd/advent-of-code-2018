use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::repeat;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coords(usize, usize);

impl Coords {
    fn in_direction(&self, direction: Direction) -> Self {
        use self::Direction::*;
        let &Coords(row, column) = self;
        match direction {
            Up => Coords(row - 1, column),
            Down => Coords(row + 1, column),
            Left => Coords(row, column - 1),
            Right => Coords(row, column + 1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn after_action(self, action: Action) -> Self {
        use self::Direction::*;
        match action {
            Action::Straight => self,
            Action::Left => match self {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            },
            Action::Right => match self {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Field {
    Cart(Direction),
    Cross,
    Slash,
    Backslash,
    Horizontal,
    Vertical,
}

impl Field {
    fn from_char(c: char) -> Option<Self> {
        use self::Direction::*;
        use self::Field::*;
        match c {
            '^' => Some(Cart(Up)),
            'v' => Some(Cart(Down)),
            '<' => Some(Cart(Left)),
            '>' => Some(Cart(Right)),
            '+' => Some(Cross),
            '/' => Some(Slash),
            '\\' => Some(Backslash),
            '-' => Some(Horizontal),
            '|' => Some(Vertical),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MapField {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl MapField {
    fn with_up(mut self) -> Self {
        self.up = true;
        self
    }

    fn with_down(mut self) -> Self {
        self.down = true;
        self
    }

    fn with_left(mut self) -> Self {
        self.left = true;
        self
    }

    fn with_right(mut self) -> Self {
        self.right = true;
        self
    }

    fn cross() -> Self {
        MapField {
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }

    fn vertical() -> Self {
        Self::default().with_up().with_down()
    }

    fn horizontal() -> Self {
        Self::default().with_left().with_right()
    }

    fn from_position_and_hashmap(position: &Coords, map: &HashMap<Coords, Field>) -> Option<Self> {
        use self::Direction::*;
        match map.get(position) {
            None => None,
            Some(Field::Cart(Up)) | Some(Field::Cart(Down)) => Some(MapField::vertical()),
            Some(Field::Cart(Left)) | Some(Field::Cart(Right)) => Some(MapField::horizontal()),
            Some(Field::Cross) => Some(MapField::cross()),
            Some(Field::Slash) => {
                if let Some(below) =
                    Self::from_position_and_hashmap(&position.in_direction(Down), map)
                {
                    if below.up {
                        Some(MapField::default().with_down().with_right())
                    } else {
                        Some(MapField::default().with_up().with_left())
                    }
                } else {
                    Some(MapField::default().with_up().with_left())
                }
            }
            Some(Field::Backslash) => {
                if let Some(below) =
                    Self::from_position_and_hashmap(&position.in_direction(Down), map)
                {
                    if below.up {
                        Some(MapField::default().with_down().with_left())
                    } else {
                        Some(MapField::default().with_up().with_right())
                    }
                } else {
                    Some(MapField::default().with_up().with_right())
                }
            }
            Some(Field::Horizontal) => Some(MapField::horizontal()),
            Some(Field::Vertical) => Some(MapField::vertical()),
        }
    }

    fn to_char(&self) -> char {
        if self.is_cross() {
            '+'
        } else if self.up && self.down {
            '|'
        } else if self.left && self.right {
            '-'
        } else if (self.up && self.left) || (self.down && self.right) {
            '/'
        } else {
            '\\'
        }
    }

    fn is_cross(&self) -> bool {
        self.up && self.down && self.left && self.right
    }

    fn has_direction(&self, direction: Direction) -> bool {
        use self::Direction::*;
        match direction {
            Up => self.up,
            Down => self.down,
            Left => self.left,
            Right => self.right,
        }
    }

    fn direction_after(&self, direction: Direction) -> Direction {
        use self::Direction::*;
        assert!(!self.is_cross());
        for d in [Up, Down, Left, Right].iter() {
            if *d != direction.opposite() && self.has_direction(*d) {
                return *d;
            }
        }
        panic!("Direction not found!");
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Action {
    Left,
    Straight,
    Right,
}

impl Default for Action {
    fn default() -> Self {
        Action::Left
    }
}

impl Action {
    fn next(self) -> Self {
        use self::Action::*;
        match self {
            Left => Straight,
            Straight => Right,
            Right => Left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Cart {
    position: Coords,
    facing: Direction,
    next_action: Action,
    crashed: bool,
}

impl Cart {
    fn from_position_and_field(position: &Coords, field: &Field) -> Option<Self> {
        if let Field::Cart(facing) = field {
            Some(Cart {
                position: *position,
                facing: *facing,
                next_action: Action::default(),
                crashed: false,
            })
        } else {
            None
        }
    }

    fn to_char(&self) -> char {
        use self::Direction::*;
        if self.crashed {
            'X'
        } else {
            match self.facing {
                Up => '^',
                Down => 'v',
                Left => '<',
                Right => '>',
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Map {
    fields: HashMap<Coords, MapField>,
    carts: Vec<Cart>,
}

impl Map {
    fn from_hashmap(map: &HashMap<Coords, Field>) -> Self {
        let mut fields = HashMap::with_capacity(map.len());
        let mut carts = Vec::new();
        for (coords, field) in map {
            fields.insert(
                *coords,
                MapField::from_position_and_hashmap(coords, map).unwrap(),
            );
            match Cart::from_position_and_field(coords, field) {
                Some(cart) => {
                    carts.push(cart);
                }
                None => {}
            }
        }
        Map { fields, carts }
    }

    fn print_out(&self) {
        let min_row = self.fields.keys().map(|Coords(row, _)| *row).min().unwrap();
        let max_row = self.fields.keys().map(|Coords(row, _)| *row).max().unwrap();
        let min_column = self
            .fields
            .keys()
            .map(|Coords(_, column)| *column)
            .min()
            .unwrap();
        let max_column = self
            .fields
            .keys()
            .map(|Coords(_, column)| *column)
            .max()
            .unwrap();

        let carts: HashMap<Coords, &Cart> = self
            .carts
            .iter()
            .map(|cart| (cart.position, cart))
            .collect();

        for row in min_row..=max_row {
            for column in min_column..=max_column {
                let coords = Coords(row, column);
                print!(
                    "{}",
                    match (carts.get(&coords), self.fields.get(&coords)) {
                        (Some(cart), _) => cart.to_char(),
                        (_, None) => ' ',
                        (_, Some(field)) => field.to_char(),
                    }
                );
            }
            println!("");
        }
        println!("{:?}", self.carts);
    }

    fn advance_tick(&mut self) -> (usize, Vec<Coords>) {
        let mut carts_left = 0;
        let mut collisions = Vec::new();
        self.carts.sort();
        let mut busy_coords: HashMap<Coords, usize> = self
            .carts
            .iter()
            .enumerate()
            .filter(|(_, cart)| !cart.crashed)
            .map(|(idx, cart)| (cart.position, idx))
            .collect();

        for i in 0..self.carts.len() {
            if !self.carts[i].crashed {
                busy_coords.remove(&self.carts[i].position);
                let new_position = self.carts[i].position.in_direction(self.carts[i].facing);
                self.carts[i].position = new_position;
                let other_idx = busy_coords.remove(&new_position);
                match other_idx {
                    Some(other) => {
                        collisions.push(new_position);
                        self.carts[i].crashed = true;
                        self.carts[other].crashed = true;
                    }
                    None => {
                        let cart = self.carts.get_mut(i).unwrap();
                        busy_coords.insert(new_position, i);
                        let new_field = self.fields.get(&new_position).unwrap();
                        if new_field.is_cross() {
                            cart.facing = cart.facing.after_action(cart.next_action);
                            cart.next_action = cart.next_action.next();
                        } else {
                            cart.facing = new_field.direction_after(cart.facing);
                        }
                    }
                }
                if !self.carts[i].crashed {
                    carts_left += 1;
                }
            }
        }
        (carts_left, collisions)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input_map: HashMap<Coords, Field> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            repeat(row)
                .zip(0..(line.len()))
                .zip(line.chars().map(Field::from_char))
        })
        .filter(|(_, field)| field.is_some())
        .map(|((row, column), field)| (Coords(row, column), field.unwrap()))
        .collect();

    let mut map = Map::from_hashmap(&input_map);

    loop {
        // map.print_out();
        let (carts_left, collisions) = map.advance_tick();
        if !collisions.is_empty() {
            println!("collision: {},{}", collisions[0].1, collisions[0].0);
        }
        if carts_left < 2 {
            map.print_out();
            println!(
                "{:?}",
                map.carts
                    .iter()
                    .filter(|cart| !cart.crashed)
                    .collect::<Vec<_>>()
            );
            break;
        }
    }

    Ok(())
}
