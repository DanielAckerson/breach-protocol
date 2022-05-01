use json::{JsonValue, self};

pub type Code = String;
pub type Sequence = Vec<Code>;

#[derive(Debug)]
pub struct CodeMatrix {
    matrix: Vec<Vec<Code>>,
}

#[derive(Debug)]
pub struct Buffer {
    pub code_indices: Vec<Option<usize>>,
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

    pub fn to_json(&self) -> Option<JsonValue> {

        None
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
    fn coord(&self, index: usize) -> Option<(usize, usize)> {
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

    fn code(&self, index: usize, matrix: CodeMatrix) -> Option<Code> {
        match self.coord(index) {
            Some((row, col)) => Some(matrix.get(row, col)),
            None => None,
        }
    }

    // TODO: return as Result, e.g. Err when full?
    fn push(&mut self, index: usize) {
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

    fn pop(&mut self) -> Option<usize> {
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

    pub fn to_json(&self) -> Option<JsonValue> {
        Some(self.code_indices.iter().fold(JsonValue::new_array(), |mut acc, i| {
            acc.push(*i).unwrap();
            acc
        }))
    }
}

fn main() {
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

    if let Some(code) = buffer.code(0, matrix) {
        println!("code: {:?}", code);
        assert_eq!(code, "b2");
    }

    let a = buffer.pop(); // now it's gone!
    println!("{:?}, {:?}", buffer, a);
    assert_eq!(buffer.coord(0), None);
    assert_eq!(buffer.pop(), None);

    assert_eq!(buffer.coord(10), None);

    for i in 0..10 {
        buffer.push(i);
    }

    assert_eq!(buffer.code_indices.len(), 5);

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
    println!("buffer from json {:?}", Buffer::from_json(&board_json));

    buffer.pop();
    buffer.pop();
    println!("buffer {:?}", buffer);
    assert_eq!(buffer.code_indices, vec![Some(0), Some(1), Some(2), None, None]);

    println!("buffer to json {:?}", Buffer::to_json(&buffer));

    let matrix = CodeMatrix::from_json(&board_json);
    println!("matrix from json {:?}", matrix);

    println!("Success!");
}
