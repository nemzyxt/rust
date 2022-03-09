enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction: Direction = Direction::Right;

    match player_direction {
        Direction::Up => println!("Player is moving UP"),
        Direction::Down => println!("Player is moving DOWN"),
        Direction::Left => println!("Player is moving LEFT"),
        Direction::Right => println!("Player is moving RIGHT"),
    }
}