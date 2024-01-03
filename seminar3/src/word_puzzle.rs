use std::collections::HashSet;

/**
    The Algorithms:
    The input consists of a two-dimensional array of letters and a list of words. The object is to find
    the words in the puzzle. These words may be horizontal, vertical, or diagonal in any direction. As
    an example, the puzzle shown in Fig. 2 contains the words this, two, fat, and that. The word this
    begins at row 1, column 1, or (1,1), and extends to (1,4); two goes from (1,1) to (3,1); fat goes
    from (4,1) to (2,3); and that goes from (4,4) to (1,1).
    There are at least two straightforward algorithms that solve the problem. For each word in the
    word list, we check each ordered triple (row, column, orientation) for the presence of the word.
    This amounts to lots of nested for loops but is basically straightforward.



    Alternatively, for each ordered quadruple (row, column, orientation, number of characters) that
    doesn’t run off an end of the puzzle, we can test whether the word indicated is in the word list.
    Again, this amounts to lots of nested for loops. It is possible to save some time if the maximum
    number of characters in any word is known. (p.22, [1])



    Running time analysis:
    If the second algorithm is used, and we assume that the maximum word size is some small
    constant, then the time to read in the dictionary containing W words and put it in a hash table is
    O(W). This time is likely to be dominated by the disk I/O and not the hashing routines. The rest of
    the algorithm would test for the presence of a word for each ordered quadruple (row, column,
    orientation, number of characters). As each lookup would be O(1), and there are only a constant
    number of orientations (8) and characters per word, the running time of this phase would be
    O(R·C). The total running time would be O(R·C+W), which is a distinct improvement over the
    original O(R·C·W). We could make further optimizations, which would decrease the running time
    in practice; these are described in the exercises. (p.238, [1])

    method 1:
    for each word: check

    NOTES:
    - words can go backwards
**/

#[derive(Debug, PartialEq)]
pub struct Location(usize, usize);
pub type LocRange = (Location, Location);

pub type WordPuzzle<'a> = &'a [&'a [char]];

pub fn solve_word_puzzle(
    puzzle: WordPuzzle,
    dictionary: &[&str],
    max_word_size: usize,
) -> Vec<LocRange> {
    // TODO: change back to add prefixes back
    let dictionary = generate_prefix_hashset(dictionary);
    let mut found = Vec::new();

    for y in 0..puzzle.len() {
        for x in 0..puzzle[0].len() {
            for size in 1..=max_word_size {
                // TODO: include functionality for scanning first for prefixes

                scan_horizontal(puzzle, &dictionary, x, y, size).map(|x| found.push(x));
                scan_vertical(puzzle, &dictionary, x, y, size).map(|x| found.push(x));
                scan_oblique_left(puzzle, &dictionary, x, y, size).map(|x| found.push(x));
                scan_oblique_right(puzzle, &dictionary, x, y, size).map(|x| found.push(x));
            }
        }
    }

    found
}

/// Scans the puzzle horizontally for a sized word. Assumes there are no palindromes.
fn scan(
    dict: &HashSet<&str>,
    str: &str,
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
) -> Option<LocRange> {
    if dict.contains(str) {
        return Some((Location(x, y), Location(x2, y2)));
    }
    let reverse = str.chars().rev().collect::<String>();
    if dict.contains(reverse.as_str()) {
        return Some((Location(x2, y2), Location(x, y)));
    }

    None
}

fn scan_horizontal(
    puzzle: WordPuzzle,
    dict: &HashSet<&str>,
    x: usize,
    y: usize,
    size: usize,
) -> Option<LocRange> {
    let x2 = x + size - 1;
    let y2 = y;
    if x2 >= puzzle[0].len() || y2 >= puzzle.len() {
        return None;
    }

    // build string
    let mut str = String::new();
    for offset in 0..size {
        str.push(puzzle[y][x + offset]);
    }

    scan(dict, &str, x, y, x2, y2)
}

fn scan_vertical(
    puzzle: WordPuzzle,
    dict: &HashSet<&str>,
    x: usize,
    y: usize,
    size: usize,
) -> Option<LocRange> {
    let x2 = x;
    let y2 = y + size - 1;
    if x2 >= puzzle[0].len() || y2 >= puzzle.len() {
        return None;
    }

    // build string
    let mut str = String::new();
    for offset in 0..size {
        str.push(puzzle[y + offset][x]);
    }

    scan(dict, &str, x, y, x2, y2)
}

fn scan_oblique_left(
    puzzle: WordPuzzle,
    dict: &HashSet<&str>,
    x: usize,
    y: usize,
    size: usize,
) -> Option<LocRange> {
    let x2 = x as isize + 1 - size as isize;
    let y2 = y + size - 1;
    if x2 < 0 || y2 >= puzzle.len() {
        return None;
    }

    // build string
    let mut str = String::new();
    for offset in 0..size {
        str.push(puzzle[y + offset][x - offset]);
    }

    scan(dict, &str, x, y, x2 as usize, y2 as usize)
}

fn scan_oblique_right(
    puzzle: WordPuzzle,
    dict: &HashSet<&str>,
    x: usize,
    y: usize,
    size: usize,
) -> Option<LocRange> {
    let x2 = x + size - 1;
    let y2 = y + size - 1;
    if x2 >= puzzle[0].len() || y2 >= puzzle.len() {
        return None;
    }

    // build string
    let mut str = String::new();
    for offset in 0..size {
        str.push(puzzle[y + offset][x + offset]);
    }

    scan(dict, &str, x, y, x2, y2)
}

fn generate_prefix_hashset<'a>(dictionary: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for word in dictionary {
        // if word.len() > 2 {
        //     set.insert(word.split_at(2).0);
        // }
        set.insert(*word);
    }
    set
}

#[cfg(test)]
mod tests {
    use crate::word_puzzle::{solve_word_puzzle, Location};

    const DICTIONARY: [&str; 4] = ["this", "two", "fat", "that"];
    const PUZZLE: [[char; 4]; 4] = [
        ['t', 'h', 'i', 's'],
        ['w', 'a', 't', 's'],
        ['o', 'a', 'h', 'g'],
        ['f', 'g', 'd', 't'],
    ];

    #[test]
    fn test_solve_word_puzzle() {
        let word_puzzle = &PUZZLE.iter().map(|row| &row[..]).collect::<Vec<_>>()[..];
        let result = solve_word_puzzle(word_puzzle, &DICTIONARY, 4);

        dbg!(&result);
        assert!(result.contains(&(Location(0, 0), Location(3, 0)))); // this
        assert!(result.contains(&(Location(0, 0), Location(0, 2)))); // two
        assert!(result.contains(&(Location(0, 3), Location(2, 1)))); // fat
        assert!(result.contains(&(Location(3, 3), Location(0, 0)))); // that
    }
}
