mod utils;
#[allow(clippy::needless_return)]
mod wordpuzzle;

use wordpuzzle::WordPuzzle;

fn main() {
    let puzzle = WordPuzzle::default();
    puzzle.puzzle();
}
