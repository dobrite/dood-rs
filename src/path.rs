
use std::collections::{HashMap, BinaryHeap};

use loc::Loc;
use grid::Grid;
use state::State;
use cascadecs::entity::Entity;

pub enum PathTarget {
    Entity(Entity),
    Loc(Loc),
    None,
}

pub type Path = Vec<Loc>;

pub fn path(grid: &Grid, start: Loc, goal: Loc) -> Path {
    let blocked = vec![]; // TODO do something with this

    let mut frontier = BinaryHeap::new();
    frontier.push(State { cost: 0, loc: start });
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(start, Loc { x: -1, y: -1 });
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        if current.loc == goal {
            break;
        };
        for next in grid.neighbors(current.loc, &blocked).iter() {
            // TODO implement costs for terrain
            let new_cost: usize = cost_so_far.get(&current.loc).unwrap() + 1;
            if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap() {
                cost_so_far.insert(*next, new_cost);
                frontier.push(State { cost: new_cost, loc: *next });
                came_from.insert(*next, current.loc);
            }
        }
    }

    came_from.to_path(start, goal)
}

pub type PathContour = HashMap<Loc, Loc>;

pub trait ToPath {
    fn to_path(&self, from: Loc, to: Loc) -> Path;
}

impl ToPath for PathContour {
    fn to_path(&self, start: Loc, goal: Loc) -> Path {
        let mut current = goal;
        let mut path = vec![current];

        while current != start {
            match self.get(&current) {
                Some(next) => {
                    path.push(*next);
                    current = *next
                }
                None => {}
            }
        }

        // TODO reverse?
        path
    }
}

//trait Mult<T> {
//  fn mult(&self) -> i32;
//}
//
//struct Pair {
//  a: i32,
//  b: i32,
//}
//
//// Marker types; because they have no variants, it's impossible
//// to create an actual value with these types.
//enum SimpleMult { }
//enum FoldMult { }
//
//impl Mult<SimpleMult> for Pair {
//  fn mult(&self) -> i32 {
//    self.a * self.b
//  }
//}
//
//impl Mult<FoldMult> for Pair {
//  fn mult(&self) -> i32 {
//    (0..self.a).fold(0, |acc, _| acc + self.b)
//  }
//}

//fn main() {
//  println!("{:?}", <Mult<SimpleMult>>::mult(&Pair { a: 3, b: 4 }));
//}

//class GridWithWeights(SquareGrid):
//    def __init__(self, width, height):
//        super().__init__(width, height)
//        self.weights = {}
//
//    def cost(self, a, b):
//        this only relies on b but could rely on a
//        return self.weights.get(b, 1)
//import heapq
//
//class PriorityQueue:
//    def __init__(self):
//        self.elements = []
//
//    def empty(self):
//        return len(self.elements) == 0
//
//    def put(self, item, priority):
//        heapq.heappush(self.elements, (priority, item))
//
//    def get(self):
//        return heapq.heappop(self.elements)[1]
//
//def dijkstra_search(graph, start, goal):
//    frontier = PriorityQueue()
//    frontier.put(start, 0)
//    came_from = {}
//    cost_so_far = {}
//    came_from[start] = None
//    cost_so_far[start] = 0
//
//    while not frontier.empty():
//        current = frontier.get()
//
//        if current == goal:
//            break
//
//        for next in graph.neighbors(current):
//            new_cost = cost_so_far[current] + graph.cost(current, next))
//            if next not in cost_so_far or new_cost < cost_so_far[next]:
//                cost_so_far[next] = new_cost
//                priority = new_cost
//                frontier.put(next, priority)
//                came_from[next] = current
//
//    return came_from, cost_so_far
//
//breadth first
//fn path(&self, grid: &Grid, blocked: &Vec<Loc>, start: Loc, goal: Loc) -> Path {
//    let mut frontier: VecDeque<Loc> = VecDeque::new();
//    frontier.push_back(start);
//    let mut came_from: PathContour = HashMap::new();
//    came_from.insert(start, (-1, -1)); // prob a bad idea
//
//    while !frontier.is_empty() {
//        let current = frontier.pop_front().unwrap();
//        if current == goal { break }
//        for next in grid.neighbors(current, blocked).iter() {
//            if !came_from.contains_key(next) {
//                frontier.push_back(*next);
//                came_from.insert(*next, current);
//            }
//        }
//    }
//
//    return came_from.to_path(start, goal)
//}
//
