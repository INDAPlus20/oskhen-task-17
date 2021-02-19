use std::{cmp, fmt::Display};
use std::io::prelude::*;
use std::convert;
use std::time::{Duration, SystemTime};
use std::process;
use std::fmt;

use cmp::min;
use process::exit;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

const MAXLENGTH: usize = 40;

#[derive(Copy, Clone)]
struct charVec {
    array: [u8; 40],
    len: usize
}

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
        let minlen = min(self.len(), other.len());
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

    // for word in wordList.iter() {
    //     println!("{}", word);
    // }

    // process::exit(1);

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
        //if misspelledWord.len() == 0 {continue }

        // println!("word: {:?}, {}", misspelledWord, misspelledWord.len());

        // "Global" Assignments
        let m = misspelledWord.len();
        let mut minimumDistance = MAXLENGTH;
        let mut oldtarget = charVec::new();
        let mut closestWords: Vec<charVec> = Vec::with_capacity(100);

        for target in wordList.iter() {
            // "Local" Assignments
            let n = target.len();
            if (if m > n {m - n} else {n - m} > minimumDistance) {
                continue
            }
        
            let similarity = oldtarget.similar(target);

            eDist(&misspelledWord, target, minimumDistance as isize, &mut dMatrix, similarity);

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

fn eDist(source: &charVec, target: &charVec, mut k: isize, Dmatrix: &mut [[usize; MAXLENGTH+1]; MAXLENGTH+1], mut offset: usize) {

    let m = source.len();
    let n = target.len();

    k = min(((m+n) as f32 * 0.5).ceil() as isize, k); 

    let absdiff = if m >= n {m - n} else {n - m};

    for i in 1..m+1 {

        let raisedFloor = cmp::max( (i as isize - k), (offset as isize + 1) ) as usize;
        let loweredCeil = cmp::min( (k+i as isize) , (n as isize) ) as usize;

        // if (raisedFloor != offset+1) | (loweredCeil != n) {panic!()}

        #[cfg(feature = "debug_specific")] {
            print!("for i: {}, ceil: {}, floor {}", i, loweredCeil, raisedFloor);
        }

        for j in raisedFloor..=loweredCeil {

            #[cfg(feature = "debug_specific")] {
                print!("j: {} ", j);
            }

            let replace_cost = (source.array[(i - 1)] != target.array[(j - 1)]) as usize;
            let length_changing = cmp::min(Dmatrix[i-1][j] + 1, Dmatrix[i][j-1] + 1);
            
            Dmatrix[i][j] = cmp::min(Dmatrix[i-1][j-1] + replace_cost, length_changing);

        }
        #[cfg(feature = "debug_specific")] {
            println!();
        }


    }


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