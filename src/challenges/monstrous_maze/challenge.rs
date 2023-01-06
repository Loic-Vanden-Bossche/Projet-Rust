use std::collections::VecDeque;
use crate::challenges::monstrous_maze::types::input::MonstrousMazeInput;
use crate::challenges::monstrous_maze::types::map::Map;
use crate::challenges::monstrous_maze::types::output::MonstrousMazeOutput;

fn parse_input(input: MonstrousMazeInput) -> Map {
    let mut maze = Vec::new();
    let mut player = (0, 0);
    let mut exit = (0, 0);

    for (y, line) in input.grid.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'I' => player = (y, x),
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
    let mut queue = VecDeque::new();
    queue.push_back((map.player.0, map.player.1));

    let mut visited = vec![vec![false; map.maze[0].len()]; map.maze.len()];
    visited[map.player.0][map.player.1] = true;

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];

    let mut prev = vec![vec![(0, 0); map.maze[0].len()]; map.maze.len()];

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == map.exit {
            let mut path = String::new();
            let mut curr = (x, y);
            while curr != map.player {
                let (dx, dy) = (curr.0 as isize - prev[curr.0][curr.1].0 as isize,
                                curr.1 as isize - prev[curr.0][curr.1].1 as isize);
                if dx == 1 {
                    path.push('v');
                } else if dx == -1 {
                    path.push('^');
                } else if dy == 1 {
                    path.push('>');
                } else if dy == -1 {
                    path.push('<');
                }
                curr = prev[curr.0][curr.1];
            }
            path = path.chars().rev().collect::<String>();
            return Some(path);
        }

        for i in 0..4 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];

            if nx >= 0 && ny >= 0 && nx < map.maze.len() as isize && ny < map.maze[0].len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if map.maze[nx][ny] != '#' && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    prev[nx][ny] = (x, y);
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    None
}

pub fn monstrous_maze(input: MonstrousMazeInput) -> MonstrousMazeOutput {
    let map = parse_input(input);

    if let Some(path) = find_path(&map) {
        MonstrousMazeOutput {
            path,
        }
    } else {
        MonstrousMazeOutput {
            path: "No path found".to_string(),
        }
    }
}