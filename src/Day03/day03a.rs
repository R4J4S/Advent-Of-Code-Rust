#[derive(PartialEq)]
pub enum ValueType {
    Number(u32),
    Symbol,
    Dot,
}

pub struct GridPointData {
    pub id: i32,
    pub value_type: ValueType,
}

pub fn solve(input: String) -> i32 {
    let grid = Grid::new(input);
    let grid_data_map = grid_to_hashmap(&grid);
    return sum_of_all_adjacent_numbers(&grid, &grid_data_map);
}

pub fn grid_to_hashmap(grid: &Grid) -> HashMap<(usize, usize), GridPointData> {
    let mut resultant_hash_map: HashMap<(usize, usize), GridPointData> = HashMap::new();
    let mut id_tracker = 0;

    for y in 0..grid.height {
        let mut visited_indexes: HashSet<usize> = HashSet::new();

        for x in 0..grid.width {
            if visited_indexes.contains(&x) {
                continue;
            }

            let map_char = grid.map[y][x];

            if map_char.is_digit(10) {
                //Create the corresponding number from digits
                let mut digits: Vec<u32> = vec![];
                let mut x_index = x;
                let mut ch = map_char;
                while ch.is_digit(10) {
                    let n = ch.to_digit(10).unwrap();
                    digits.push(n);

                    x_index += 1;

                    if x_index >= grid.width {
                        break;
                    }

                    ch = grid.map[y][x_index];
                }

                let number = digits.iter().fold(0, |acc, digit| (acc * 10 + digit));
                //println!("Number created: {}", &number);

                //Assign to the hashmap
                for (i, _digit) in digits.iter().enumerate() {
                    resultant_hash_map.insert(
                        (x + i, y),
                        GridPointData {
                            id: id_tracker,
                            value_type: ValueType::Number(number),
                        },
                    );
                    visited_indexes.insert(x + i);

                    println!("({},{}) : {} , ID: {}", x + i, y, number, id_tracker);
                }
            } else {
                resultant_hash_map.insert(
                    (x, y),
                    GridPointData {
                        id: id_tracker,
                        value_type: {
                            if map_char == '.' {
                                ValueType::Dot
                            } else {
                                ValueType::Symbol
                            }
                        },
                    },
                );

                visited_indexes.insert(x);

                println!("({},{}), ID: {}", x, y, id_tracker);
            }

            id_tracker += 1;
        }
    }

    resultant_hash_map
}

pub fn sum_of_all_adjacent_numbers(
    grid: &Grid,
    grid_data_map: &HashMap<(usize, usize), GridPointData>,
) -> i32 {
    let neighbouring_indexes: Vec<(i32, i32)> = vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut required_sum = 0;
    let mut used_ids: HashSet<i32> = HashSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let grid_point_data = grid_data_map.get(&(x, y)).unwrap();

            match grid_point_data.value_type {
                ValueType::Symbol => {
                    for neighbour in &neighbouring_indexes {
                        let x2 = x as i32 + neighbour.0;
                        let y2 = y as i32 + neighbour.1;

                        if x2 >= grid.width as i32 || y2 >= grid.height as i32 || x2 < 0 || y2 < 0 {
                            break;
                        }

                        let point = grid_data_map.get(&(x2 as usize, y2 as usize)).unwrap();

                        if used_ids.contains(&point.id) {
                            continue;
                        }

                        if let ValueType::Number(num) = point.value_type {
                            required_sum += num;
                        }

                        used_ids.insert(point.id);
                    }
                }
                _ => (),
            }
        }
    }

    required_sum as i32
}
