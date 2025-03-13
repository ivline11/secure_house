//for command line
use std::env;
//for 강제종료
use std::process::exit;
//긴문자 사용하기 위해서
use std::io;
use std::io::BufRead;
//cheat key
static FIREFIGHTER_KEY: &str = "FIREFIGHTER_SECRET_KEY";

//struct
struct HouseState {
    lock_state : bool,
    owner : String,
    authorized_keys: Vec<String>,
    inserted_key: Option<String>,
    people_inside: Vec<String>,
}

impl HouseState {
    fn initialization(owner_name: &str, initial_keys: &[String]) -> Self {
        let mut hs = HouseState {
            lock_state: true,
            people_inside : Vec::new(),
            owner: owner_name.to_string(),
            inserted_key: None,
            authorized_keys: Vec::new(),
        };
        for k in initial_keys {
            hs.authorized_keys.push(k.clone());
        }
        hs.authorized_keys.push(FIREFIGHTER_KEY.to_string());
        hs
    }
    
    //key authorization checking
    fn check_key(&self, key: &str) -> bool{
        self.authorized_keys.contains(&key.to_string())
    }
    fn reset_lock_state(&mut self) {
        self.inserted_key = None;
        self.lock_state = true;
    }

    //insert step
    fn insert_key(&mut self, user: &str, key: &str){
        self.inserted_key = Some(key.to_string());
        self.lock_state = true;
        println!("KEY {} INSERTED BY {}", key, user);
    }

    //turn step
    fn turn_key(&mut self, user: &str){
        if let Some(k) = &self.inserted_key{
            if self.check_key(k){
                self.lock_state = false;
                println!("SUCCESS {} TURNS KEY {}", user, k);
            } else {
                println!("FAILURE {} HAD INVALID KEY {} INSERTED", user, k);
                self.lock_state = true;
            }
        } else {
            println!("FAILURE {} HAD NO KEY INSERTED", user);
        }
    }
    //enter step
    //self.lock_state ! 붙혀보기
    fn enter_house(&mut self, user: &str){
        if self.inserted_key.is_some() && self.lock_state == false {
            self.people_inside.push(user.to_string());
            println!("ACCESS ALLOWED");
            self.reset_lock_state();
        } else {
            println!("ACCESS DENIED");
        }
    }
    //return who's inside the house
    fn people_inside_house(&mut self){
        if self.people_inside.is_empty(){
            println!("NOBODY HOME");
        } else {
            println!("{}", self.people_inside.join(","));
        }
    }
    
    //rekey
    //position 알필요 없으므로 contains 사용
    fn rekey(&mut self, user: &str, new_keys: &[String]){
        if self.owner != user || !self.people_inside.contains(&self.owner){
            println!("LOCK CHANGE DENIED");
        }else{
            self.authorized_keys.clear();
            for k in new_keys{
                self.authorized_keys.push(k.clone());
            }
            self.authorized_keys.push(FIREFIGHTER_KEY.to_string());
            self.reset_lock_state();
            println!("LOCK CHANGED");
        }
    }
    
    //leave house
    //position 알아야 vector에서 뺄 수 있으므로 다르게.
    fn leave_house(&mut self, user: &str){
        if let Some(pos) = self.people_inside.iter().position(|x| x == user) {
            self.people_inside.remove(pos);
            println!("{} LEFT", user);
        } else {
            println!("{} NOT HERE", user);
        }
    }

}
//main function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <owner_name> [<key_1> <key_2> ...]", args[0]);
        exit(1);
    }
    let owner= &args[1];
    let initial_keys = if args.len() > 2 {
        &args[2..]
    } else {
        &[]
    };

    let mut house = HouseState::initialization(owner, initial_keys);

    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };
        // Trim trailing whitespace
        let trimmed = line.trim();
        if trimmed.is_empty() {
            // Empty line -> terminate the program
            break;
        }

        // Split into tokens
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.is_empty() {
            println!("ERROR");
            continue;
        }
        // 1) INSERT KEY
        if tokens.len() == 4
            && tokens[0] == "INSERT"
            && tokens[1] == "KEY"
        {
            let user = tokens[2];
            let key = tokens[3];
            house.insert_key(user, key);
            continue;
        }

        // 2) TURN KEY
        if tokens.len() == 3
            && tokens[0] == "TURN"
            && tokens[1] == "KEY"
        {
            let user = tokens[2];
            house.turn_key(user);
            continue;
        }

        // 3) ENTER HOUSE
        if tokens.len() == 3
            && tokens[0] == "ENTER"
            && tokens[1] == "HOUSE"
        {
            let user = tokens[2];
            house.enter_house(user);
            continue;
        }

        // 4) WHO'S INSIDE?
        if tokens.len() == 2
            && tokens[0] == "WHO'S"
            && tokens[1] == "INSIDE?"
        {
            house.people_inside_house();
            continue;
        }

        // 5) CHANGE LOCKS
        if tokens.len() >= 3
            && tokens[0] == "CHANGE"
            && tokens[1] == "LOCKS"
        {
            let user = tokens[2];
            // any new keys are tokens[3..]
            let new_keys: Vec<String> = tokens[3..].iter().map(|s| s.to_string()).collect();
            house.rekey(user, &new_keys);
            continue;
        }

        // 6) LEAVE HOUSE 
        if tokens.len() == 3
            && tokens[0] == "LEAVE"
            && tokens[1] == "HOUSE"
        {
            let user = tokens[2];
            house.leave_house(user);
            continue;
        }

        //ERROR
        println!("ERROR");
    }
}

