use std::{io::prelude::*, convert, time::SystemTime, collections::BinaryHeap, cmp, fmt, char::MAX, process::exit};


#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

const MAXLENGTH: usize = 40;

#[derive(Copy, Clone)]
struct charVec {
    array: [u8; 40],
    len: usize
}
// #[derive(Copy, Clone, Eq, PartialEq)]
// struct Node {
//     i: usize,
//     j: usize,
//     weight: usize,
// }

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> cmp::Ordering {
//         // Notice that the we flip the ordering on costs.
//         // In case of a tie we compare positions - this step is necessary
//         // to make implementations of `PartialEq` and `Ord` consistent.
//         other.weight.cmp(&self.weight)
//             .then_with(|| self.i.cmp(&other.i)).then_with(|| self.j.cmp(&other.j))
//     }
// }

// // `PartialOrd` needs to be implemented as well .
// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }


impl charVec {

    pub fn new() -> Self {
        Self {
            array: [0; 40],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, byte: u8) {
        self.array[self.len] = byte;
        self.len += 1;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn similar(&self, other: &charVec) -> usize {
        let minlen = cmp::min(self.len(), other.len());
        for i in 0..minlen {
            if self.array[i] != other.array[i] {
                return i;
            }
        }
        return minlen;
    }
}

impl fmt::Display for charVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for i in 0..self.len() {
            let b = self.array[i];
            match b {
                0x7b => write!(f, "å"),
                0x7c => write!(f, "ä"),
                0x7d => write!(f, "ö"),
                r => write!(f, "{}", r as char),
                _ => continue
            };
        }
        Ok(())
    }
}

impl fmt::Debug for charVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for b in self.array.iter() {
            write!(f, "{} ", b);
        }

        Ok(())
    }

}

fn main() {

    #[cfg(feature = "bench")]
    let nowTotal = SystemTime::now();
    
    // Take input

    let mut wordList: Vec<charVec> = Vec::with_capacity(500_000);

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    // | --

    // Convert and allocate as bytes

    let mut bytes = input.bytes();

    let mut wordBuffer = charVec::new();
    
    loop {
        if let Some(byte) = bytes.next() {
            match byte {
                b'#' => {
                    bytes.next();
                    wordBuffer.clear();
                    break;
                },
                b'\n' | b'\r' => {
                    wordList.push(wordBuffer);
                    wordBuffer.clear();
                }
                0xc3 => wordBuffer.push(
                        match bytes.next() {
                            Some(0xa5) => 0x7b, // å => {
                            Some(0xa4) => 0x7c, // ä => |
                            Some(0xb6) => 0x7d, // ö => }
                            _ => panic!("non-allowed character!")
                        }
                ),
                b => wordBuffer.push(b),
            }
        }
        else {
            break;
        }

    }

    // | --

    // Initialize matrix

    let mut dMatrix = [[99 as usize;MAXLENGTH+1];MAXLENGTH+1];

    for i in 0..MAXLENGTH+1 {
        dMatrix[i][0] = i;
        dMatrix[0][i] = i;
    }

    // | --


    // Do for each misspelled word
    'outer: loop {

        let mut misspelledWord = charVec::new();

        loop {
            if let Some(byte) = bytes.next() {
                match byte {
    
                    b'\n' | b'\r' => {
                        break;
                    }
                    0xc3 => misspelledWord.push(
                            match bytes.next() {
                                Some(0xa5) => 0x7b, // å => {
                                Some(0xa4) => 0x7c, // ä => |
                                Some(0xb6) => 0x7d, // ö => }
                                _ => panic!("non-allowed character!")
                            }
                    ),
                    b => misspelledWord.push(b),
                }
    
            } else {
                break 'outer;
            }
        }

        // "Global" Assignments
        let m = misspelledWord.len();
        let mut minimumDistance = MAXLENGTH;
        let mut oldtarget = charVec::new();
        let mut closestWords: Vec<charVec> = Vec::with_capacity(100);

        for target in wordList.iter() {

            let n = target.len();
            if (if m > n {m - n} else {n - m} > minimumDistance) { continue }
        
            let similarity = oldtarget.similar(target);

            eDist(&misspelledWord, target, minimumDistance, &mut dMatrix, similarity);

            let distance = dMatrix[m][n];

            if distance < minimumDistance {
                minimumDistance = distance;
                closestWords.clear();
                closestWords.push(*target);
            }
            else if distance == minimumDistance {
                closestWords.push(*target);
            }
            oldtarget = *target;
            
        }

        print!("{} ({}) ", misspelledWord, minimumDistance);
        for word in closestWords {
            print!("{} ", word);
        }
        println!();
    }

    // | --
    
    #[cfg(feature = "bench")]
    println!("{:?}", nowTotal.elapsed().unwrap());
}

fn eDist(source: &charVec, target: &charVec, mut k: usize, dMatrix: &mut [[usize; MAXLENGTH+1]; MAXLENGTH+1], mut offset: usize) {

    let m = source.len();
    let n = target.len();

    if k < cmp::min(m, n) {

        let P1y = cmp::max(0, offset as isize - k as isize) as usize;
        let P2y = n-k;
        let P3y = cmp::max(offset+k, P2y);

        #[cfg(feature = "debug_specific")]
        println!("P1y: {}, P2y: {}, P3y: {}, k: {}, offset: {}, m: {}, n: {}", P1y, P2y, P3y, k, offset, m, n);
    
        for i in P1y+1..=P2y {
    
            for j in offset+1..=i+k {

                #[cfg(feature = "debug_specific")]
                println!("L1: {} : {}", i, j);

                let replace_cost = (source.array[(i - 1)] != target.array[(j - 1)]) as usize;
                let length_changing = cmp::min(dMatrix[i-1][j] + 1, dMatrix[i][j-1] + 1);
                
                dMatrix[i][j] = cmp::min(dMatrix[i-1][j-1] + replace_cost, length_changing);
    
            }
    
        }
        
        for i in P2y+1..=P3y {

            for j in offset+1..=n {

                #[cfg(feature = "debug_specific")]
                println!("L2: {} : {}", i, j);

                let replace_cost = (source.array[(i - 1)] != target.array[(j - 1)]) as usize;
                let length_changing = cmp::min(dMatrix[i-1][j] + 1, dMatrix[i][j-1] + 1);
                
                dMatrix[i][j] = cmp::min(dMatrix[i-1][j-1] + replace_cost, length_changing);

            }

        }

        for i in P3y+1..=m {

            for j in i-k..=n {
                
                #[cfg(feature = "debug_specific")]
                println!("L3: {} : {}", i, j);

                let replace_cost = (source.array[(i - 1)] != target.array[(j - 1)]) as usize;
                let length_changing = cmp::min(dMatrix[i-1][j] + 1, dMatrix[i][j-1] + 1);
                
                dMatrix[i][j] = cmp::min(dMatrix[i-1][j-1] + replace_cost, length_changing);

            }
        }








    }
    else {

        for i in 1..m+1 {

            let raisedFloor = cmp::max( (i as isize - k as isize), (offset as isize + 1) ) as usize;
            let loweredCeil = cmp::min( (i as isize + k as isize) , (n as isize) ) as usize;

            for j in raisedFloor..=loweredCeil {

                let replace_cost = (source.array[(i - 1)] != target.array[(j - 1)]) as usize;
                let length_changing = cmp::min(dMatrix[i-1][j] + 1, dMatrix[i][j-1] + 1);

                dMatrix[i][j] = cmp::min(dMatrix[i-1][j-1] + replace_cost, length_changing);
            }

        }

    }
    #[cfg(feature = "debug_specific")]
    println!("{} -> {}: {}", source, target, dMatrix[m][n]);
    #[cfg(feature = "debug_specific")]
    printMatrix(dMatrix, m+1, n+1);

}

fn printMatrix(matrix: &[[usize; MAXLENGTH+1]; MAXLENGTH+1], m: usize, n: usize) {
    for row in matrix[0..m].iter() {
        for el in row[0..n].iter() {
            print!("{} ", el);
        }
        println!();
    }
    println!();
}
