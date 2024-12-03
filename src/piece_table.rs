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

struct PieceTable{
    read: String,
    add: String,
    pieces: Vec<Piece>,
    length: usize,
}

impl PieceTable{
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
        self.length += text.len();

        if cur_piece.buffer == Buffer::Add{
            if cur_piece.length + cur_piece.offset == self.add.len() && cur_piece.length == piece_and_offset[1]{
                cur_piece.length += text.len();
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
                offset: self.add.len(),
                length: text.len()
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
        let stop_piece_and_offset: Vec<usize> = self.get_piece_by_index(idx + length).unwrap();
        self.length -= length;

        if start_piece_and_offset[0] == stop_piece_and_offset[0]{
            let mut piece = self.pieces[start_piece_and_offset[0]];

            if start_piece_and_offset[1] == 0{
                piece.offset += length;
                piece.length -= length;
                return
            }else if stop_piece_and_offset[1] == piece.length{
                piece.length -= length;
                return
            } 
        }

        let start_piece = self.pieces[start_piece_and_offset[0]];
        let stop_piece = self.pieces[stop_piece_and_offset[0]];

        let delete_pieces: Vec<Piece> = vec![
            Piece{
                buffer: Buffer::Add,
                offset: start_piece.offset,
                length: start_piece_and_offset[1]
            },
            Piece{
                buffer: Buffer::Add,
                offset: stop_piece.offset,
                length: stop_piece.length - stop_piece_and_offset[1]
            }
        ].into_iter().filter(|piece| piece.length > 0).collect();

        let delete_cnt = stop_piece_and_offset[0] - start_piece_and_offset[0];

        let mut pieces: Vec<Piece> = self.pieces[..start_piece_and_offset[0]].to_vec();
        pieces.extend(delete_pieces.iter());
        pieces.extend(self.pieces[start_piece_and_offset[0]+delete_cnt..].iter());
    }

    pub fn get_text(&self) -> String{
        let mut text: String = String::from("");
        for piece in self.pieces.iter(){
            if piece.buffer == Buffer::Add{
                text += &self.add[piece.offset..piece.offset+piece.length];
            }
            else{
                text += &self.read[piece.offset..piece.offset+piece.length];
            }
        }
        return text;
    }
}