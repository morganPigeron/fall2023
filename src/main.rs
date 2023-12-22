use std::{io, f64::consts::PI};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

struct Creature {
    id: i32,
    color: i32,
    creature_type: i32,
}

struct VisibleCreature {
    creature_id: i32,
    creature_x : i32,
    creature_y : i32,
    creature_vx: i32,
    creature_vy: i32,
}

#[derive(Clone, Copy)]
struct Drone {
    drone_id: i32,
    drone_x : i32,
    drone_y : i32,
    emergency: i32,
    battery : i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Scan {
    drone_id: i32,
    creature_id: i32,
}

#[derive(Debug, Clone)]
struct Blip {
    drone_id: i32,
    creature_id: i32,
    radar: String,
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

struct DroneBehavior {
    drone_state: Drone,
    saved: bool,
    will_save: bool,
    direction: i32,
    debounce: i32,
    blip: Vec<Blip>,
}

impl DroneBehavior {
    fn new(state: Drone) -> Self {
        return Self {
            drone_state: state.clone(),
            will_save: false,
            saved: false,
            direction: 1,
            debounce: 0,
            blip: Vec::new(),
        };
    }

    fn set_my_blip(&mut self, game_state:&GameState) {
        self.blip = game_state.radar_blip.iter().filter(|b| b.drone_id == self.drone_state.drone_id).cloned().collect();
    }

    fn behave(&mut self, game_state: &GameState, creatures: &Vec<Creature>) {
        
        self.set_my_blip(game_state);
        let mvp = game_state.get_mvp(creatures);
        eprintln!("{:?}", mvp);

        let mut dephasage = 300f64;
        let a = 2_800f64; //amplitude
        let b = 2f64 * PI / 5_000f64; //period
        let k = 6_200f64; //axe

        if        self.drone_state.drone_x + 600 > 10000 {
            eprint!(" -1 ");
            self.direction = -1;
        } else if self.drone_state.drone_x - 600 < 0 {
            eprint!("  1 ");
            self.direction = 1;
        }

        let next_x: f64 = self.drone_state.drone_x as f64 + (600f64 * self.direction as f64);
        let y: f64;
        if self.direction < 0 {
            y = (-1.0*a) * (b*next_x).sin() + k;
        }
        else {
            y = a * (b*next_x).sin() + k;
        }

        let mut light = 0;
        self.debounce += 1;

        if self.drone_state.battery > 5 && self.drone_state.drone_y > 3_000 && self.debounce > 10 {
            light = 1;
            self.debounce = 0;
        }

        if game_state.do_i_need_to_save(self.drone_state.drone_id, creatures) || self.will_save {
            self.will_save = true;
            if self.drone_state.drone_y < 500 {
                self.will_save = false;
            }
            println!("MOVE {} {} {} I will save", self.drone_state.drone_x, 450, 0);
        } else {
            println!("MOVE {} {} {} Searching", next_x.round() as i32, y.round() as i32, light);
        }
    
        eprintln!("id:{};will_save:{};direction:{};debounce:{},blip:{:?}", 
                  self.drone_state.drone_id,
                  self.will_save,
                  self.direction,
                  self.debounce,
                  self.blip
                  );
    }
}

fn get_drone_by_id<'a>(id: i32, drones: &'a mut Vec<DroneBehavior>) -> &'a mut DroneBehavior {
    drones.iter_mut().find(|d| d.drone_state.drone_id == id).expect("id given by codingame")
}


impl GameState {
    fn get_my_drone_count(&self) -> usize {
        return self.my_drone.len();
    }

    fn get_drone_scan(&self, drone_id: i32) -> Vec<Scan> {
        return self.drone_scan
            .iter()
            .filter(|s| s.drone_id == drone_id)
            .cloned()
            .collect();
    } 

    fn get_my_drones_scan(&self) -> Vec<Scan> {
        self.drone_scan
            .iter()
            .filter(|s| self.my_drone.iter().filter(|d| d.drone_id == s.drone_id).count() > 0) 
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
            match creature.creature_type {
                0 => a += 1,
                1 => b += 1,
                2 => c += 1,
                _ => (), 
            }
        }
        
        assert!(a <= 4 && b <= 4 && c <= 4, "fishes {:#?}" , scanned);

        if a >= 4 || b >= 4 || c >= 4 {
            return true;
        }
        return false;
    }

    fn do_i_need_to_save(&self, drone_id: i32, creatures: &Vec<Creature>) -> bool {
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
        
        creatures.iter().map(|c| c.id).filter(|c| !my_fishes.contains(c)).collect()
    }
}



/**
 * Score points by scanning valuable fish faster than your opponent.
 **/ 
fn main() {

    let creatures = init();
    let mut drones_behavior = Vec::new();

    loop {

        let game_state = get_game_state();

        for i in 0..game_state.get_my_drone_count() {

            let drone = game_state.my_drone.get(i).expect("plz codingame");
            //eprintln!("my scan {:#?}", game_state.get_drone_scan(drone.drone_id));

            //init or update
            if drones_behavior.len() < game_state.get_my_drone_count() {
                drones_behavior.push(DroneBehavior::new(drone.clone()));
            } 
            let mut drone_behavior = get_drone_by_id(drone.drone_id, &mut drones_behavior);
            drone_behavior.drone_state = drone.clone();
            drone_behavior.behave(&game_state, &creatures);
        }
    }
}

/*
   Protocole de jeu
   Entrées d'Initialisation
   Première ligne : creatureCount un entier pour le nombre de créature en jeu.
   Les creatureCount lignes suivantes : 3 entiers décrivants chaque créature :
   creatureId l'id unique de la créature.
   color (de 0 à 3) et type (de 0 à 2).
   Entrées pour un tour de Jeu
   myScore pour votre score actuel.
   foeScore pour le score de votre adversaire.

   myScanCount pour le nombre de vos scans.
   Prochaines myScanCount lignes : creatureId l'identifiant de chaque créature scannée.

   foeScanCount pour le nombre de scans de votre adversaire.
   Prochaines foeScanCount lignes : creatureId l'identifiant de chaque créature scannée.

   Pour votre drone :
   droneId : l'identifiant de ce drone.
   droneX et droneY : la position de ce drone.
   battery : le niveau de batterie de ce drone.
   Pour le drone de votre adversaire :
   droneId : l'identifiant de ce drone.
   droneX et droneY : la position de ce drone.
   battery : le niveau de batterie de ce drone.

   Pour chaque poisson :
   creatureId : l'id unique de la créature.
   creatureX et creatureY : la position de la créature.
   creatureVx et creatureVy : la vitesse actuelle de la créature.
   Les variables restantes peuvent être ignorées et seront utilisées dans des ligues ultérieures.
   Sortie
   Une ligne : une instruction valide pour votre drone :
   MOVE x y light : fait bouger le drone vers (x,y), avec les moteurs allumés.
   WAIT light. Les moteurs sont éteints. Le drone va couler mais peut toujours scanner les poissons aux alentours.
   light à 1 pour activer la lumière augmentée, 0 sinon.
   Contraintes
   creatureCount = 12 pour cette ligue
   myDroneCount = 1 pour cette ligue

   Temps de réponse par tour ≤ 50ms
   Temps de réponse pour le premier tour ≤ 1000ms
   */
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
        let _type = parse_input!(inputs[2], i32);

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
            creature_x ,
            creature_y ,
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
        let radar = inputs[2].trim().to_string();
        radar_blip.push(Blip {
            drone_id,
            creature_id,
            radar,
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
