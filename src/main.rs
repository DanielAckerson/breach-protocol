use json::{JsonValue, self};

pub type Code = String;
pub type Sequence = Vec<Code>;

pub struct CodeMatrix {
    matrix: Vec<Vec<Code>>,
    // TODO: impl: fn get(row, col); 
}

#[derive(Debug)]
pub struct Buffer {
    pub item_indices: Vec<Option<usize>>,
}

impl CodeMatrix {
    pub fn get(&self, row: usize, col: usize) -> Code {
        self.matrix[row][col].to_string()
    }
}

impl Buffer {
    pub fn new(capacity: usize) -> Buffer {
        Buffer { item_indices: vec![None; capacity] }
    }

    pub fn contains(&self, coord: (usize, usize)) -> bool {
        for i in 0..self.item_indices.len() {
            match self.coord(i) {
                Some(my_coord) if my_coord == coord => return true,
                None => return false,
                _ => continue,
            }
        }

        false
    }

    //    buffer = [col, row, col, row, ...]
    // index % 2 = [  0,   1,   0,   1, ...]
    fn coord(&self, index: usize) -> Option<(usize, usize)> {
        let prev_index = if index > 0 {
            self.item_indices[index - 1].unwrap()
        } else {
            0usize // special case; row = 0 implicitly at puzzle start, i.e. i == 0
        };

        return match self.item_indices[index] {
            Some(item_index) if index % 2 == 0 => Some((prev_index, item_index)),
            Some(item_index) => Some((item_index, prev_index)),
            None => None,
        }
    }

    fn code(&self, index: usize, matrix: CodeMatrix) -> Option<Code> {
        match self.coord(index) {
            Some((row, col)) => Some(matrix.get(row, col)),
            None => None,
        }
    }

    fn push(&mut self, index: usize) {
        for i in 0..self.item_indices.len() {
            match self.item_indices[i] {
                None => {
                    self.item_indices[i] = Some(index);
                    return;
                },
                _ => continue,
            }
        }
    }

    fn pop(&mut self) -> Option<usize> {
        for i in 0..self.item_indices.len() {
            match self.item_indices[i] {
                // None => self.item_indices[i] = Some(index),
                None => {
                    if i == 0 {
                        return None
                    }

                    let index = self.item_indices[i - 1];
                    self.item_indices[i - 1] = None;
                    return index;
                },
                _ => continue,
            }
        }

        None
    }
}

fn main() {
    let matrix = CodeMatrix {
        matrix: vec![
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
        ]
    };

    let mut buffer = Buffer::new(5);
    println!("{:?}", buffer);
    assert_eq!(buffer.coord(0), None);
    assert!(!buffer.contains((0, 1))); // shouldn't exist yet

    buffer.push(1); // now it exists
    println!("{:?}", buffer);
    assert_eq!(buffer.coord(0), Some((0, 1)));
    assert!(buffer.contains((0, 1)));
    assert!(!buffer.contains((0, 0)));

    if let Some(code) = buffer.code(0, matrix) {
        println!("code: {:?}", code);
        assert_eq!(code, "b2");
    }

    let a = buffer.pop(); // now it's gone!
    println!("{:?}, {:?}", buffer, a);
    assert_eq!(buffer.coord(0), None);
    assert_eq!(buffer.pop(), None);

    for i in 0..10 {
        buffer.push(i);
    }

    assert_eq!(buffer.item_indices.len(), 5);

    let matrix_json = json::object!{
        buffer: [ null, null, null, null, null, ],
        sequences: [
            [ "55", "55", "7a", ],
            [ "bd", "bd", "bd", ],
            [ "55", "e9", "55", ],
        ],
        code_matrix: [
            [ "e9", "e9", "7a", "bd", "55", "55", ],
            [ "1c", "1c", "1c", "7a", "55", "e9", ],
            [ "1c", "7a", "7a", "1c", "55", "1c", ],
            [ "bd", "e9", "55", "7a", "55", "7a", ],
            [ "55", "55", "55", "7a", "55", "1c", ],
            [ "bd", "bd", "e9", "1c", "55", "e9", ],
        ],
    };

    println!("{}", matrix_json.dump());
    println!("Success!");
}
