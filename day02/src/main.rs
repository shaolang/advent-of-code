use std::fs;

fn main() {
    let mut pos = Position::new();
    let inputs = load_file_inputs("inputs.txt");

    pos.update_many(&inputs);

    print!("horizontal * depth = {}", pos.horizontal * pos.depth);
}

struct Position {
    pub horizontal: i32,
    pub depth: i32,
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
        let count: i32 = inputs[1].parse().unwrap();

        match inputs[0] {
            "up" => { self.depth -= count; },
            "down" => { self.depth += count; },
            _ => { self.horizontal += count; },
        }
    }

    pub fn update_many(&mut self, commands: &str) {
        for command in commands.lines() {
            self.update(command);
        }
    }
}

fn load_file_inputs(fname: &str) -> String {
    fs::read_to_string(fname).unwrap()
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

    #[test]
    fn multiple_updates() {
        let mut pos = Position::new();
        pos.update_many("forward 5\ndown 4\nup 3\nforward 2");

        assert_eq!(pos.horizontal, 7);
        assert_eq!(pos.depth, 1);
    }

    #[test]
    fn multiple_updates_using_challenge_1_inputs() {
        let cmds = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let mut pos = Position::new();

        pos.update_many(cmds);

        assert_eq!(pos.horizontal, 15);
        assert_eq!(pos.depth, 10);
    }
}
