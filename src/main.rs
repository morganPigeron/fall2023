use itertools::Itertools;
use std::{cmp::Ordering, f64::consts::PI, io, ops::Mul};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn calculate_angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (y2 - y1).atan2(x2 - x1)
}

#[derive(PartialEq, Clone, Copy)]
enum CreatureType {
    Monster = -1,
    Type0 = 0,
    Type1 = 1,
    Type2 = 2,
}

impl Into<CreatureType> for i32 {
    fn into(self) -> CreatureType {
        match self {
            -1 => CreatureType::Monster,
            0 => CreatureType::Type0,
            1 => CreatureType::Type1,
            2 => CreatureType::Type2,
            _ => todo!(),
        }
    }
}

struct Creature {
    id: i32,
    color: i32,
    creature_type: CreatureType,
}

struct VisibleCreature {
    creature_id: i32,
    creature_x: i32,
    creature_y: i32,
    creature_vx: i32,
    creature_vy: i32,
}

impl VisibleCreature {
    fn is_a_monster(&self, creatures: &Vec<Creature>) -> bool {
        if let Some(creature) = creatures.iter().find(|c| c.id == self.creature_id) {
            return creature.creature_type == CreatureType::Monster;
        }
        return false;
    }
}

#[derive(Clone, Copy)]
struct Drone {
    drone_id: i32,
    drone_x: i32,
    drone_y: i32,
    emergency: i32,
    battery: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Scan {
    drone_id: i32,
    creature_id: i32,
}

#[derive(Debug, Clone)]
enum Radar {
    TL,
    TR,
    BL,
    BR,
}

impl From<&str> for Radar {
    fn from(value: &str) -> Self {
        match value {
            "TL" => Radar::TL,
            "TR" => Radar::TR,
            "BL" => Radar::BL,
            "BR" => Radar::BR,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Blip {
    drone_id: i32,
    creature_id: i32,
    radar: Radar,
}

struct GameState {
    my_score: i32,
    foe_score: i32,
    my_scan: Vec<i32>,
    foe_scan: Vec<i32>,
    my_drone: Vec<Drone>,
    foe_drone: Vec<Drone>,
    drone_scan: Vec<Scan>,
    visible_creature: Vec<VisibleCreature>,
    radar_blip: Vec<Blip>,
}

#[derive(Clone)]
struct DroneBehavior {
    id: i32,
    drone_state: Drone,
    saved: bool,
    will_save: bool,
    direction: i32,
    debounce: i32,
    blip: Vec<Blip>,
    log: String,
    is_escaping: bool,
    escape_pos: (i32, i32),
}

impl DroneBehavior {
    fn new(state: Drone, id: i32) -> Self {
        return Self {
            id,
            drone_state: state.clone(),
            will_save: false,
            saved: false,
            direction: 1,
            debounce: 10,
            blip: Vec::new(),
            log: String::new(),
            is_escaping: false,
            escape_pos: (0, 0),
        };
    }

    fn x_i32(&self) -> i32 {
        self.drone_state.drone_x
    }

    fn y_i32(&self) -> i32 {
        self.drone_state.drone_y
    }

    fn x_f64(&self) -> f64 {
        self.drone_state.drone_x as f64
    }

    fn y_f64(&self) -> f64 {
        self.drone_state.drone_y as f64
    }

    fn do_i_see_it(&self, creature: &VisibleCreature) -> bool {
        let vx = creature.creature_x - self.x_i32();
        let vy = creature.creature_y - self.y_i32();
        let dist = ((vx * vx + vy * vy) as f64).sqrt();
        eprintln!("dist {} with id {}", dist, creature.creature_id);
        dist <= 2_000.0
    }

    fn set_my_blip(&mut self, game_state: &GameState) {
        self.blip = game_state
            .radar_blip
            .iter()
            .filter(|b| b.drone_id == self.drone_state.drone_id)
            .cloned()
            .collect();
    }

    fn behave(&mut self, game_state: &GameState, creatures: &Vec<Creature>) {
        self.log = "".to_string();
        self.set_my_blip(game_state);

        // basic sinus movement
        let a = 3_000f64; //amplitude
        let b = 2f64 * PI / 5_000f64; //period
        let k = 6_500f64; //axe

        if self.x_i32() + 600 > 10000 {
            self.direction = -1;
        } else if self.x_i32() - 600 < 0 {
            self.direction = 1;
        }

        let mut next_x: f64 = self.y_f64() + (600f64 * self.direction as f64);
        let mut y: f64;
        if self.direction < 0 {
            y = (-1.0 * a) * (b * next_x).sin() + k;
        } else {
            y = a * (b * next_x).sin() + k;
        }
        // basic sinus movement end

        //move toward mvp
        let mvp = game_state.get_mvp(creatures);
        eprintln!("{:?}", mvp);
        if let Some(fish_id) = mvp.get(self.id as usize) {
            if let Some(result) = self.blip.iter().find(|b| b.creature_id == *fish_id) {
                match result.radar {
                    Radar::TL => {
                        next_x = (self.x_i32() - 600).into();
                        y = (self.y_i32() - 600).into();
                    }
                    Radar::TR => {
                        next_x = (self.x_i32() + 600).into();
                        y = (self.y_i32() - 600).into();
                    }
                    Radar::BL => {
                        next_x = (self.x_i32() - 600).into();
                        y = (self.y_i32() + 600).into();
                    }
                    Radar::BR => {
                        next_x = (self.x_i32() + 600).into();
                        y = (self.y_i32() + 600).into();
                    }
                }
            }
        }

        //check if any visible creature is a monster
        let mut near_monsters = Vec::<&VisibleCreature>::new();
        for creature in &game_state.visible_creature {
            if creature.is_a_monster(creatures) {
                if !self.do_i_see_it(creature) {
                    continue;
                }
                near_monsters.push(creature);

                self.log = self.log.clone() + " " + &format!("{}!", creature.creature_id);

                let monster_to_me: (f64, f64) = (
                    (creature.creature_x - self.x_i32()).mul(2).into(),
                    (creature.creature_y - self.y_i32()).mul(2).into(),
                );

                let escape_pos: (f64, f64) = (
                    self.x_f64() - monster_to_me.0,
                    self.y_f64() - monster_to_me.1,
                );

                next_x = (escape_pos.0).clamp(500.0, 9_500.0);
                y = (escape_pos.1).clamp(500.0, 9_500.0);

                self.is_escaping = true;
                self.escape_pos = (next_x as i32, y as i32);

                self.debounce -= 5;
            }
        }

        //check if i need to flash
        let mut light = 0;
        self.debounce += 1;
        if self.drone_state.battery > 5
            && self.y_i32() > 3_000
            && self.debounce > 10
            && !self.is_escaping
        {
            light = 1;
            self.debounce = 0;
            self.log = self.log.clone() + " flash!";
        }

        //check if we escaped
        if self.is_escaping {
            self.log = self.log.clone()
                + " escaping to "
                + &format!("{} {}", self.escape_pos.0, self.escape_pos.1);
            next_x = self.escape_pos.0.into();
            y = self.escape_pos.1.into();
        }
        if self.x_i32() == self.escape_pos.0 && self.y_i32() == self.escape_pos.1 {
            self.is_escaping = false;
        }

        //check if i need to save actual creatures
        if game_state.do_i_need_to_save(creatures) || self.will_save {
            self.will_save = true;
            if self.y_i32() < 500 {
                self.will_save = false;
            }
            self.log = self.log.clone() + " save!";
            println!("MOVE {} {} {} {}", next_x.round() as i32, 450, 0, self.log);
        } else {
            self.log = self.log.clone() + " searching";
            println!(
                "MOVE {} {} {} {}",
                next_x.round() as i32,
                y.round() as i32,
                light,
                self.log
            );
        }

        eprintln!(
            "id:{};will_save:{};direction:{};debounce:{},blip:{:?}",
            self.drone_state.drone_id, self.will_save, self.direction, self.debounce, self.blip
        );
    }
}

fn get_drone_by_id<'a>(id: i32, drones: &'a mut Vec<DroneBehavior>) -> &'a mut DroneBehavior {
    drones
        .iter_mut()
        .find(|d| d.drone_state.drone_id == id)
        .expect("id given by codingame")
}

impl GameState {
    fn get_my_drone_count(&self) -> usize {
        return self.my_drone.len();
    }

    fn get_drone_scan(&self, drone_id: i32) -> Vec<Scan> {
        return self
            .drone_scan
            .iter()
            .filter(|s| s.drone_id == drone_id)
            .cloned()
            .collect();
    }

    fn get_my_drones_scan(&self) -> Vec<Scan> {
        self.drone_scan
            .iter()
            .filter(|s| {
                self.my_drone
                    .iter()
                    .filter(|d| d.drone_id == s.drone_id)
                    .count()
                    > 0
            })
            .cloned()
            .collect()
    }

    fn is_four_of_a_kind(&self, fishes: &Vec<Scan>, creatures: &Vec<Creature>) -> bool {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        let mut scanned: Vec<i32> = fishes.iter().map(|f| f.creature_id).collect();
        scanned.sort();
        scanned.dedup();

        for fish in &scanned {
            let creature = creatures.iter().find(|f| f.id == *fish).expect("lol");
            match &creature.creature_type {
                CreatureType::Type0 => a += 1,
                CreatureType::Type1 => b += 1,
                CreatureType::Type2 => c += 1,
                _ => (),
            }
        }

        assert!(a <= 4 && b <= 4 && c <= 4, "fishes {:#?}", scanned);

        if a >= 4 || b >= 4 || c >= 4 {
            return true;
        }
        return false;
    }

    fn do_i_need_to_save(&self, creatures: &Vec<Creature>) -> bool {
        //if I have all fish from a type
        let my_fishes = self.get_my_drones_scan();
        if self.is_four_of_a_kind(&my_fishes, creatures) {
            return true;
        }
        return false;
    }

    fn get_mvp(&self, creatures: &Vec<Creature>) -> Vec<i32> {
        let my_fishes = self.get_my_drones_scan();
        let mut my_fishes: Vec<i32> = my_fishes.iter().map(|f| f.creature_id).collect();
        my_fishes.sort();
        my_fishes.dedup();

        let fish_to_collect: Vec<(i32, CreatureType)> = creatures
            .iter()
            .map(|c| (c.id, c.creature_type))
            .filter(|(c, _t)| !my_fishes.contains(c))
            .collect();

        let type_0 = fish_to_collect
            .iter()
            .filter(|(_c, t)| *t == CreatureType::Type0)
            .count();
        let type_1 = fish_to_collect
            .iter()
            .filter(|(_c, t)| *t == CreatureType::Type1)
            .count();
        let type_2 = fish_to_collect
            .iter()
            .filter(|(_c, t)| *t == CreatureType::Type2)
            .count();

        if type_0 < type_1 && type_0 < type_2 {
            return fish_to_collect
                .iter()
                .filter(|(_c, t)| *t == CreatureType::Type0)
                .map(|x| x.0)
                .collect();
        } else if type_1 < type_0 && type_1 < type_2 {
            return fish_to_collect
                .iter()
                .filter(|(_c, t)| *t == CreatureType::Type1)
                .map(|x| x.0)
                .collect();
        } else if type_2 < type_0 && type_2 < type_1 {
            return fish_to_collect
                .iter()
                .filter(|(_c, t)| *t == CreatureType::Type2)
                .map(|x| x.0)
                .collect();
        } else {
            return fish_to_collect.iter().map(|x| x.0).collect();
        }
    }
}

// ===================================================================================================================
// ===================================================================================================================
// Main ==============================================================================================================
// ===================================================================================================================
// ===================================================================================================================

fn main() {
    let creatures = init();
    let mut drones_behavior = Vec::new();
    loop {
        let game_state = get_game_state();
        for i in 0..game_state.get_my_drone_count() {
            let drone = game_state.my_drone.get(i).expect("plz codingame");
            //init or update
            fun_name(&mut drones_behavior, &game_state, drone, &creatures);
        }
    }
}

fn fun_name(
    drones_behavior: &mut Vec<DroneBehavior>,
    game_state: &GameState,
    drone: &Drone,
    creatures: &Vec<Creature>,
) {
    if drones_behavior.len() < game_state.get_my_drone_count() {
        let mut d = DroneBehavior::new(drone.clone(), drones_behavior.len() as i32);
        if drones_behavior.len() % 2 == 0 {
            d.direction = -1 * d.direction;
        }
        drones_behavior.push(d.clone());
    }
    let drone_behavior = get_drone_by_id(drone.drone_id, drones_behavior);
    drone_behavior.drone_state = drone.clone();
    drone_behavior.behave(game_state, creatures);
}

// ===================================================================================================================
// ===================================================================================================================
// GET GAME INPUT ====================================================================================================
// ===================================================================================================================
// ===================================================================================================================

fn init() -> Vec<Creature> {
    let mut creature = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let creature_count = parse_input!(input_line, i32);
    for i in 0..creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let creature_id = parse_input!(inputs[0], i32);
        let color = parse_input!(inputs[1], i32);
        let _type = parse_input!(inputs[2], i32).into();

        creature.push(Creature {
            id: creature_id,
            color,
            creature_type: _type,
        });
    }

    return creature;
}

fn get_game_state() -> GameState {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_score = parse_input!(input_line, i32);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_score = parse_input!(input_line, i32);

    let mut my_scan = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_scan_count = parse_input!(input_line, i32);
    for i in 0..my_scan_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature_id = parse_input!(input_line, i32);
        my_scan.push(creature_id);
    }

    let mut foe_scan = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_scan_count = parse_input!(input_line, i32);
    for i in 0..foe_scan_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let creature_id = parse_input!(input_line, i32);
        foe_scan.push(creature_id);
    }

    let mut my_drone = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let my_drone_count = parse_input!(input_line, i32);
    for i in 0..my_drone_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let drone_id = parse_input!(inputs[0], i32);
        let drone_x = parse_input!(inputs[1], i32);
        let drone_y = parse_input!(inputs[2], i32);
        let emergency = parse_input!(inputs[3], i32);
        let battery = parse_input!(inputs[4], i32);

        my_drone.push(Drone {
            drone_id,
            drone_x,
            drone_y,
            emergency,
            battery,
        });
    }

    let mut foe_drone = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let foe_drone_count = parse_input!(input_line, i32);
    for i in 0..foe_drone_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let drone_id = parse_input!(inputs[0], i32);
        let drone_x = parse_input!(inputs[1], i32);
        let drone_y = parse_input!(inputs[2], i32);
        let emergency = parse_input!(inputs[3], i32);
        let battery = parse_input!(inputs[4], i32);

        foe_drone.push(Drone {
            drone_id,
            drone_x,
            drone_y,
            emergency,
            battery,
        });
    }

    let mut drone_scan = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let drone_scan_count = parse_input!(input_line, i32);
    for i in 0..drone_scan_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let drone_id = parse_input!(inputs[0], i32);
        let creature_id = parse_input!(inputs[1], i32);

        drone_scan.push(Scan {
            drone_id,
            creature_id,
        });
    }

    let mut visible_creature = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let visible_creature_count = parse_input!(input_line, i32);
    for i in 0..visible_creature_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let creature_id = parse_input!(inputs[0], i32);
        let creature_x = parse_input!(inputs[1], i32);
        let creature_y = parse_input!(inputs[2], i32);
        let creature_vx = parse_input!(inputs[3], i32);
        let creature_vy = parse_input!(inputs[4], i32);

        visible_creature.push(VisibleCreature {
            creature_id,
            creature_x,
            creature_y,
            creature_vx,
            creature_vy,
        });
    }

    let mut radar_blip = Vec::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let radar_blip_count = parse_input!(input_line, i32);
    for i in 0..radar_blip_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let drone_id = parse_input!(inputs[0], i32);
        let creature_id = parse_input!(inputs[1], i32);
        let radar = inputs[2].trim();
        radar_blip.push(Blip {
            drone_id,
            creature_id,
            radar: radar.into(),
        });
    }

    return GameState {
        my_score,
        foe_score,
        my_scan,
        foe_scan,
        my_drone,
        foe_drone,
        drone_scan,
        visible_creature,
        radar_blip,
    };
}
