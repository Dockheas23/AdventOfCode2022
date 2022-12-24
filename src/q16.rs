use crate::parser::*;
use std::collections::{HashMap, HashSet};
use std::io;

const INPUT_FILE: &str = "input/input16.txt";
const _INPUT_FILE_SAMPLE: &str = "input/input16_sample.txt";

#[derive(Debug)]
struct Vertex(String, usize);

#[derive(Debug)]
struct Edge(String, String, usize);

#[derive(Debug)]
struct Input {
    vertices: HashMap<String, Vertex>,
    edges: HashMap<String, Vec<Edge>>,
    important_vertices: HashSet<String>,
}

impl TryFrom<&mut FileLines> for Input {
    type Error = io::Error;

    fn try_from(lines: &mut FileLines) -> Result<Self, Self::Error> {
        let mut vertices = HashMap::new();
        let mut edges = HashMap::new();
        let mut important_vertices = HashSet::new();
        for line in lines {
            let trimmed = line
                .trim_start_matches("Valve ")
                .replace("has flow rate=", "")
                .replace("; ", "")
                .replace(|c: char| c.is_ascii_lowercase(), "");
            let mut parts = trimmed.splitn(3, ' ');
            let name = parts.next().unwrap();
            let flow = parts.next().unwrap().parse().unwrap();
            vertices.insert(String::from(name), Vertex(String::from(name), flow));
            if flow > 0 {
                important_vertices.insert(String::from(name));
            }
            let mut neighbours = Vec::new();
            for neighbour in parts.next().unwrap().trim().split(", ") {
                neighbours.push(Edge(String::from(name), String::from(neighbour), 1));
            }
            edges.insert(String::from(name), neighbours);
        }
        Ok(Input {
            vertices,
            edges,
            important_vertices,
        })
    }
}

pub fn part_1() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_1(input))
}

fn _part_1_sample() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_1(input))
}

fn do_part_1(input: Input) -> usize {
    const START_POINT: &str = "AA";
    const TIME_REMAINING: usize = 30;
    let shortest_paths =
        build_shortest_paths(&input.vertices, &input.edges, &input.important_vertices);
    find_best_flow_from(
        &input.vertices,
        &shortest_paths,
        &HashSet::new(),
        START_POINT,
        TIME_REMAINING,
    )
}

pub fn part_2() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(INPUT_FILE)?)?;
    Ok(do_part_2(input))
}

fn _part_2_sample() -> io::Result<usize> {
    let input = Input::try_from(&mut FileLines::new(_INPUT_FILE_SAMPLE)?)?;
    Ok(do_part_2(input))
}

fn do_part_2(input: Input) -> usize {
    const START_POINT: &str = "AA";
    const TIME_REMAINING: usize = 26;
    let vertices = &input.vertices;
    let shortest_paths = build_shortest_paths(vertices, &input.edges, &input.important_vertices);
    let opened: HashSet<&str> = HashSet::new();
    find_best_double_route(
        vertices,
        &shortest_paths,
        &opened,
        START_POINT,
        START_POINT,
        TIME_REMAINING,
        TIME_REMAINING,
    )
}

fn build_shortest_paths<'a>(
    vertices: &'a HashMap<String, Vertex>,
    edges: &'a HashMap<String, Vec<Edge>>,
    important_vertices: &'a HashSet<String>,
) -> HashMap<&'a str, HashMap<String, usize>> {
    let mut shortest_paths = HashMap::new();
    shortest_paths.insert("AA", shortest_paths_from(vertices, edges, "AA"));
    for vertex in important_vertices {
        shortest_paths.insert(vertex, shortest_paths_from(vertices, edges, vertex));
    }
    shortest_paths
}

fn shortest_paths_from(
    vertices: &HashMap<String, Vertex>,
    edges: &HashMap<String, Vec<Edge>>,
    from: &str,
) -> HashMap<String, usize> {
    let mut paths: HashMap<String, usize> = HashMap::new();
    let mut queue: Vec<String> = Vec::new();
    let mut done: HashSet<String> = HashSet::new();
    if let Some(vertex) = vertices.get(from) {
        let name = vertex.0.as_str();
        if let Some(edges) = edges.get(name) {
            for edge in edges {
                let neighbour = edge.1.as_str();
                queue.push(String::from(neighbour));
                paths.insert(String::from(neighbour), 1);
            }
        }
        paths.insert(String::from(name), 0);
        done.insert(String::from(name));
        while !queue.is_empty() {
            let (i, nearest) = queue
                .iter()
                .enumerate()
                .min_by_key(|(_, v)| paths.get(v.as_str()).unwrap())
                .map(|(i, v)| (i, String::from(v.as_str())))
                .unwrap();
            queue.remove(i);
            let distance = *paths.get(nearest.as_str()).unwrap();
            if let Some(edges) = edges.get(nearest.as_str()) {
                for edge in edges {
                    let neighbour = edge.1.as_str();
                    if !done.contains(neighbour) {
                        queue.push(String::from(neighbour));
                    }
                    if !paths.contains_key(neighbour)
                        || distance + 1 < *paths.get(neighbour).unwrap()
                    {
                        paths.insert(String::from(neighbour), distance + 1);
                    }
                }
            }
            done.insert(String::from(nearest.as_str()));
        }
        paths
            .into_iter()
            .filter(|(p, _)| vertices.get(p).unwrap().1 > 0)
            .collect()
    } else {
        HashMap::new()
    }
}

fn find_best_flow_from(
    vertices: &HashMap<String, Vertex>,
    shortest_paths: &HashMap<&str, HashMap<String, usize>>,
    opened: &HashSet<&str>,
    from: &str,
    time_left: usize,
) -> usize {
    if time_left <= 1 {
        0
    } else if let Some(vertex) = vertices.get(from) {
        let new_flow = vertex.1 * (time_left - 1);
        let time_spent_here = if new_flow == 0 { 0 } else { 1 };
        new_flow
            + shortest_paths
                .get(from)
                .unwrap()
                .iter()
                .filter(|(p, _)| p != &from && !opened.contains(p.as_str()))
                .map(|(p, d)| {
                    let new_time_left = if d + time_spent_here > time_left {
                        0
                    } else {
                        time_left - d - time_spent_here
                    };
                    find_best_flow_from(
                        vertices,
                        shortest_paths,
                        &opened.union(&HashSet::from([from])).map(|&x| x).collect(),
                        p,
                        new_time_left,
                    )
                })
                .max()
                .unwrap_or(0)
    } else {
        0
    }
}

fn expected_value(flow: usize, time_left: usize, distance: usize) -> usize {
    if distance + 1 >= time_left {
        0
    } else {
        flow * (time_left - distance - 1)
    }
}

fn top_n_vertices<'a>(
    vertices: &'a HashMap<String, Vertex>,
    shortest_paths: &'a HashMap<&str, HashMap<String, usize>>,
    opened: &HashSet<&str>,
    from: &str,
    time_left: usize,
    n: usize,
) -> Vec<&'a str> {
    let mut candidates = shortest_paths
        .get(from)
        .unwrap()
        .iter()
        .filter(|(v, _)| *v != from && !opened.contains(v.as_str()))
        .collect::<Vec<_>>();
    candidates
        .sort_by_key(|(v, &d)| expected_value(vertices.get(v.as_str()).unwrap().1, time_left, d));
    candidates.reverse();
    candidates
        .into_iter()
        .map(|(v, _)| v.as_str())
        .take(n)
        .collect()
}

fn get_flow(vertex: &Vertex, time_left: usize) -> (usize, usize) {
    if time_left <= 1 {
        (0, 0)
    } else {
        let flow = vertex.1 * (time_left - 1);
        if flow == 0 {
            (0, 0)
        } else {
            (flow, 1)
        }
    }
}

fn resolve_remaining_time(time_left: usize, cost_to_next: usize) -> usize {
    if cost_to_next >= time_left {
        0
    } else {
        time_left - cost_to_next
    }
}

fn find_best_double_route(
    vertices: &HashMap<String, Vertex>,
    shortest_paths: &HashMap<&str, HashMap<String, usize>>,
    opened: &HashSet<&str>,
    h_location: &str,
    e_location: &str,
    h_time_left: usize,
    e_time_left: usize,
) -> usize {
    const TOP_N: usize = 12;
    let (h_flow, h_time) = get_flow(vertices.get(h_location).unwrap(), h_time_left);
    let (e_flow, e_time) = get_flow(vertices.get(e_location).unwrap(), e_time_left);
    let new_opened = opened
        .union(&HashSet::from([h_location, e_location]))
        .map(|&x| x)
        .collect();
    let h_next = top_n_vertices(
        vertices,
        shortest_paths,
        &new_opened,
        h_location,
        h_time_left,
        TOP_N,
    );
    let e_next = top_n_vertices(
        vertices,
        shortest_paths,
        &new_opened,
        e_location,
        e_time_left,
        TOP_N,
    );
    assert_eq!(h_next.len(), e_next.len());
    if h_next.len() == 0 {
        return h_flow + e_flow;
    } else if h_next.len() == 1 {
        let v_name = h_next[0];
        let vertex = vertices.get(v_name).unwrap();
        let h_distance = shortest_paths.get(h_location).unwrap().get(v_name).unwrap();
        let e_distance = shortest_paths.get(e_location).unwrap().get(v_name).unwrap();
        let h_value = expected_value(vertex.1, h_time_left, h_distance + h_time);
        let e_value = expected_value(vertex.1, e_time_left, e_distance + e_time);
        return h_flow + e_flow + std::cmp::max(h_value, e_value);
    }
    let mut best_remaining = 0;
    for h in &h_next {
        for e in &e_next {
            if h == e {
                continue;
            }
            if h_location == "AA" && e > h {
                continue;
            }
            let h_distance = shortest_paths.get(h_location).unwrap().get(*h).unwrap();
            let e_distance = shortest_paths.get(e_location).unwrap().get(*e).unwrap();
            if h_distance + h_time < h_time_left && e_distance + e_time < e_time_left {
                let c = find_best_double_route(
                    vertices,
                    shortest_paths,
                    &new_opened,
                    h,
                    e,
                    resolve_remaining_time(h_time_left, h_distance + h_time),
                    resolve_remaining_time(e_time_left, e_distance + e_time),
                );
                if c > best_remaining {
                    best_remaining = c;
                }
            } else if h_distance + h_time < h_time_left {
                let c = find_best_flow_from(
                    vertices,
                    shortest_paths,
                    &new_opened,
                    h,
                    resolve_remaining_time(h_time_left, h_distance + h_time),
                );
                if c > best_remaining {
                    best_remaining = c;
                }
            } else if e_distance + e_time < e_time_left {
                let c = find_best_flow_from(
                    vertices,
                    shortest_paths,
                    &new_opened,
                    e,
                    resolve_remaining_time(e_time_left, e_distance + e_time),
                );
                if c > best_remaining {
                    best_remaining = c;
                }
            }
        }
    }
    h_flow + e_flow + best_remaining
}

#[cfg(test)]
mod tests {
    use super::{_part_1_sample, _part_2_sample, part_1, part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(1850, part_1().unwrap());
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(1651, _part_1_sample().unwrap());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(2306, part_2().unwrap());
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(1707, _part_2_sample().unwrap());
    }
}
