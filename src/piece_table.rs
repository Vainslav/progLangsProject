struct Table{
    buffer: String,
    start_index: usize,
    length: usize,
}

struct PieceTable{
    original: String,
    new: String,
    table: Vec<Table>,
    length: i32,
}

impl PieceTable{
    pub fn insert(&mut self, idx: i32, value: String){
        assert!(self.length >= idx);

        let i = self.new.len();
        self.new.push_str(&value);
        self.table.push(Table{
            buffer: String::from("new"),
            start_index: i,
            length: value.len(),
        })
    }
}