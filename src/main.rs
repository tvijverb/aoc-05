// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use itertools::Itertools;

#[derive(Debug)]
pub struct RangeMap {
    pub origin_start: i64,
    pub dest_start: i64,
    pub range_size: i64,
}

#[derive(Debug)]
pub struct SeedToSoil {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct SoilToFertilizer {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct FertilizerToWater {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct WaterToLight {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct LightToTemperature {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct TemperatureToHumidity {
    pub ranges: Vec<RangeMap>,
}

#[derive(Debug)]
pub struct HumidityToLocation {
    pub ranges: Vec<RangeMap>,
}

impl SoilToFertilizer {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl SeedToSoil {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl FertilizerToWater {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl WaterToLight {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl LightToTemperature {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl TemperatureToHumidity {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

impl HumidityToLocation {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
        }
    }

    pub fn add_rangemap(&mut self, range: RangeMap) {
        self.ranges.push(range);
    }

    pub fn add_range(&mut self, origin_start: i64, dest_start: i64, range_size: i64) {
        let range = RangeMap {
            origin_start: origin_start,
            dest_start: dest_start,
            range_size: range_size,
        };
        self.ranges.push(range);
    }

    pub fn get_dest(&self, origin: i64) -> i64 {
        let mut dest = origin;
        for range in &self.ranges {
            if origin >= range.origin_start && origin < range.origin_start + range.range_size - 1 {
                dest = range.dest_start + (origin - range.origin_start);
                break;
            }
        }
        dest
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let mut seeds = Vec::new();

    static SEEDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    // get character positions for each part number in the line
    let seed_matches = SEEDS_REGEX.find_iter(line);
    for mut chunk in &seed_matches.chunks(2) {
        // chunk.array_chunks().collect::<Vec<_>>();
        let seed_start = &chunk.next().unwrap().as_str();
        let seed_start = seed_start.parse::<i64>().unwrap();
        let range = &chunk.next().unwrap().as_str();
        let range = range.parse::<i64>().unwrap();
        for seed in seed_start..seed_start + range {
            seeds.push(seed);
        }
    }
    seeds
}

fn parse_range(line: &str) -> Option<RangeMap> {
    let mut origin_start = 0;
    let mut dest_start = 0;
    let mut range_size = 0;

    static SEED_TO_SOIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    // get character positions for each part number in the line
    let seed_to_soil_matches = SEED_TO_SOIL_REGEX.find_iter(line);

    for (idx, seed_to_soil_match) in seed_to_soil_matches.enumerate() {
        let seed_to_soil = seed_to_soil_match.as_str();
        let seed_to_soil = seed_to_soil.parse::<i64>().unwrap();
        if idx == 0 {
            dest_start = seed_to_soil;
        } else if idx == 1 {
            origin_start = seed_to_soil;
        } else if idx == 2 {
            range_size = seed_to_soil;
        }
    }

    if origin_start == 0 && dest_start == 0 && range_size == 0 {
        return None;
    }


    Some(RangeMap {
        origin_start: origin_start,
        dest_start: dest_start,
        range_size: range_size,
    })
}

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let mut reader = BufReader::new(file);
    let mut seeds;
    let mut min_location = 999999999999;
    // let mut locations = Vec::new();
    let mut seed_to_soil = SeedToSoil::new();
    let mut soil_to_fertilizer = SoilToFertilizer::new();
    let mut fertilizer_to_water = FertilizerToWater::new();
    let mut water_to_light = WaterToLight::new();
    let mut light_to_temperature = LightToTemperature::new();
    let mut temperature_to_humidity = TemperatureToHumidity::new();
    let mut humidity_to_location = HumidityToLocation::new();
    let mut is_seed_to_soil = false;
    let mut is_soil_to_fertilizer = false;
    let mut is_fertilizer_to_water = false;
    let mut is_water_to_light = false;
    let mut is_light_to_temperature = false;
    let mut is_temperature_to_humidity = false;
    let mut is_humidity_to_location = false;

    // extract seeds from first line
    let mut first_line_str = String::new();
    reader.read_line(&mut first_line_str);
    seeds = parse_seeds(&first_line_str);

    // extract maps from remaining lines
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains("seed-to-soil map") {
            is_seed_to_soil = true;
        }
        else if line.contains("soil-to-fertilizer map") {
            is_soil_to_fertilizer = true;
        }
        else if line.contains("fertilizer-to-water map") {
            is_fertilizer_to_water = true;
        }
        else if line.contains("water-to-light map") {
            is_water_to_light = true;
        }
        else if line.contains("light-to-temperature") {
            is_light_to_temperature = true;
        }
        else if line.contains("temperature-to-humidity") {
            is_temperature_to_humidity = true;
        }
        else if line.contains("humidity-to-location") {
            is_humidity_to_location = true;
        }
        if line.as_str() == "" {
            is_seed_to_soil = false;
            is_soil_to_fertilizer = false;
            is_fertilizer_to_water = false;
            is_water_to_light = false;
            is_light_to_temperature = false;
            is_temperature_to_humidity = false;
            is_humidity_to_location = false;
        }

        if (is_seed_to_soil) {
            let seed_to_soi_rangemap = parse_range(&line);
            if seed_to_soi_rangemap.is_some() {
                seed_to_soil.add_rangemap(seed_to_soi_rangemap.unwrap());
            }
        } else if is_soil_to_fertilizer {
            let soil_to_fertilizer_rangemap = parse_range(&line);
            if soil_to_fertilizer_rangemap.is_some() {
                soil_to_fertilizer.add_rangemap(soil_to_fertilizer_rangemap.unwrap());
            }
        } else if is_fertilizer_to_water {
            let fertilizer_to_water_rangemap = parse_range(&line);
            if fertilizer_to_water_rangemap.is_some() {
                fertilizer_to_water.add_rangemap(fertilizer_to_water_rangemap.unwrap());
            }
        } else if is_water_to_light {
            let water_to_light_rangemap = parse_range(&line);
            if water_to_light_rangemap.is_some() {
                water_to_light.add_rangemap(water_to_light_rangemap.unwrap());
            }
        } else if is_light_to_temperature {
            let light_to_temperature_rangemap = parse_range(&line);
            if light_to_temperature_rangemap.is_some() {
                light_to_temperature.add_rangemap(light_to_temperature_rangemap.unwrap());
            }
        } else if is_temperature_to_humidity {
            let temperature_to_humidity_rangemap = parse_range(&line);
            if temperature_to_humidity_rangemap.is_some() {
                temperature_to_humidity.add_rangemap(temperature_to_humidity_rangemap.unwrap());
            }
        } else if is_humidity_to_location {
            let humidity_to_location_rangemap = parse_range(&line);
            if humidity_to_location_rangemap.is_some() {
                humidity_to_location.add_rangemap(humidity_to_location_rangemap.unwrap());
            }
        }
    }

    // print seeds size
    println!("seeds size: {:?}", seeds.len());

    for seed in &seeds {
        let soil = seed_to_soil.get_dest(*seed);
        let fertilizer = soil_to_fertilizer.get_dest(soil);
        let water = fertilizer_to_water.get_dest(fertilizer);
        let light = water_to_light.get_dest(water);
        let temperature = light_to_temperature.get_dest(light);
        let humidity = temperature_to_humidity.get_dest(temperature);
        let location = humidity_to_location.get_dest(humidity);
        if location < min_location {
            min_location = location;
        }
    }

    println!("min location: {:?}", min_location);

    // println!("seed_to_soil: {:?}", seed_to_soil);

    Ok(())
}