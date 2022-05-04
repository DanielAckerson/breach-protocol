use json::{JsonValue, self};

pub type Code = String;

#[derive(Debug)]
pub struct Sequence {
    codes: Vec<Code>,
}

// TODO: rethink terms
#[derive(Debug)]
pub struct Programs {
    sequences: Vec<Sequence>,
}

#[derive(Debug)]
pub struct CodeMatrix {
    matrix: Vec<Vec<Code>>,
}

#[derive(Debug)]
pub struct Buffer {
    code_indices: Vec<Option<usize>>,
}

impl Sequence {
    pub fn from_json(data: &JsonValue) -> Option<Sequence> {
        match data {
            JsonValue::Array(sequence_json) => Some(Sequence {
                codes: sequence_json.iter()
                                    .map_while(JsonValue::as_str)
                                    .map(String::from)
                                    .collect()
            }),
            _ => None,
        }
    }
}

impl Programs {
    pub fn from_json(data: &JsonValue) -> Option<Programs> {
        match data {
            JsonValue::Array(sequences_json) => {
                let mut sequences: Vec<Sequence> = Vec::new();

                for sequence_json in sequences_json.iter() {
                    if let Some(sequence) = Sequence::from_json(&sequence_json) {
                        sequences.push(sequence);
                    }
                }

                None
            },
            JsonValue::Object(board) => Programs::from_json(&board["sequences"]),
            _ => None,
        }
    }

    pub fn to_json() -> Option<JsonValue> {

        None
    }
}

impl CodeMatrix {
    pub fn get(&self, row: usize, col: usize) -> Code {
        self.matrix[row][col].to_string()
    }

    // TODO: could possibly build from only iterators instead of mut rows
    // https://www.reddit.com/r/rust/comments/3rz4gu/comment/cwsq4ex/
    pub fn from_json(data: &JsonValue) -> Option<CodeMatrix> {
        let mut rows: Vec<Vec<Code>> = Vec::new();

        match data {
            JsonValue::Array(code_matrix) => {
                for row in code_matrix.iter() {
                    match row {
                        JsonValue::Array(row_vec) => rows.push(row_vec.iter().map_while(JsonValue::as_str).map(String::from).collect()),
                        _ => return None
                    }
                }
                
                Some(CodeMatrix { matrix: rows })
            },
            JsonValue::Object(board) => CodeMatrix::from_json(&board["code_matrix"]),
            _ => None,
        }
    }

    // TODO: handle Result; None never returned
    // TODO: make more efficient
    pub fn to_json(&self) -> Option<JsonValue> {
        Some(self.matrix.iter().fold(JsonValue::new_array(), |mut rows, row| {
            row.iter().fold(JsonValue::new_array(), |mut cols, col| {
                cols.push(col.clone()).unwrap(); // TODO: handle Result
                cols
            });

            rows.push(row.clone()).unwrap(); // TODO: handle Result
            rows
        }))

        // None
    }
}

impl Buffer {
    pub fn new(capacity: usize) -> Buffer {
        Buffer { code_indices: vec![None; capacity] }
    }

    pub fn contains(&self, coord: (usize, usize)) -> bool {
        for i in 0..self.code_indices.len() {
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
    pub fn coord(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.code_indices.len() {
            return None
        }

        let prev_index = if index > 0 {
            self.code_indices[index - 1].unwrap()
        } else {
            0usize // special case; row = 0 implicitly at puzzle start, i.e. i == 0
        };

        return match self.code_indices[index] {
            Some(code_index) if index % 2 == 0 => Some((prev_index, code_index)),
            Some(code_index) => Some((code_index, prev_index)),
            None => None,
        }
    }

    pub fn code(&self, index: usize, matrix: CodeMatrix) -> Option<Code> {
        match self.coord(index) {
            Some((row, col)) => Some(matrix.get(row, col)),
            None => None,
        }
    }

    // TODO: return as Result, e.g. Err when full?
    pub fn push(&mut self, index: usize) {
        for i in self.code_indices.iter_mut() {
            match i {
                None => {
                    *i = Some(index);
                    return;
                },
                _ => continue,
            }
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        let mut i_iter = self.code_indices.iter_mut().peekable();

        while let Some(code_index) = i_iter.next() {
            match i_iter.peek() {
                Some(None) | None => {
                    let popped = *code_index;
                    *code_index = None;

                    return popped;
                },
                _ => continue,
            }
        }

        None
    }

    pub fn from_json(data: &JsonValue) -> Option<Buffer> {
        match data {
            JsonValue::Array(buffer) => Some(Buffer {
                code_indices: buffer.iter().map(JsonValue::as_usize).collect()
            }),
            JsonValue::Object(board) => Buffer::from_json(&board["buffer"]),
            _ => None,
        }
    }

    // TODO: handle Result; None never returned
    pub fn to_json(&self) -> Option<JsonValue> {
        Some(self.code_indices.iter().fold(JsonValue::new_array(), |mut acc, i| {
            acc.push(*i).unwrap();
            acc
        }))
    }
}

fn main() {
    println!("noop");
}

#[test]
fn buffer_tests() {
    let mut buffer = Buffer::new(5);
    println!("{:?}", buffer);
    assert_eq!(buffer.coord(0), None);
    assert!(!buffer.contains((0, 1))); // shouldn't exist yet

    buffer.push(1); // now it exists
    println!("{:?}", buffer);
    assert_eq!(buffer.coord(0), Some((0, 1)));
    assert!(buffer.contains((0, 1)));
    assert!(!buffer.contains((0, 0)));

    let matrix = CodeMatrix {
        matrix: vec![
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
            ["c9", "b2", "74", "a1", "65"].into_iter().map(String::from).collect(),
        ]
    };

    let code = buffer.code(0, matrix).unwrap();
    println!("code: {:?}", code);
    assert_eq!(code, "b2");

    let a = buffer.pop(); // now it's gone!
    println!("{:?}, {:?}", buffer, a);
    assert_eq!(buffer.coord(0), None);
    assert_eq!(buffer.pop(), None);

    assert_eq!(buffer.coord(10), None);

    for i in 0..10 {
        buffer.push(i);
    }

    assert_eq!(buffer.code_indices.len(), 5);

    buffer.pop();
    buffer.pop();
    println!("buffer {:?}", buffer);
    assert_eq!(buffer.code_indices, vec![Some(0), Some(1), Some(2), None, None]);
}

#[test]
fn json_tests() {
    //TODO: implement sequences to and from json

    let board_json = json::object!{
        buffer: [null, null, null, null, null],
        sequences: [
            ["55", "55", "7a"],
            ["bd", "bd", "bd"],
            ["55", "e9", "55"],
        ],
        code_matrix: [
            ["e9", "e9", "7a", "bd", "55", "55"],
            ["1c", "1c", "1c", "7a", "55", "e9"],
            ["1c", "7a", "7a", "1c", "55", "1c"],
            ["bd", "e9", "55", "7a", "55", "7a"],
            ["55", "55", "55", "7a", "55", "1c"],
            ["bd", "bd", "e9", "1c", "55", "e9"],
        ],
    };

    println!("{}", board_json.dump());

    let buffer = Buffer::from_json(&board_json).unwrap();
    println!("buffer from json {:?}", buffer);
    println!("buffer to json {:?}", buffer.to_json());

    let matrix = CodeMatrix::from_json(&board_json).unwrap();
    println!("matrix from json {:?}", matrix);
    println!("matrix to json {:?}", matrix.to_json());

    let programs = Programs::from_json(&board_json);
    println!("programs from json {:?}", programs);
}
