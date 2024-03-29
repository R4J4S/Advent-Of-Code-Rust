mod utils;

use day03::day03a;
use utils::grid::Grid;

pub fn solve(input: String) -> i32 {
    let grid = Grid::new(input);
    let grid_data_map = day03a::grid_to_hashmap(&grid);
    return sum_of_all_gear_ratios(&grid, &grid_data_map);
}

pub fn sum_of_all_gear_ratios(
    grid: &Grid,
    grid_data_map: &HashMap<(usize, usize), day03a::GridPointData>,
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
            let grid_data_point = grid_data_map.get(&(x, y)).unwrap();

            match grid_data_point.value_type {
                day03a::ValueType::Symbol(symbol) => {
                    if symbol == '*' {
                        let mut neighbouring_numbers = vec![];
                        for neighbour in &neighbouring_indexes {
                            let x2 = x as i32 + neighbour.0;
                            let y2 = y as i32 + neighbour.1;

                            if x2 >= grid.width as i32
                                || y2 >= grid.height as i32
                                || x2 < 0
                                || y2 < 0
                            {
                                break;
                            }

                            let point = grid_data_map.get(&(x2 as usize, y2 as usize)).unwrap();

                            if used_ids.contains(&point.id) {
                                continue;
                            }

                            if let day03a::ValueType::Number(num) = point.value_type {
                                neighbouring_numbers.push(num);
                            }

                            used_ids.insert(point.id);
                        }

                        if neighbouring_numbers.len() == 2 {
                            required_sum +=
                                neighbouring_numbers.iter().fold(1, |acc, num| (acc * num));
                        }
                    }
                }
                _ => (),
            }
        }
    }

    required_sum as i32
}
