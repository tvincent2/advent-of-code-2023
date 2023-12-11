use std::ops::{Deref, Range};

#[derive(Debug, PartialEq)]
struct SeedMap {
    destination: Range<usize>,
    source: Range<usize>,
}

impl From<&str> for SeedMap {
    fn from(input: &str) -> Self {
        let numbers: Vec<usize> = input
            .split(" ")
            .map(|s| s.parse::<usize>().expect("unparseable number"))
            .collect();
        let destination_start = numbers[0];
        let source_start = numbers[1];
        let length = numbers[2];
        SeedMap {
            destination: destination_start..destination_start + length,
            source: source_start..source_start + length,
        }
    }
}

#[derive(Debug, PartialEq)]
struct SeedMaps {
    maps: Vec<SeedMap>,
}

impl SeedMaps {
    fn next_step(&self, step: usize) -> usize {
        match self.iter().find(|map| map.source.contains(&step)) {
            Some(map) => {
                let offset = step - map.source.start;
                map.destination.start + offset
            }
            None => step,
        }
    }
}

impl Deref for SeedMaps {
    type Target = Vec<SeedMap>;

    fn deref(&self) -> &Self::Target {
        &self.maps
    }
}

#[derive(Debug, PartialEq)]
struct Almanach {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range<usize>>,
    seed_to_soil: SeedMaps,
    soil_to_fertilizer: SeedMaps,
    fertilizer_to_water: SeedMaps,
    water_to_light: SeedMaps,
    light_to_temperature: SeedMaps,
    temperature_to_humidity: SeedMaps,
    humidity_to_location: SeedMaps,
}

impl Almanach {
    fn compute_seed_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.next_step(seed);
        let fertilizer = self.soil_to_fertilizer.next_step(soil);
        let water = self.fertilizer_to_water.next_step(fertilizer);
        let light = self.water_to_light.next_step(water);
        let temperature = self.light_to_temperature.next_step(light);
        let humidity = self.temperature_to_humidity.next_step(temperature);
        let location = self.humidity_to_location.next_step(humidity);
        location
    }

    fn find_lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.compute_seed_location(*seed))
            .min()
            .expect("there should be a min location")
    }

    fn find_lowest_location_from_ranges(&self) -> usize {
        self.seed_ranges
            .iter()
            .inspect(|range| println!("about to handle {:?}", range))
            .flat_map(|range| range.clone().map(|seed| self.compute_seed_location(seed)))
            .min()
            .expect("there should be a min location")
    }
}

impl From<&str> for Almanach {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds: Vec<usize> = lines
            .next()
            .expect("missing seeds line")
            .split(" ")
            .skip(1)
            .map(|s| s.parse::<usize>().expect("unparseable number"))
            .collect();
        let seed_ranges = seeds
            .chunks_exact(2)
            .map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .collect();
        // skip the next two lines because the first one is empty
        let seed_to_soil_vec = lines
            .by_ref()
            .skip(2)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let seed_to_soil = SeedMaps {
            maps: seed_to_soil_vec,
        };
        // from now on, we can skip only one line because the empty one has been consumed by take_while
        let soil_to_fertilizer_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let soil_to_fertilizer = SeedMaps {
            maps: soil_to_fertilizer_vec,
        };
        let fertilizer_to_water_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let fertilizer_to_water = SeedMaps {
            maps: fertilizer_to_water_vec,
        };
        let water_to_light_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let water_to_light = SeedMaps {
            maps: water_to_light_vec,
        };
        let light_to_temperature_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let light_to_temperature = SeedMaps {
            maps: light_to_temperature_vec,
        };
        let temperature_to_humidity_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let temperature_to_humidity = SeedMaps {
            maps: temperature_to_humidity_vec,
        };
        let humidity_to_location_vec = lines
            .by_ref()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| SeedMap::from(line))
            .collect();
        let humidity_to_location = SeedMaps {
            maps: humidity_to_location_vec,
        };
        Almanach {
            seeds,
            seed_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }
}

fn main() {
    let input = include_str!("../../input/day-05");
    let almanach = Almanach::from(input);
    let lowest_location = almanach.find_lowest_location();
    println!("Lowest location: {}", lowest_location);

    let lowest_location_from_ranges = almanach.find_lowest_location_from_ranges();
    println!(
        "Lowest location from ranges: {}",
        lowest_location_from_ranges
    );
}

#[cfg(test)]
mod tests {
    use crate::{Almanach, SeedMap, SeedMaps};

    #[test]
    fn parse_seed_map() {
        let input = "50 98 2";
        let source_map = SeedMap::from(input);
        assert_eq!(
            source_map,
            SeedMap {
                destination: 50..52,
                source: 98..100,
            }
        );
    }

    #[test]
    fn parse_almanach() {
        let input = include_str!("../../input/day-05-test");
        let almanach = Almanach::from(input);
        assert_eq!(
            almanach,
            Almanach {
                seeds: vec![79, 14, 55, 13],
                seed_ranges: vec![79..93, 55..68],
                seed_to_soil: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 50..52,
                            source: 98..100
                        },
                        SeedMap {
                            destination: 52..100,
                            source: 50..98
                        }
                    ]
                },
                soil_to_fertilizer: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 0..37,
                            source: 15..52
                        },
                        SeedMap {
                            destination: 37..39,
                            source: 52..54
                        },
                        SeedMap {
                            destination: 39..54,
                            source: 0..15
                        }
                    ]
                },
                fertilizer_to_water: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 49..57,
                            source: 53..61
                        },
                        SeedMap {
                            destination: 0..42,
                            source: 11..53
                        },
                        SeedMap {
                            destination: 42..49,
                            source: 0..7
                        },
                        SeedMap {
                            destination: 57..61,
                            source: 7..11
                        }
                    ]
                },
                water_to_light: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 88..95,
                            source: 18..25
                        },
                        SeedMap {
                            destination: 18..88,
                            source: 25..95
                        }
                    ]
                },
                light_to_temperature: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 45..68,
                            source: 77..100
                        },
                        SeedMap {
                            destination: 81..100,
                            source: 45..64
                        },
                        SeedMap {
                            destination: 68..81,
                            source: 64..77
                        }
                    ]
                },
                temperature_to_humidity: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 0..1,
                            source: 69..70
                        },
                        SeedMap {
                            destination: 1..70,
                            source: 0..69
                        }
                    ]
                },
                humidity_to_location: SeedMaps {
                    maps: vec![
                        SeedMap {
                            destination: 60..97,
                            source: 56..93
                        },
                        SeedMap {
                            destination: 56..60,
                            source: 93..97
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn map_next_step() {
        let maps = SeedMaps {
            maps: vec![
                SeedMap {
                    destination: 50..52,
                    source: 98..100,
                },
                SeedMap {
                    destination: 52..100,
                    source: 50..98,
                },
            ],
        };

        assert_eq!(maps.next_step(79), 81);
        assert_eq!(maps.next_step(14), 14);
        assert_eq!(maps.next_step(55), 57);
        assert_eq!(maps.next_step(13), 13);
    }

    #[test]
    fn location() {
        let input = include_str!("../../input/day-05-test");
        let almanach = Almanach::from(input);

        assert_eq!(almanach.compute_seed_location(79), 82);
        assert_eq!(almanach.compute_seed_location(14), 43);
        assert_eq!(almanach.compute_seed_location(55), 86);
        assert_eq!(almanach.compute_seed_location(13), 35);
    }

    #[test]
    fn lowest_location() {
        let input = include_str!("../../input/day-05-test");
        let almanach = Almanach::from(input);

        assert_eq!(almanach.find_lowest_location(), 35);
    }

    #[test]
    fn lowest_location_with_ranges() {
        let input = include_str!("../../input/day-05-test");
        let almanach = Almanach::from(input);

        assert_eq!(almanach.find_lowest_location_from_ranges(), 46);
    }
}
