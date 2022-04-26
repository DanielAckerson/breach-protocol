
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

struct Board {
    pub buffer: Buffer,
    pub sequences: Vec<Sequence>,
    pub code_matrix: CodeMatrix,
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

    println!("Success!");
}
