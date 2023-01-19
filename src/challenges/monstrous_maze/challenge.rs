use simplelog::debug;
use crate::challenges::monstrous_maze::types::input::MonstrousMazeInput;
use crate::challenges::monstrous_maze::types::map::Map;
use crate::challenges::monstrous_maze::types::output::MonstrousMazeOutput;
use std::collections::{BinaryHeap};

fn parse_input(input: MonstrousMazeInput) -> Map {
    let mut maze = Vec::new();
    let mut player = (0, 0);
    let mut exit = (0, 0);

    for (y, line) in input.grid.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'I' => player = (y, x),
                'Y' => player = (y, x),
                'X' => exit = (y, x),
                _ => (),
            }
            row.push(ch);
        }
        maze.push(row);
    }

    Map {
        maze,
        player,
        exit,
    }
}

fn find_path(map: &Map) -> Option<String> {
    let mut distances = vec![std::usize::MAX; map.maze.len() * map.maze[0].len()];
    let mut heap = BinaryHeap::new();
    let mut prev = vec![(0, 0); map.maze.len() * map.maze[0].len()];

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];

    distances[map.player.0 * map.maze[0].len() + map.player.1] = 0;
    heap.push((map.player.0, map.player.1));

    for _ in 0..map.maze.len() * map.maze[0].len() {
        if let Some((x, y)) = heap.pop() {
            let distance = distances[x * map.maze[0].len() + y];

            for i in 0..4 {
                let nx = x as isize + dx[i];
                let ny = y as isize + dy[i];

                if nx >= 0 && ny >= 0 && nx < map.maze.len() as isize && ny < map.maze[0].len() as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    let new_distance = distance + 1;

                    if map.maze[nx][ny] != '#' && distances[nx * map.maze[0].len() + ny] == std::usize::MAX {
                        distances[nx * map.maze[0].len() + ny] = new_distance;
                        prev[nx * map.maze[0].len() + ny] = (x, y);
                        heap.push((nx, ny));
                    } else if map.maze[nx][ny] != '#' && new_distance < distances[nx * map.maze[0].len() + ny] {
                        distances[nx * map.maze[0].len() + ny] = new_distance;
                        prev[nx * map.maze[0].len() + ny] = (x, y);
                        heap.push((nx, ny));
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut path = String::new();
    let mut current = (map.exit.0, map.exit.1);

    while current != (map.player.0, map.player.1) {
        let prev = prev[current.0 * map.maze[0].len() + current.1];
        let dx = current.0 as isize - prev.0 as isize;
        let dy = current.1 as isize - prev.1 as isize;

        match (dx, dy) {
            (1, 0) => path.push_str("v"),
            (-1, 0) => path.push_str("^"),
            (0, 1) => path.push_str(">"),
            (0, -1) => path.push_str("<"),
            _ => (),
        }

        current = prev;
    }

    Some(String::from_iter(path.chars().rev()))
}

pub fn monstrous_maze(input: MonstrousMazeInput) -> MonstrousMazeOutput {
    let map = parse_input(input);
    if let Some(path) = find_path(&map) {
        debug!("Path found: {}", path);
        MonstrousMazeOutput {
            path,
        }
    } else {
        MonstrousMazeOutput {
            path: "No path found".to_string(),
        }
    }
}

