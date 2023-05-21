#![allow(dead_code)]
#![allow(unused_variables)]

pub fn basics_scalar() {
    let x = 3.0;

    let tup = (10, "Bimal", 233.45);

    let (x, y, z) = tup;

    println!("x:{} y:{} z:{}", x, y, z);

    let mut nums = vec![1, 3, 4];
    nums.push(6);

    let mut vet = Vec::<i32>::with_capacity(2);
    vet.push(10);
    vet.push(2);
    vet.push(11);

    println!("{}", vet.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v; // none owning reference to vector vc

    println!("{:?}", sv);

    let name = String::from("Tyler");
    let course = "rust".to_string();
    let new_name = name.replace(&name, "Ty");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    println!("{}", "ONE".to_lowercase().to_string() == "one")
}

//-----------------Enum implementation ----------------------------------------------

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8,
    },
    Wait,
    Attack(Direction),
}

pub fn structs_basics() {
    struct Color(u8, u8, u8);

    let simulated_player_action = PlayerAction::Move {
        direction: Direction::N,
        speed: 2,
    };

    match simulated_player_action {
        PlayerAction::Wait => println!("player want to wait"),
        PlayerAction::Move { direction, speed } => {
            println!("player want to move direction {:?} with speed {}", direction, speed)
        }
        PlayerAction::Attack(direction) => {
            println!("player want to attack direction {:?}", direction)
        }
    }
}

//---------------struct implementation ----------------------------------------------

struct Player {
    name: String,
    iq: u8,
    fiends: u8,
}

impl Player {
    fn with_name(name: &str) -> Self {
        return Player {
            name: name.to_string(),
            iq: 100,
            fiends: 100,
        };
    }

    fn get_friends(&self) -> u8 {
        return self.fiends;
    }

    fn set_friends(&mut self, count: u8) {
        self.fiends = count
    }
}

pub fn get_player() {
    let mut player = Player::with_name("Dave");
    player.set_friends(23);

    println!("{}'s friends count : {} ", player.name, player.get_friends());
    let _ = Player::get_friends(&player);
}
