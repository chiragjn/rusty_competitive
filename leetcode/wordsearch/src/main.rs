// https://leetcode.com/problems/word-search/

struct Solution {}

struct Finder<'a> {
    board: &'a Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}

impl<'a> Finder<'a> {
    fn new(board: &'a Vec<Vec<char>>) -> Self {
        let mut visited = vec![];
        for i in 0..board.len() {
            visited.push(vec![false; board[i].len()]);
        }
        return Finder {
            board: board,
            visited: visited,
        };
    }

    fn recurse(&mut self, i: usize, j: usize, k: usize, chars: &Vec<char>) -> bool {
        if k >= chars.len() {
            return true;
        }
        if i >= self.board.len()
            || j >= self.board[i].len()
            || self.visited[i][j]
            || self.board[i][j] != chars[k]
        {
            return false;
        }
        let mut possible = false;
        self.visited[i][j] = true;
        let mut cords = vec![(i + 1, j), (i, j + 1)];
        if i > 0 {
            cords.push((i - 1, j));
        }
        if j > 0 {
            cords.push((i, j - 1));
        }
        for (a, b) in cords {
            possible |= self.recurse(a, b, k + 1, chars);
            if possible {
                break;
            }
        }

        self.visited[i][j] = false;
        return possible;
    }

    fn find(&mut self, word: &String) -> bool {
        if word.len() == 0 {
            return true;
        }
        let chars: Vec<char> = word.chars().collect();
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == chars[0] && self.recurse(i, j, 0, &chars) {
                    return true;
                }
            }
        }
        return false;
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut f = Finder::new(&board);
        return f.find(&word);
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(Solution::exist(board.clone(), String::from("ABCCED")), true);
    assert_eq!(Solution::exist(board.clone(), String::from("SEE")), true);
    assert_eq!(Solution::exist(board.clone(), String::from("ABCB")), false);
}
