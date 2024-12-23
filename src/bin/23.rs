advent_of_code::solution!(23);

// Needed to load in the linker to use BLAS
extern crate accelerate_provider;

// use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::{c_char, c_double, c_float, c_int};

// See https://github.com/blas-lapack-rs/accelerate-src
extern "C" {
    fn ssymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );

    // The return type is `double` because of the bug in Apple's Accelerate.
    // See https://stackoverflow.com/a/77017238
    fn sdot_(
        n: *const c_int,
        x: *const c_float,
        incx: *const c_int,
        y: *const c_float,
        incy: *const c_int,
    ) -> c_double;
}

const ALPHABET_SIZE: usize = 26;
const TABLE_DIM: usize = ALPHABET_SIZE * ALPHABET_SIZE; // Max number of 2 letter combinations
const TABLE_SIZE: usize = TABLE_DIM * TABLE_DIM; // Max number of 2 letter computer connections

type GraphMatrix = Box<[c_float]>;
type GraphList = HashMap<usize, HashSet<usize>>;

fn to_key(s: &str) -> usize {
    debug_assert_eq!(s.len(), 2);
    let buf = s.as_bytes();
    (usize::from(buf[0] - b'a') * 26) + usize::from(buf[1] - b'a')
}

fn from_key(k: usize) -> String {
    debug_assert!(k < TABLE_DIM);
    let (a, b) = (k / 26, k % 26);
    String::from_utf8(vec![a as u8 + b'a', b as u8 + b'a'])
        .unwrap()
        .to_string()
}

// Calculate the number of triangles in the graph using BLAS on a square matrix
// Unsafe because of the FFI calls to BLAS
fn triangles(table: &GraphMatrix, square_tmp: &mut GraphMatrix) -> i32 {
    unsafe {
        ssymm_(
            &(b'L' as c_char),
            &(b'U' as c_char),
            &(TABLE_DIM as c_int),
            &(TABLE_DIM as c_int),
            &1.0,
            table.as_ptr(),
            &(TABLE_DIM as c_int),
            table.as_ptr(),
            &(TABLE_DIM as c_int),
            &0.0,
            square_tmp.as_mut_ptr(),
            &(TABLE_DIM as c_int),
        );
        let sum = sdot_(
            &(TABLE_SIZE as c_int),
            square_tmp.as_ptr(),
            &1,
            table.as_ptr(),
            &1,
        ) as i32;
        debug_assert_eq!(sum % 6, 0);
        sum / 6
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    // Naive implementation here - takes multiple seconds on main input
    // let mut connections = HashMap::new();
    // for line in input.lines() {
    //     let (from, to) = line.split_once("-")?;
    //     connections.entry(from).or_insert_with(Vec::new).push(to);
    //     connections.entry(to).or_insert_with(Vec::new).push(from);
    // }

    // connections
    //     .iter()
    //     .combinations(3)
    //     .filter(|comb| {
    //         let (a, b, c) = (comb[0], comb[1], comb[2]);

    //         let triplet = a.1.contains(b.0) && a.1.contains(c.0) && b.1.contains(c.0);
    //         triplet && comb.iter().any(|c| c.0.starts_with('t'))
    //     })
    //     .count()
    //     .into()
    //

    // BLAS implementation using adjacency matrix
    let mut table: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut table_no_t: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut graph: GraphList = GraphList::new();
    let t_range = to_key("ta")..=to_key("tz");
    for line in input.lines() {
        let (i, j) = line.split_once("-").unwrap();
        let (i, j) = (to_key(i), to_key(j));

        table[i * TABLE_DIM + j] = 1.0;
        table[j * TABLE_DIM + i] = 1.0;
        if !t_range.contains(&i) && !t_range.contains(&j) {
            table_no_t[i * TABLE_DIM + j] = 1.0;
            table_no_t[j * TABLE_DIM + i] = 1.0;
        }
        graph.entry(i).or_default().insert(j);
        graph.entry(j).or_default().insert(i);
    }
    let mut square_tmp: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let trace = triangles(&table, &mut square_tmp);
    let trace_no_t = triangles(&table_no_t, &mut square_tmp);
    Some(trace - trace_no_t)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut table: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut table_no_t: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut graph: GraphList = GraphList::new();
    let t_range = to_key("ta")..=to_key("tz");
    for line in input.lines() {
        let (i, j) = line.split_once("-").unwrap();
        let (i, j) = (to_key(i), to_key(j));

        table[i * TABLE_DIM + j] = 1.0;
        table[j * TABLE_DIM + i] = 1.0;
        if !t_range.contains(&i) && !t_range.contains(&j) {
            table_no_t[i * TABLE_DIM + j] = 1.0;
            table_no_t[j * TABLE_DIM + i] = 1.0;
        }
        graph.entry(i).or_default().insert(j);
        graph.entry(j).or_default().insert(i);
    }

    let mut queue: VecDeque<(GraphList, Vec<usize>)> = VecDeque::new();
    queue.push_back((graph, vec![]));
    let mut best = Vec::<usize>::new();
    while let Some((mut graph, clique)) = queue.pop_front() {
        if graph.is_empty() {
            if clique.len() > best.len() {
                best = clique;
            }
            continue;
        }

        let candidate = *graph.iter().min_by_key(|(_, ns)| ns.len()).unwrap().0;
        let neighbors = &graph[&candidate];
        if clique.len() + 1 + neighbors.len() > best.len() {
            let mut neighbors_graph = GraphList::new();
            neighbors.iter().for_each(|&n| {
                neighbors_graph.insert(n, graph[&n].intersection(neighbors).copied().collect());
            });
            queue.push_back((
                neighbors_graph,
                clique.iter().copied().chain([candidate]).collect(),
            ));
        }

        if clique.len() + graph.len() - 1 > best.len() {
            for n in neighbors.clone() {
                graph.get_mut(&n).unwrap().remove(&candidate);
            }
            graph.remove(&candidate);
            queue.push_back((graph, clique));
        }
    }
    best.sort();
    Some(
        best.into_iter()
            .map(from_key)
            .collect::<Vec<String>>()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
