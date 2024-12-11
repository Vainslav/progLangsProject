#[derive(PartialEq, Eq, Clone, Copy)]
enum Buffer{
    Read,
    Add
}

#[derive(Clone, Copy)]
struct Piece{
    buffer: Buffer,
    offset: usize,
    length: usize,
}

#[derive(Clone)]
pub struct PieceTable{
    read: Vec<char>,
    add: Vec<char>,
    pieces: Vec<Piece>,
    length: usize,
    num_lines: usize,
}

impl PieceTable{

    pub fn new() -> PieceTable {
        PieceTable {
            read: vec![],
            add: vec![],
            pieces: vec![],
            length: 0,
            num_lines: 0,
        }
    }

    pub fn assign_buffer(&mut self, str: String){
        if str.len() > 0{
            self.pieces.push(Piece{
                buffer: Buffer::Read,
                offset: 0,
                length: str.chars().count()
            })
        }

        self.length += str.chars().count();
        for ch in str.chars(){
            self.read.push(ch);
        }
        self.num_lines += str.split('\n').count();
    }


    pub fn get_piece_by_index(&self, idx: usize) -> Result<Vec<usize>, i8>{
        let mut offset: usize = idx;
        for i in 0..self.pieces.len(){
            if offset <= self.pieces[i].length{
                return Ok(vec![i, offset]);
            }
            offset -= self.pieces[i].length;
        }
        Err(1)
    }

    pub fn insert(&mut self, idx: usize, text: String){
        assert!(self.length >= idx);

        let piece_and_offset: Vec<usize> = self.get_piece_by_index(idx).expect("I hape it won't happen");
        let cur_piece = &mut self.pieces[piece_and_offset[0]];
        self.length += text.chars().count();
        self.num_lines += text.split("\n").count();
        let add_len = self.add.len();
        for ch in text.chars(){
            self.add.push(ch);
        }

        if cur_piece.buffer == Buffer::Add{
            if cur_piece.length + cur_piece.offset == self.add.len() && cur_piece.length == piece_and_offset[1]{
                cur_piece.length += text.len();
                return
            }
        }

        let pieces_vector: Vec<Piece> = vec![
            Piece{
                buffer: cur_piece.buffer,
                offset: cur_piece.offset,
                length: piece_and_offset[1]
            },
            Piece{
                buffer: Buffer::Add,
                offset: add_len,
                length: text.chars().count()
            },
            Piece{
                buffer: cur_piece.buffer,
                offset: cur_piece.offset + piece_and_offset[1],
                length: cur_piece.length - piece_and_offset[1]
            }
        ].into_iter().filter(|piece| piece.length > 0).collect();

        
        let mut pieces = self.pieces[..piece_and_offset[0]].to_vec();
        pieces.extend(pieces_vector.iter());
        pieces.extend(self.pieces[piece_and_offset[0]+1..].iter());
        self.pieces = pieces;
    }

    pub fn remove(&mut self, idx: usize, length: usize){
        if length <= 0{
            panic!();
        };

        let start_piece_and_offset: Vec<usize> = self.get_piece_by_index(idx).unwrap();
        let stop_piece_and_offset: Vec<usize> = match self.get_piece_by_index(idx + length) {
            Ok(value) => value,
            Err(_) => start_piece_and_offset.clone(),
        };
        self.length -= length;

        if start_piece_and_offset[0] == stop_piece_and_offset[0]{
            if start_piece_and_offset[1] == 0{
                self.pieces[start_piece_and_offset[0]].offset += length;
                self.pieces[start_piece_and_offset[0]].length -= length;
                return
            }else if stop_piece_and_offset[1] == self.pieces[start_piece_and_offset[0]].length{
                self.pieces[start_piece_and_offset[0]].length -= length;
                return
            } 
        }

        let start_piece = self.pieces[start_piece_and_offset[0]];
        let stop_piece = self.pieces[stop_piece_and_offset[0]];

        let delete_pieces: Vec<Piece> = vec![
            Piece{
                buffer: start_piece.buffer,
                offset: start_piece.offset,
                length: start_piece_and_offset[1]
            },
            Piece{
                buffer: stop_piece.buffer,
                offset: stop_piece.offset + stop_piece_and_offset[1],
                length: stop_piece.length - stop_piece_and_offset[1]
            }
        ].into_iter().filter(|piece| piece.length > 0).collect();

        let delete_cnt = stop_piece_and_offset[0] - start_piece_and_offset[0];

        let mut pieces: Vec<Piece> = self.pieces[..start_piece_and_offset[0]].to_vec();
        pieces.extend(delete_pieces.iter());
        pieces.extend(self.pieces[(start_piece_and_offset[0]+delete_cnt+1)..].iter());
        self.pieces = pieces;
    }

    pub fn push(&mut self, text: String){
        self.insert(self.length, text);
    }

    pub fn pop(&mut self){
        let pieces_len = self.pieces.len();
        self.pieces[pieces_len - 1].length -= 1;
        if self.pieces[pieces_len - 1].length == 0{
            self.pieces.pop();
        }
        self.length -= 1;
    }

    pub fn get_text(&self) -> String{
        let mut text: String = String::from("");
        for piece in self.pieces.iter(){
            if piece.buffer == Buffer::Add{
                for i in piece.offset..piece.offset+piece.length{
                    text += &self.add[i].to_string();
                }
            }
            else{
                for i in piece.offset..piece.offset+piece.length{
                    text += &self.read[i].to_string();
                }
            }
        }
        return text;
    }

    pub fn get_length(&self) -> usize{
        self.length
    }
}