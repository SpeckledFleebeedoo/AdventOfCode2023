struct Galaxy {
    position: Position,
    offset: Position,
}

struct Position {
    x: i32,
    y: i32
}

fn main() {
    let file_path = "./input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in lines.iter().enumerate(){
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Galaxy{position: Position {x: x as i32, y: y as i32}, offset: Position {x: 0, y: 0}});
            }
        }
    }

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut empty_rows: Vec<i32> = Vec::new();
    let mut empty_columns: Vec<i32> = Vec::new();

    print!("Empty rows: ");
    for h in 0..height {
        let mut has_galaxies = false;
        for galaxy in &galaxies {
            if galaxy.position.y == h {
                has_galaxies = true
            }
        }
        if has_galaxies == false {
            empty_rows.push(h);
            print!("{}, ", h);
        }
    }
    println!("");

    print!("Empty cols: ");
    for w in 0..width {
        let mut has_galaxies = false;
        for galaxy in &galaxies {
            if galaxy.position.x == w {
                has_galaxies = true
            }
        }
        if has_galaxies == false {
            empty_columns.push(w);
            print!("{}, ", w);
        }
    }
    println!("");

    for e in empty_columns {
        for galaxy in &mut galaxies {
            if galaxy.position.x > e {
                // Expanding each empty column a million times means adding 999999 new columns
                galaxy.offset.x += 999999;
            }
        }
    }

    for e in empty_rows {
        for galaxy in &mut galaxies {
            if galaxy.position.y > e {
                galaxy.offset.y += 999999;
            }
        }
    }

    for g in &mut galaxies {
        g.position.x = g.position.x + g.offset.x;
        g.position.y = g.position.y + g.offset.y;
    }

    let mut total_distance: u64 = 0;
    for g1 in 0..galaxies.len() {
        for g2 in g1+1..galaxies.len() {
            let gal1 = &galaxies[g1];
            let gal2 = &galaxies[g2];
            let distance = (gal1.position.x - gal2.position.x).abs() + (gal1.position.y - gal2.position.y).abs();
            // println!("Galaxy {} ({}, {}) to Galaxy {} ({}, {}), Distance: {}", gal1.number, gal1.position.x, gal1.position.y, gal2.number, gal2.position.x, gal2.position.y, distance);
            total_distance += distance as u64;
        }
    }

    println!("Total distance: {}", total_distance)
}