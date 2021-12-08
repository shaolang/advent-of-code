fn main() {
    println!("Hello, world!");
}

struct Position {
    pub horizontal: i16,
    pub depth: i16,
}

impl Position {
    pub fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn update(&mut self, command: &str) {
        let inputs: Vec<&str> = command.split(' ').collect();
        let count: i16 = inputs[1].parse().unwrap();

        match inputs[0] {
            "up" => { self.depth -= count; },
            "down" => { self.depth += count; },
            _ => { self.horizontal += count; },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_forward_by_5() {
        let mut pos = Position::new();
        pos.update("forward 5");

        assert_eq!(pos.horizontal, 5);
        assert_eq!(pos.depth, 0);
    }

    #[test]
    fn move_down_by_5() {
        let mut pos = Position::new();
        pos.update("down 5");

        assert_eq!(pos.horizontal, 0);
        assert_eq!(pos.depth, 5);
    }

    #[test]
    fn move_up_by_3() {
        let mut pos = Position::new();
        pos.horizontal = 10;
        pos.depth = 20;

        pos.update("up 3");

        assert_eq!(pos.horizontal, 10);
        assert_eq!(pos.depth, 17);
    }
}
