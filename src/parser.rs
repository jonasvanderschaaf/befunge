type SymResult = Result<(), ()>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Parser {
    grid: Vec<Vec<char>>,
    direction: Direction,
    stack: Vec<i64>,
    is_ascii_mode: bool,
    position: (usize, usize),
    is_halted: bool,
}

const DEFAULT_PARSER: Parser = Parser {
    grid: Vec::new(),
    direction: Direction::Right,
    stack: Vec::new(),
    is_ascii_mode: false,
    position: (0, 0),
    is_halted: false,
};

impl Parser {
    pub fn new(str_grid: Box<[&str]>) -> Parser {
        /* Parse the given grid into a parser. */
        let mut parser_grid: Vec<Vec<char>> = Vec::new();

        for line in str_grid.iter() {
            let chars: Vec<char> = line.clone().chars().collect();
            parser_grid.push(chars);
        }

        for line in str_grid.iter() {
            let chars: Vec<char> = line.clone().chars().collect();
            for c in chars {
                print!("{}", c);
            }
            print!("\n");
        }

        Parser {
            grid: parser_grid,
            ..DEFAULT_PARSER
        }
    }

    fn walk(&mut self) -> SymResult {
        match self.direction {
            Direction::Up => {
                if !(self.position.1 > 0) {
                    Err(())
                } else {
                    self.position.1 -= 1;
                    Ok(())
                }
            }
            Direction::Down => {
                if !(self.position.1 < usize::MAX) {
                    Err(())
                } else {
                    self.position.1 += 1;
                    Ok(())
                }
            }
            Direction::Left => {
                if !(self.position.0 > 0) {
                    Err(())
                } else {
                    self.position.0 -= 1;
                    Ok(())
                }
            }
            Direction::Right => {
                if !(self.position.0 < usize::MAX) {
                    Err(())
                } else {
                    self.position.0 += 1;
                    Ok(())
                }
            }
        }
    }

    fn get_instruction(&mut self) -> char {
        let (x, y) = self.position;
        let line_opt = self.grid.get(y);
        match line_opt {
            Some(line) => {
                let instruction_opt = line.get(x);

                instruction_opt.unwrap_or(&' ').clone()
            }
            None => ' ',
        }
    }

    fn execute_instruction(&mut self) -> SymResult {
        let instruction = self.get_instruction();
        println!("{}", instruction);

        match instruction {
            '>' => {
                self.direction = Direction::Right;
                Ok(())
            }
            '<' => {
                self.direction = Direction::Left;
                Ok(())
            }
            '^' => {
                self.direction = Direction::Up;
                Ok(())
            }
            'v' => {
                self.direction = Direction::Down;
                Ok(())
            }
            '@' => {
                self.is_halted = true;
                Ok(())
            }
            _ => Err(()),
        }
    }

    fn execute_step(&mut self) -> SymResult {
        let result: SymResult = self.execute_instruction();

        if result.is_err() {
            return result;
        }

        self.walk()
    }

    pub fn run(&mut self) -> SymResult {
        loop {
            let result = self.execute_step();
        }
    }
}
