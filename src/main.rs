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


struct Drone {
    drone_id: i32,
    drone_x : i32,
    drone_y : i32,
    emergency: i32,
    battery : i32,
}

#[derive(Debug, Clone)]
struct Scan {
    drone_id: i32,
    creature_id: i32,
}

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

    fn is_four_of_a_kind(&self, fishes: &Vec<Scan>, creatures: &Vec<Creature>) -> bool {
        
        return false;
    }

    fn do_i_need_to_save(&self, drone_id: i32, creatures: &Vec<Creature>) -> bool {

        //if I have all fish from a type
        let my_fishes = self.get_drone_scan(drone_id);
        let other_scan = self.foe_scan;

        if self.is_four_of_a_kind(&my_fishes, creatures) && 
           !self.is_four_of_a_kind(&other_scan, creatures) {
            return true;
        }
        
        return false;
    }
}


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

/**
 * Score points by scanning valuable fish faster than your opponent.
 **/ 
fn main() {

    let creatures = init();
    
    let mut direction = 1;
    let mut debounce = 0;
    // game loop
    loop {

        let game_state = get_game_state();
        
        for i in 0..game_state.get_my_drone_count() {

            // Write an action using println!("message...");
            // To debug: eprintln!("Debug message...");


            let drone = game_state.my_drone.get(i).expect("plz codingame");
            eprintln!("my scan {:#?}", game_state.get_drone_scan(drone.drone_id));

            let mut dephasage = 300f64;
            let a = 2_800f64; //amplitude
            let b = 2f64 * PI / 5_000f64; //period
            let k = 6_200f64; //axe
            if        drone.drone_x + 600 > 10000 {
                direction = -1;
            } else if drone.drone_x - 600 < 0 {
                direction = 1;
            }

            let next_x: f64 = (drone.drone_x as f64 + (600f64 * direction as f64));
            let y: f64;
            if direction < 0 {
                y = (-1.0*a) * (b*next_x).sin() + k;
            }
            else {
                y = a * (b*next_x).sin() + k;
            }

            let mut light = 0;
            debounce += 1;

            if drone.battery > 5 && drone.drone_y > 3_000 && debounce > 10{
                light = 1;
                debounce = 0;
            }
            println!("MOVE {} {} {}", next_x.round() as i32, y.round() as i32, light); // MOVE <x> <y> <light (1|0)> | WAIT <light (1|0)>


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
