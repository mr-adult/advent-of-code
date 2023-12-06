use std::collections::HashSet;
use std::fmt::Display;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let (seeds, maps) = process_input(data);

    return seeds
        .into_iter()
        .map(|seed| {
            let location = maps.iter().enumerate().fold(seed, |acc, (map_num, map)| {
                let mut found = false;
                map.iter().fold(acc, |acc2, line| {
                    if !found && line[1] <= acc2 && line[1] + line[2] > acc2 {
                        found = true;
                        line[0] - line[1] + acc2
                    } else {
                        acc2
                    }
                })
            });

            location
        })
        .min()
        .unwrap_or(0);
}

fn part2(data: &str) -> impl Display {
    let (seeds, maps) = process_input(data);

    let mut current_ranges = seeds
        .chunks(2)
        .map(|range| [range[0], range[1]])
        .collect::<HashSet<_>>();

    for map in maps {
        println!("");
        println!("Next round");
        println!("");
        let mut new_ranges = HashSet::new();

        for range in current_ranges {
            let mut sorted_lines = map.iter().collect::<Vec<_>>();
            sorted_lines.sort_by(|line1, line2| line1[1].cmp(&line2[1]));

            let mut last_line = Vec::new();
            for (line_num, line) in sorted_lines.into_iter().enumerate() {
                if line_num == 0 && range[0] < line[1] {
                    println!("Line: {:?}", line);
                    println!(
                        "Kept {:?}",
                        [
                            range[0],
                            [line[1] - range[0], range[1]].into_iter().min().unwrap()
                        ]
                    );
                    new_ranges.insert([
                        range[0],
                        [line[1] - range[0], range[1]].into_iter().min().unwrap(),
                    ]);
                }

                if line[1] < range[0] + range[1] && range[0] < line[1] + line[2] {
                    let lower_bound =
                        [line[1], range[0]].into_iter().max().unwrap() + line[0] - line[1];

                    let new_range = [
                        lower_bound,
                        [range[0] + range[1], line[1] + line[2]]
                            .into_iter()
                            .min()
                            .unwrap()
                            + line[0]
                            - line[1]
                            - lower_bound,
                    ];

                    println!(
                        "from: {:?} with range: {:?} -> result: {:?}",
                        range, line, new_range
                    );
                    new_ranges.insert(new_range);
                }

                last_line = line.clone();
            }

            if last_line[1] + last_line[2] < range[0] + range[1] {
                println!(
                    "Kept last line. Line: {:?}, Range: {:?}, Result: {:?}",
                    last_line,
                    range,
                    [
                        last_line[1] + last_line[2],
                        range[0] + range[1] - (last_line[1] + last_line[2]),
                    ]
                );
                new_ranges.insert([
                    last_line[1] + last_line[2],
                    range[0] + range[1] - (last_line[1] + last_line[2]),
                ]);
            }
        }

        current_ranges = new_ranges;
    }

    current_ranges
        .into_iter()
        .map(|range| range[0])
        .min()
        .unwrap()
}

fn process_input(input: &str) -> (Vec<isize>, Vec<Vec<Vec<isize>>>) {
    let sections = input.split("\n\n");
    let mut sections = sections.into_iter();

    // special case: first section is seeds
    let seeds = sections.next().unwrap();
    let seeds = seeds.split(' ').collect::<Vec<_>>()[1..] // first is "seeds:"
        .into_iter()
        .map(|seed| seed.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let results = sections
        .into_iter()
        .map(|section| {
            section
                .lines()
                .enumerate()
                .filter(|(i, _)| *i != 0)
                .map(|(_, line)| {
                    line.split(' ')
                        .into_iter()
                        .map(|val| val.parse::<isize>().expect("val to be valid isize"))
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();

    (seeds, results)
}
