#[allow(clippy::needless_return)]

mod wordpuzzle;
mod utils;

use wordpuzzle::WordPuzzle;

fn main() {
    let puzzle = WordPuzzle::default();
    puzzle.puzzle();
}
