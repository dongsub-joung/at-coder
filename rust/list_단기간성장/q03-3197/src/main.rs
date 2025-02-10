// BFS  - https://docs.rs/pathfinding/latest/pathfinding/directed/bfs/index.html
// or   - https://docs.rs/petgraph/latest/petgraph/visit/struct.Bfs.html
// implem - https://gist.github.com/vTurbine/16fbb99225ad4c0ac80b24855dd61a7c
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn successors(&self) -> Vec<Pos> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
         Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
  }
}

static GOAL: Pos = Pos(4, 6);
let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);
assert_eq!(result.expect("no path found").len(), 5);
