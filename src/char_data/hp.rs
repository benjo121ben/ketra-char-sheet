use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HpInfo {
    ancestry_hp: i32,
    class_hp:i32,
    max_hp: i32,
    current_hp: i32,
    temp_hp:i32
}

impl HpInfo {
    pub fn new(ancestry_hp: i32, class_hp: i32, level: i32, con: i32) -> Self{
        Self {
            ancestry_hp,
            class_hp,
            max_hp: ancestry_hp + (class_hp + con) * level,
            current_hp: ancestry_hp + (class_hp + con) * level,
            temp_hp: 0,
        }
    }

    pub fn calculate_max_hp(self: &mut Self, level: i32, con: i32) -> i32{
        self.max_hp = self.ancestry_hp + (self.class_hp + con) * level;
        self.max_hp
    }

    pub fn get_max_hp(self: &Self) -> i32{
        return self.max_hp;
    }

    pub fn get_hp(self: &Self) -> i32{
        return self.current_hp;
    }

    pub fn get_temp(self: &Self) -> i32{
        return self.temp_hp;
    }

    pub fn set_temp(self: &mut Self, value: i32){
        self.temp_hp = std::cmp::max(value, 0);
    }

    pub fn change_hp(self: &mut Self, value: i32) {
        let mut change = value;
        if(self.temp_hp > 0 && value < 0) {
            self.temp_hp += change; 
            change = self.temp_hp;
            self.temp_hp = std::cmp::max(self.temp_hp, 0);
        }
        if change != 0 {
            self.current_hp = std::cmp::max(std::cmp::min(self.current_hp + change, self.max_hp),0);
        }
    }
}