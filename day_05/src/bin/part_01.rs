#[derive(Debug, PartialEq)]
struct MapLine {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}
fn main() {
    let inputs = include_str!("input.txt");
    let parsed_input: Vec<&str> = inputs.split("\n\n").collect();

    let seeds = get_seeds(parsed_input[0]);
    let seed_to_soil_map = map_values(parsed_input[1]);
    let soil_to_fertilizer_map = map_values(parsed_input[2]);
    let fertilizer_to_water_map = map_values(parsed_input[3]);
    let water_to_light_map = map_values(parsed_input[4]);
    let light_to_temparature_map = map_values(parsed_input[5]);
    let temparature_to_humidity_map = map_values(parsed_input[6]);
    let humidity_to_location_map = map_values(parsed_input[7]);

    //let mut after_map_values: Vec<u64> = Vec::new();
    let after_map_values: Vec<u32> = seeds
        .iter()
        .map(|seed| {
            let soil_mapped = map_source_to_destination(&seed_to_soil_map, *seed);
            let fertilizer_mapped = map_source_to_destination(&soil_to_fertilizer_map, soil_mapped);
            let water_mapped =
                map_source_to_destination(&fertilizer_to_water_map, fertilizer_mapped);
            let light_mapped = map_source_to_destination(&water_to_light_map, water_mapped);
            let temp_mapped = map_source_to_destination(&light_to_temparature_map, light_mapped);
            let humidity_mapped =
                map_source_to_destination(&temparature_to_humidity_map, temp_mapped);
            map_source_to_destination(&humidity_to_location_map, humidity_mapped)
        })
        .collect();
    println!("Answer");
    println!("{:?}", after_map_values.iter().min().unwrap());
}

fn map_values(input: &str) -> Vec<MapLine> {
    let part_parse: Vec<&str> = input.split(':').collect();
    let seed_parse: Vec<&str> = part_parse[1].split('\n').collect();
    let mut mappings: Vec<MapLine> = Vec::new();
    seed_parse
        .iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| mappings.push(parse_input_line(line)));

    mappings
}
fn map_source_to_destination(mappings: &Vec<MapLine>, source_number: u32) -> u32 {
    let found_mapping: Vec<_> = mappings
        .iter()
        .filter(|mapping| {
            mapping.source_range_start <= source_number as u64
                && source_number as u64 <= mapping.source_range_start + mapping.range_length
        })
        .collect();

    if found_mapping.len() == 0 {
        return source_number;
    }

    (found_mapping[0].destination_range_start
        + (source_number as u64 - found_mapping[0].source_range_start))
        .try_into()
        .unwrap()
}

fn parse_input_line(input_line: &str) -> MapLine {
    let splits: Vec<u64> = input_line
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    MapLine {
        destination_range_start: splits[0],
        source_range_start: splits[1],
        range_length: splits[2],
    }
}

fn get_seeds(input_line: &str) -> Vec<u32> {
    let part_parse: Vec<&str> = input_line.split(':').collect();

    part_parse[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
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
