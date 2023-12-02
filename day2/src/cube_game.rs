use std::collections::HashMap;

pub struct CubeGame {
    pub id: i32,
    pub cube_sets: Vec<HashMap<CubeColor, i32>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            "blue" => CubeColor::Blue,
            _ => panic!("Unrecognized color")
        }
    }
}

impl CubeGame {
    pub fn from_str(s: &str) -> Self {
        let (game_side, cube_side) = {
            let mut splitter = s.split(':');
            (splitter.next().expect("Split ':' missing left"), splitter.next().expect("Splut ':' missing right"))
        };

        let id = game_side.split(' ').nth(1).expect("Unable to find ' ' split in game side").parse::<i32>().expect("Non numeric game id");
        let cube_sections: Vec<&str> = cube_side.trim().split(';').collect();
        let cube_sets: Vec<HashMap<_, _>> = cube_sections.iter()
            .map(|s| {
                let section_split: HashMap<CubeColor, i32> = s.trim().split(',').map(|s| {
                    let mut inner_split = s.trim().split(' ');
                    let (i, c) =  (inner_split.next().unwrap().parse::<i32>().unwrap(), CubeColor::from_str(inner_split.next().unwrap()));
                    (c, i)
                }).collect();
                section_split
            })
            .collect();
        CubeGame {
            id,
            cube_sets,
        }
    }

    pub fn can_be_played_with_bag(&self, bag_content: &HashMap<CubeColor, i32>) -> bool {
        self.cube_sets.iter().all(|draw| draw.iter().all(|(color, draw_num)| bag_content.get(color).is_some_and(|bag_num| draw_num <= bag_num)))
    }

    pub fn minimum_cubes_required(&self) -> HashMap<CubeColor, i32> {
        let reds = self.cube_sets.iter().max_by_key(|cube_set| cube_set.get(&CubeColor::Red)).map(|h| h.get(&CubeColor::Red).unwrap_or(&0).to_owned());
        let greens = self.cube_sets.iter().max_by_key(|cube_set| cube_set.get(&CubeColor::Green)).map(|h| h.get(&CubeColor::Green).unwrap_or(&0).to_owned());
        let blues = self.cube_sets.iter().max_by_key(|cube_set| cube_set.get(&CubeColor::Blue)).map(|h| h.get(&CubeColor::Blue).unwrap_or(&0).to_owned());

        let mut h = HashMap::new();
        if let Some(r) = reds {
            h.insert(CubeColor::Red, r);
        }

        if let Some(g) = greens {
            h.insert(CubeColor::Green, g);
        }

        if let Some(b) = blues {
            h.insert(CubeColor::Blue, b);
        }
        
        h
    }
}