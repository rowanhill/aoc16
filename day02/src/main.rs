trait Keypad {
    fn new() -> Self;
    fn reset(&mut self);
    fn get_key(&self) -> String;
    fn left(&mut self);
    fn right(&mut self);
    fn up(&mut self);
    fn down(&mut self);
}

struct SimpleKeypad {
    num: u8
}

impl Keypad for SimpleKeypad {
    fn new() -> SimpleKeypad {
        SimpleKeypad { num: 5 }
    }

    fn reset(&mut self) {
        self.num = 5;
    }

    fn get_key(&self) -> String {
        self.num.to_string()
    }

    fn left(&mut self) {
        self.num -= if (self.num - 1) % 3 == 0 { 0 } else { 1 };
    }

    fn right(&mut self) {
        self.num += if (self.num - 1) % 3 == 2 { 0 } else { 1 };
    }

    fn up(&mut self) {
        self.num -= if (self.num - 1) / 3 == 0 { 0 } else { 3 };
    }

    fn down(&mut self) {
        self.num += if (self.num - 1) / 3 == 2 { 0 } else { 3 };
    }
}

static COMPLEX_KEYS: [[&'static str; 5]; 5] = [
    ["n/a", "n/a", "1", "n/a", "n/a"],
    ["n/a", "2", "3", "4", "n/a"],
    ["5", "6", "7", "8", "9"],
    ["n/a", "A", "B", "C", "n/a"],
    ["n/a", "n/a", "D", "n/a", "n/a"]
];

struct ComplexKeypad {
    row: i8,
    col: i8
}

impl Keypad for ComplexKeypad {
    fn new() -> Self {
        ComplexKeypad {
            row: 2,
            col: 0
        }
    }

    fn reset(&mut self) {
        self.row = 2;
        self.col = 0;
    }

    fn get_key(&self) -> String {
        Self::get_key(self.row, self.col).to_string()
    }

    fn left(&mut self) {
        let col = std::cmp::max(0, self.col - 1);
        let new_pos = Self::get_key(self.row, col);
        if new_pos != "n/a" {
            self.col = col;
        }
    }

    fn right(&mut self) {
        let col = std::cmp::min(4, self.col + 1);
        let new_pos = Self::get_key(self.row, col);
        if new_pos != "n/a" {
            self.col = col;
        }
    }

    fn up(&mut self) {
        let row = std::cmp::max(0, self.row - 1);
        let new_pos = Self::get_key(row, self.col);
        if new_pos != "n/a" {
            self.row = row;
        }
    }

    fn down(&mut self) {
        let row = std::cmp::min(4, self.row + 1);
        let new_pos = Self::get_key(row, self.col);
        if new_pos != "n/a" {
            self.row = row;
        }
    }
}

impl ComplexKeypad {
    fn get_key(row: i8, col: i8) -> &'static str {
        COMPLEX_KEYS[row as usize][col as usize]
    }
}

fn main() {
    let test_code:&str = r"ULL
RRDDD
LURDL
UUUUD
";
    let code = r"RDLULDLDDRLLLRLRULDRLDDRRRRURLRLDLULDLDLDRULDDLLDRDRUDLLDDRDULLLULLDULRRLDURULDRUULLLUUDURURRDDLDLDRRDDLRURLLDRRRDULDRULURURURURLLRRLUDULDRULLDURRRLLDURDRRUUURDRLLDRURULRUDULRRRRRDLRLLDRRRDLDUUDDDUDLDRUURRLLUDUDDRRLRRDRUUDUUULDUUDLRDLDLLDLLLLRRURDLDUURRLLDLDLLRLLRULDDRLDLUDLDDLRDRRDLULRLLLRUDDURLDLLULRDUUDRRLDUDUDLUURDURRDDLLDRRRLUDULDULDDLLULDDDRRLLDURURURUUURRURRUUDUUURULDLRULRURDLDRDDULDDULLURDDUDDRDRRULRUURRDDRLLUURDRDDRUDLUUDURRRLLRR
RDRRLURDDDDLDUDLDRURRLDLLLDDLURLLRULLULUUURLDURURULDLURRLRULDDUULULLLRLLRDRRUUDLUUDDUDDDRDURLUDDRULRULDDDLULRDDURRUURLRRLRULLURRDURRRURLDULULURULRRLRLUURRRUDDLURRDDUUDRDLLDRLRURUDLDLLLLDLRURDLLRDDUDDLDLDRRDLRDRDLRRRRUDUUDDRDLULUDLUURLDUDRRRRRLUUUDRRDLULLRRLRLDDDLLDLLRDDUUUUDDULUDDDUULDDUUDURRDLURLLRUUUUDUDRLDDDURDRLDRLRDRULRRDDDRDRRRLRDULUUULDLDDDUURRURLDLDLLDLUDDLDLRUDRLRLDURUDDURLDRDDLLDDLDRURRULLURULUUUUDLRLUUUDLDRUDURLRULLRLLUUULURLLLDULLUDLLRULRRLURRRRLRDRRLLULLLDURDLLDLUDLDUDURLURDLUURRRLRLLDRLDLDRLRUUUDRLRUDUUUR
LLLLULRDUUDUUDRDUUURDLLRRLUDDDRLDUUDDURLDUDULDRRRDDLLLRDDUDDLLLRRLURDULRUUDDRRDLRLRUUULDDULDUUUDDLLDDDDDURLDRLDDDDRRDURRDRRRUUDUUDRLRRRUURUDURLRLDURDDDUDDUDDDUUDRUDULDDRDLULRURDUUDLRRDDRRDLRDLRDLULRLLRLRLDLRULDDDDRLDUURLUUDLLRRLLLUUULURUUDULRRRULURUURLDLLRURUUDUDLLUDLDRLLRRUUDDRLUDUDRDDRRDDDURDRUDLLDLUUDRURDLLULLLLUDLRRRUULLRRDDUDDDUDDRDRRULURRUUDLUDLDRLLLLDLUULLULLDDUDLULRDRLDRDLUDUDRRRRLRDLLLDURLULUDDRURRDRUDLLDRURRUUDDDRDUUULDURRULDLLDLDLRDUDURRRRDLDRRLUDURLUDRRLUDDLLDUULLDURRLRDRLURURLUUURRLUDRRLLULUULUDRUDRDLUL
LRUULRRUDUDDLRRDURRUURDURURLULRDUUDUDLDRRULURUDURURDRLDDLRUURLLRDLURRULRRRUDULRRULDLUULDULLULLDUDLLUUULDLRDRRLUURURLLUUUDDLLURDUDURULRDLDUULDDRULLUUUURDDRUURDDDRUUUDRUULDLLULDLURLRRLRULRLDLDURLRLDLRRRUURLUUDULLLRRURRRLRULLRLUUDULDULRDDRDRRURDDRRLULRDURDDDDDLLRRDLLUUURUULUDLLDDULDUDUUDDRURDDURDDRLURUDRDRRULLLURLUULRLUDUDDUUULDRRRRDLRLDLLDRRDUDUUURLRURDDDRURRUDRUURUUDLRDDDLUDLRUURULRRLDDULRULDRLRLLDRLURRUUDRRRLRDDRLDDLLURLLUDL
ULURLRDLRUDLLDUDDRUUULULUDDDDDRRDRULUDRRUDLRRRLUDLRUULRDDRRLRUDLUDULRULLUURLLRLLLLDRDUURDUUULLRULUUUDRDRDRUULURDULDLRRULUURURDULULDRRURDLRUDLULULULUDLLUURULDLLLRDUDDRRLULUDDRLLLRURDDLDLRLLLRDLDRRUUULRLRDDDDRUDRUULDDRRULLDRRLDDRRUDRLLDUDRRUDDRDLRUDDRDDDRLLRDUULRDRLDUDRLDDLLDDDUUDDRULLDLLDRDRRUDDUUURLLUURDLULUDRUUUDURURLRRDULLDRDDRLRDULRDRURRUDLDDRRRLUDRLRRRRLLDDLLRLDUDUDDRRRUULDRURDLLDLUULDLDLDUUDDULUDUDRRDRLDRDURDUULDURDRRDRRLLRLDLU
";

    print_answer(test_code, &mut SimpleKeypad::new());
    print_answer(code, &mut SimpleKeypad::new());
    print_answer(test_code, &mut ComplexKeypad::new());
    print_answer(code, &mut ComplexKeypad::new());
}

fn print_answer<T>(code: &str, keypad: &mut T) where T: Keypad {
    let mut answer = String::new();

    for char in code.chars() {
        // print!("  {:?}: {} ->", char, keypad.get_key());
        match char {
            'R' => keypad.right(),
            'L' => keypad.left(),
            'U' => keypad.up(),
            'D' => keypad.down(),
            _ => {
                // print!(" <answer> !! RESET TO: ");
                answer += &keypad.get_key();
                keypad.reset();
            }
        }
        // println!(" {}", keypad.get_key());
    }

    println!("{}", answer);
}
