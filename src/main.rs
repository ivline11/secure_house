//for command line
use std::env;
//for 강제종료
use std::process::exit;
//긴문자 사용하기 위해서
std::collections::HashSet<String>
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
    fn check_key(self, key: &str) -> bool{
        self.authorized_keys.contains(key);
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
            if self.check_key = true;
            self.lock_state = false;
            println!("SUCCESS {} TURNS KEY {}", user, k);
        } else {
            println!("FAILURE {} HAD INVALID KEY {} INSERTED", user, k);
            self.lock_state = true;
        }
    } else {
        println("FAILURE {} HAD NO KEY INSERTED", user);
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
        if self.owner != user || self.people_inside.contains(&self.owner){
            println!("LOCK STATE DENIED");
        }else{
            self.authorized_keys.clear();
            for k in new_keys{
                self.authorized_keys.push(k.clone());
            }
            self.authorized_keys.push(FIREFIGHTER_KEY.to_string());
            self.reset_lock_state();
            println("LOCK CHANGED");
        }
    }
    
    //leave house
    //position 알아야 vector에서 뺄 수 있으므로 다르게.
    fn leave_house(&mut self, user: &str){
        if let Some(pos) = self.inside.iter().position(|x| x == user) {
            self.inside.remove(pos);
            println!("{} LEFT", user);
        } else {
            println!("{} NOT HERE", user);
        }
    }

}

