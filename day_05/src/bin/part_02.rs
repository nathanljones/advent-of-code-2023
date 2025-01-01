#[derive(Debug, PartialEq)]
struct MapLine {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}
fn main() {
    let inputs = include_str!("input.txt");
    let parsed_input: Vec<&str> = inputs.split("\n\n").collect();
    let mut count: u32 = 0;
    /*println!("{:?}", parsed_input[1]);
    println!("{:?}", parsed_input[2]);
    println!("{:?}", parsed_input[3]);
    println!("{:?}", parsed_input[4]);
    println!("{:?}", parsed_input[5]);
    println!("{:?}", parsed_input[6]);
    println!("{:?}", parsed_input[7]);*/

    let seeds = get_seeds(parsed_input[0]);
    //println!("{:?}", seeds);
    let seed_to_soil_map = map_values(parsed_input[1]);
    let soil_to_fertilizer_map = map_values(parsed_input[2]);
    let fertilizer_to_water_map = map_values(parsed_input[3]);
    let water_to_light_map = map_values(parsed_input[4]);
    let light_to_temparature_map = map_values(parsed_input[5]);
    let temparature_to_humidity_map = map_values(parsed_input[6]);
    let humidity_to_location_map = map_values(parsed_input[7]);

    let mut after_map_values: Vec<u32> = Vec::new();
    let seed_length = seeds.len();
    for seed in seeds {
        println! {"Checking seed {} of {} count is {}",seed,seed_length, count}
        let soil_mapped = map_source_to_destination(&seed_to_soil_map, seed);
        let fertilizer_mapped = map_source_to_destination(&soil_to_fertilizer_map, soil_mapped);
        let water_mapped = map_source_to_destination(&fertilizer_to_water_map, fertilizer_mapped);
        let light_mapped = map_source_to_destination(&water_to_light_map, water_mapped);
        let temp_mapped = map_source_to_destination(&light_to_temparature_map, light_mapped);
        let humidity_mapped = map_source_to_destination(&temparature_to_humidity_map, temp_mapped);
        after_map_values.push(map_source_to_destination(
            &humidity_to_location_map,
            humidity_mapped,
        ));
        count += 1;
    }
    //println!("{:?}", after_map_values);
    println!("Answer");
    println!("{:?}", after_map_values.iter().min());
}

fn map_values(input: &str) -> Vec<MapLine> {
    let part_parse: Vec<&str> = input.split(':').collect();
    let seed_parse: Vec<&str> = part_parse[1].split('\n').collect();
    let mut mappings: Vec<MapLine> = Vec::new();
    for line in seed_parse {
        if !line.is_empty() {
            mappings.push(parse_input_line(line));
        };
    }
    mappings
}
fn map_source_to_destination(mappings: &Vec<MapLine>, source_number: u32) -> u32 {
    let mut stored_range_start: u32 = 0;
    let mut found: bool = false;
    let mut ret_val: u32 = 0;
    for map_line in mappings {
        if map_line.source_range_start >= stored_range_start
            && source_number >= map_line.source_range_start
        {
            stored_range_start = map_line.source_range_start;
            found = true;
        }
    }

    if !found {
        ret_val = source_number;
    } else {
        for map_line in mappings {
            if map_line.source_range_start == stored_range_start {
                let adjusted_source_number = source_number - map_line.source_range_start;
                if adjusted_source_number > map_line.range_length {
                    ret_val = source_number;
                } else {
                    ret_val = adjusted_source_number + map_line.destination_range_start;
                }
            }
        }
    }
    ret_val
}

fn parse_input_line(input_line: &str) -> MapLine {
    let splits: Vec<u32> = input_line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    MapLine {
        destination_range_start: splits[0],
        source_range_start: splits[1],
        range_length: splits[2],
    }
}

fn get_seeds(input_line: &str) -> Vec<u32> {
    let part_parse: Vec<&str> = input_line.split(':').collect();
    let mut parsed_numbers: Vec<u32> = part_parse[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut ret_value: Vec<u32> = Vec::new();

    while !parsed_numbers.is_empty() {
        let start: u32 = parsed_numbers[0];
        let no_times: u32 = parsed_numbers[1];
        for count in 0..no_times {
            ret_value.push(count + start);
        }
        parsed_numbers.remove(0);
        parsed_numbers.remove(0);
    }

    ret_value
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_mapping() {
        let mut mappings: Vec<MapLine> = Vec::new();
        mappings.push(MapLine {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        });
        mappings.push(MapLine {
            destination_range_start: 52,
            source_range_start: 50,
            range_length: 48,
        });
        mappings.push(MapLine {
            destination_range_start: 0,
            source_range_start: 5,
            range_length: 5,
        });

        assert_eq!(map_source_to_destination(&mappings, 79), 81);
        assert_eq!(map_source_to_destination(&mappings, 14), 14);
        assert_eq!(map_source_to_destination(&mappings, 55), 57);
        assert_eq!(map_source_to_destination(&mappings, 13), 13);
        assert_eq!(map_source_to_destination(&mappings, 130), 130);
        assert_eq!(map_source_to_destination(&mappings, 6), 1);

        /*Seed number 79 corresponds to soil number 81.
        Seed number 14 corresponds to soil number 14.
        Seed number 55 corresponds to soil number 57.
        Seed number 13 corresponds to soil number 13.*/
    }
    #[test]
    fn test_parse_input_line() {
        let input_line = "375094277 737335351 103046502";
        let mapped_line = MapLine {
            destination_range_start: 375094277,
            source_range_start: 737335351,
            range_length: 103046502,
        };

        assert_eq!(parse_input_line(input_line), mapped_line);
    }
    #[test]
    fn test_seeds() {
        let input = "seeds: 41218238 421491713 1255413673 350530906 944138913 251104806 481818804 233571979 2906248740 266447632 3454130719 50644329 1920342932 127779721 2109326496 538709762 3579244700 267233350 4173137165 60179884";
        assert_eq!(get_seeds(input)[2], 1255413673);
        assert_eq!(get_seeds(input)[3], 350530906);
    }
}
