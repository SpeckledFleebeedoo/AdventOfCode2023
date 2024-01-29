use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    EastSouth,
    SouthWest,
    WestNorth,
    Ground,
    Start
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32
}

fn create_grid() -> (Vec<Vec<PipeType>>, Position) {
    let pipe_hash = HashMap::from([
        ('|', PipeType::Vertical),
        ('-', PipeType::Horizontal),
        ('L', PipeType::NorthEast),
        ('J', PipeType::WestNorth),
        ('7', PipeType::SouthWest),
        ('F', PipeType::EastSouth),
        ('.', PipeType::Ground),
        ('S', PipeType::Start)
    ]);

    let mut grid:  Vec<Vec<PipeType>> = Vec::new();
    let mut start_position = Position{x: 0, y: 0};

    let file_path = "./input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        let mut line_vec = Vec::new();
        for char in line.chars().collect::<Vec<_>>() {
            let pipe_type = pipe_hash[&char];
            line_vec.push(pipe_type);
            if pipe_type == PipeType::Start {
                start_position = Position{x: x, y: y};
            }
            x += 1;
        }
        grid.push(line_vec);
        y += 1;
    };
    return (grid, start_position)
}

fn find_connections(grid: &Vec<Vec<PipeType>>, pos: &Position) -> Vec<Position> {

    let surrounding_tiles: Vec<Position> = match grid[pos.y as usize][pos.x as usize] {
        PipeType::Vertical => return vec![Position{y: pos.y+1, x: pos.x}, Position{y: pos.y-1, x: pos.x}],
        PipeType::Horizontal => return vec![Position{y: pos.y, x: pos.x+1}, Position{y: pos.y, x: pos.x-1}],
        PipeType::NorthEast => return vec![Position{y: pos.y-1, x: pos.x}, Position{y: pos.y, x: pos.x+1}],
        PipeType::EastSouth => return vec![Position{y: pos.y+1, x: pos.x}, Position{y: pos.y, x: pos.x+1}],
        PipeType::SouthWest => return vec![Position{y: pos.y+1, x: pos.x}, Position{y: pos.y, x: pos.x-1}],
        PipeType::WestNorth => return vec![Position{y: pos.y-1, x: pos.x}, Position{y: pos.y, x: pos.x-1}],
        PipeType::Ground => panic!("Looking for connections to ground at {}, {}", pos.y, pos.x),
        PipeType::Start => vec![Position{y: pos.y-1, x: pos.x}, Position{y: pos.y, x: pos.x+1}, Position{y: pos.y+1, x: pos.x}, Position{y: pos.y, x: pos.x-1}],
    };

    let mut connecting_tiles: Vec<Position> = Vec::new();

    match grid[surrounding_tiles[0].y as usize][surrounding_tiles[0].x as usize] { // north
        PipeType::EastSouth => connecting_tiles.push(surrounding_tiles[0]),
        PipeType::SouthWest => connecting_tiles.push(surrounding_tiles[0]),
        PipeType::Vertical => connecting_tiles.push(surrounding_tiles[0]),
        _ => (),
    };
    match grid[surrounding_tiles[1].y as usize][surrounding_tiles[1].x as usize] { // east
        PipeType::WestNorth => connecting_tiles.push(surrounding_tiles[1]),
        PipeType::SouthWest => connecting_tiles.push(surrounding_tiles[1]),
        PipeType::Horizontal => connecting_tiles.push(surrounding_tiles[1]),
        _ => (),
    };
    match grid[surrounding_tiles[2].y as usize][surrounding_tiles[2].x as usize] { // south
        PipeType::NorthEast => connecting_tiles.push(surrounding_tiles[2]),
        PipeType::WestNorth => connecting_tiles.push(surrounding_tiles[2]),
        PipeType::Vertical => connecting_tiles.push(surrounding_tiles[2]),
        _ => (),
    };
    match grid[surrounding_tiles[3].y as usize][surrounding_tiles[3].x as usize] { // west
        PipeType::EastSouth => connecting_tiles.push(surrounding_tiles[3]),
        PipeType::NorthEast => connecting_tiles.push(surrounding_tiles[3]),
        PipeType::Horizontal => connecting_tiles.push(surrounding_tiles[3]),
        _ => (),
    };

    return connecting_tiles;

    
}

fn main() {
    let (grid, start_position) = create_grid();

    let connections = find_connections(&grid, &start_position);
    let mut visited_cells: Vec<Position> = Vec::new();
    visited_cells.push(start_position);

    let mut current_cell = connections[0];

    let mut highest_distance = 1; // Accounting for first step in line above

    loop {
        visited_cells.push(current_cell);
        highest_distance += 1;
        let connections = find_connections(&grid, &current_cell);
        if visited_cells.contains(&connections[0]) {
            current_cell = connections[1]
        } else {
            current_cell = connections[0]
        };

        if grid[current_cell.y as usize][current_cell.x as usize] == PipeType::Start {
            break;
        }
        
    }
    println!("{}", highest_distance/2);
}