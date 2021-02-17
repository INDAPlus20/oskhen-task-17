use std::cmp;
use std::io::prelude::*;
use std::convert;

#[allow(dead_code)]

const MAXLENGTH: usize = 40;

fn main() {

    // Take input

    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer);
    let mut lines = buffer.split("\n");

    let mut wordlist: Vec<&str> = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim() == "#" {
            break
        }
        wordlist.push(line.trim());
    }

    // Initialize matrix

    let mut Dmatrix = [[99 as usize;MAXLENGTH+1];MAXLENGTH+1];

    for i in 0..MAXLENGTH+1 {
        Dmatrix[i][0] = i;
        Dmatrix[0][i] = i;
    }

    // minEdistance("aske", "maska", 40, &mut Dmatrix, 0);

    // printMatrix(&Dmatrix, 4+1, 5+1);
    // println!("{}", Dmatrix[4][5]);

    while let Some(line) = lines.next() {
        
        if line.len() == 0 {
            break;
        }

        minimalWords(line.trim(), &wordlist, &mut Dmatrix);

    }

}

fn minimalWords(source: &str, wordlist: &Vec<&str>, Dmatrix: &mut [[usize; MAXLENGTH+1]; MAXLENGTH+1]) {

    let m = source.chars().count();

    let mut minimumDistance = MAXLENGTH;

    let mut oldtarget = "";

    let mut closestWords: Vec<&&str> = Vec::new(); 

    for target in wordlist {

        let n = target.chars().count();

        if (m as isize - n as isize).abs() as usize > minimumDistance {
            continue
        }

        let characterSimilarity: usize = similarity(target, oldtarget); 

        minEdistance(source, target, minimumDistance, Dmatrix, characterSimilarity);

        let distance = Dmatrix[m][n];

        if distance < minimumDistance {
            minimumDistance = distance;
            closestWords.clear();
            closestWords.push(target);
        }
        else if distance == minimumDistance {
            closestWords.push(target);
        }

        oldtarget = target;
    }

    print!("{} ({}) ", source, minimumDistance);
    for word in closestWords {
        print!("{} ", word);
    }
    println!();

}

fn minEdistance(source: &str, target: &str, threshold: usize, Dmatrix: &mut [[usize; MAXLENGTH+1]; MAXLENGTH+1], offset: usize) {

    let m = source.chars().count();
    let n = target.chars().count();

    let p = ((threshold as isize - (n as isize - m as isize).abs() ) as f32 * 0.5).floor() as isize + 1;

    for i in 1..m+1 {

        if n >= m {

            let mut raisedfloor = i as isize - p;
            let loweredroof = ((n - m) as isize + p + i as isize) as usize;

            if raisedfloor < 0 { // Avoid underflows
                raisedfloor = 0;
            }

            for j in cmp::max(offset+1, raisedfloor as usize)..cmp::min(n+1, loweredroof) {

                let replace_cost;

                if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                    replace_cost = 0;
                } else {
                    replace_cost = 1;
                }

                let length_changing = cmp::min(Dmatrix[i-1][j] + 1, Dmatrix[i][j-1] + 1);
                
                Dmatrix[i][j] = cmp::min(Dmatrix[i-1][j-1] + replace_cost, length_changing);

            }

        }

        else {

            let mut raisedfloor = n as isize - m as isize - p + i as isize;
            let loweredroof = (i as isize + p) as usize;

            if raisedfloor < 0 { // Avoid underflows
                raisedfloor = 0;
            }


            for j in cmp::max(offset+1, raisedfloor as usize)..cmp::min(n+1, loweredroof) {

                let replace_cost;

                if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                    replace_cost = 0;
                } else {
                    replace_cost = 1;
                }

                let length_changing = cmp::min(Dmatrix[i-1][j] + 1, Dmatrix[i][j-1] + 1);

                Dmatrix[i][j] = cmp::min(Dmatrix[i-1][j-1] + replace_cost, length_changing);

            }

        }

    }

    // println!();
    // println!("source: {}, target: {}, distance: {}, offset: {}, threshold: {}, p: {}", source, target, Dmatrix[m][n], offset, threshold, p);
    // printMatrix(&Dmatrix, m+1, n+1);
    // println!();

}


fn similarity(source: &str, target: &str) -> usize {

    for (index,chars) in source.chars().zip(target.chars()).enumerate(){
        //print!("chars: {:#?} ",chars);
        if chars.0!=chars.1{
            return index;
        }
    }
    return cmp::min(source.chars().count(), target.chars().count())
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
