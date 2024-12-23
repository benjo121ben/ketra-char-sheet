use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use super::{auto_bonus_prog::AbpData, conditions::Condition, gear::Gear, hp::{HpInfo, ShieldInfo}, proficiency::ProficiencyLevel, stats::{Attributes, CalculatedStat, ProficiencyType}, tactics::Tactic};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name: String,

    pub hp_info: HpInfo, 
    
    pub shield_info: ShieldInfo,
    
    pub level: i32,
    
    pub attributes: Attributes,
    
    #[serde(default)]
    pub text: String,
    
    #[serde(default)]
    pub background: String,
    
    #[serde(default)]
    pub class: String,
    
    #[serde(default)]
    pub proficiencies: Vec<CalculatedStat>,

    #[serde(default)]
    pub tactics: Vec<Tactic>,

    #[serde(default)]
    pub feats: Vec<String>,
    
    #[serde(default)]
    pub conditions: Vec<Condition>,
    
    #[serde(default)]
    pub gear_list: Vec<Gear>,
    
    pub horse_hp_info: HpInfo, 
    
    #[serde(default)]
    pub override_prof: HashMap<String, String>,
    
    #[serde(default)]
    pub abp_data: AbpData,
    
    #[serde(default)]
    pub flags: HashMap<String, bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCharacter {
    pub name: String,
    
    pub level: i32,

    pub hp_info: HpInfo,

    pub attributes: Vec<i32>,

    #[serde(default)]
    pub text: String,

    #[serde(default)]
    pub background: String,

    #[serde(default)]
    pub class: String,

    #[serde(default)]
    pub tactics: Vec<Tactic>,

    #[serde(default)]
    pub proficiencies: Vec<(String, ProficiencyType, ProficiencyLevel)>,

    #[serde(default)]
    pub feats: Vec<String>,

    #[serde(default)]
    pub conditions: Vec<Condition>,

    #[serde(default)]
    pub gear_list: Vec<Gear>,   

    pub shield_info: ShieldInfo,

    pub horse_hp_info: HpInfo,

    #[serde(default)]
    pub override_prof: HashMap<String, String>,

    #[serde(default)]
    pub abp_data: AbpData,

    #[serde(default)]
    pub flags: HashMap<String, bool>
}

impl Character {
    pub fn zero() -> Character {
        Character {
            name: String::from(""),
            hp_info: HpInfo::new(0,0,1, 0),
            horse_hp_info: HpInfo::new(0,0,1, 0),
            shield_info: ShieldInfo::new(20,18,5, false),
            level: 1,
            text: String::from(""),
            attributes: Attributes::zero(),
            background: String::from("Squire"),
            class: String::from("Commander"),
            proficiencies: CalculatedStat::default_array(),
            feats: vec![],
            conditions: vec![],
            tactics: vec![],
            gear_list: vec![],
            override_prof: HashMap::new(),
            abp_data: AbpData::default(),
            flags: HashMap::new(),
        }
    }
}

impl Character {
    fn get_attribute_and_lore_flag_from_skill_name(skill_name: &str, p_type: &ProficiencyType) -> String{
        return String::from(match p_type {
            ProficiencyType::Save => {
                match skill_name {
                    "Fortitude" => "con",
                    "Reflex" => "dex",
                    "Will" => "wis",
                    _ => {panic!("This save does not exist {skill_name}");}
                }
            },
            ProficiencyType::Skill => {
                match skill_name {
                    "Acrobatics" => "dex",
                    "Arcana" => "int",
                    "Athletics" => "str",
                    "Crafting" => "int",
                    "Deception" => "cha",
                    "Diplomacy" => "cha",
                    "Intimidation" => "cha",
                    "Medicine" => "wis",
                    "Nature" => "wis",
                    "Occultism" => "int",
                    "Performance" => "cha",
                    "Religion" => "wis",
                    "Society" => "int",
                    "Stealth" => "dex",
                    "Survival" => "wis",
                    "Thievery" => "dex",
                    _ => {panic!("This skill does not exist {skill_name}");}
                }
            },
            ProficiencyType::Lore => "int",
            ProficiencyType::Armor => "dex",
            ProficiencyType::Weapon => "str",
            ProficiencyType::Spell => "key",
            ProficiencyType::ClassDC => "key",
            ProficiencyType::Perception => "wis",
        });
        
    }

    pub fn get_prof_obj_from_name(self: &Self, skill_name: &str) -> Option<CalculatedStat>{
        return self.proficiencies
        .iter()
        .find(|prof| prof.name==skill_name).cloned();
    }

    pub fn get_prof_indx_from_name(self: &Self, skill_name: &str) -> Option<usize>{
        for (indx, skill) in self.proficiencies.iter().enumerate() {
            if skill.name == skill_name {
                return Some(indx);
            }
        }
        return None;
    }

    pub fn calculate_ac(self: & Self) -> i32 {
        let calc_stat = self.get_prof_obj_from_name("Medium").expect("Character must have a medium proficiency");
        let dex_cap = 1;
        let item_bonus = 4;
        let prof_bonus = calc_stat.proficiency.get_bonus(self.level);
        let raised_bonus = if self.shield_info.raised {2} else {0};
        10 + std::cmp::min(self.attributes.get_stat_val("dex").expect("Defense expects a dex attribute to be set"), dex_cap) + prof_bonus + item_bonus + raised_bonus
    }
}


impl From<SimpleCharacter> for Character{
    fn from(simp_char: SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: simp_char.name,
            hp_info: simp_char.hp_info,
            horse_hp_info: simp_char.horse_hp_info,
            shield_info: simp_char.shield_info,
            level: simp_char.level,
            text: simp_char.text,
            attributes: Attributes::from(&simp_char.attributes),
            background: simp_char.background,
            class: simp_char.class,
            proficiencies: vec![],
            feats: simp_char.feats,
            conditions: simp_char.conditions,
            tactics: simp_char.tactics,
            gear_list: simp_char.gear_list,
            override_prof: simp_char.override_prof,
            abp_data: simp_char.abp_data,
            flags: simp_char.flags
        };

        for skill_tuple in simp_char.proficiencies {
            let attribute = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str(), &skill_tuple.1);
            ret_val.proficiencies.push(CalculatedStat::new(skill_tuple.1, &attribute, skill_tuple.0.as_str(), skill_tuple.2))
        }

        return ret_val;
    }
}

impl From<&SimpleCharacter> for Character{
    fn from(simp_char: &SimpleCharacter) -> Self {
        let mut ret_val = Character {
            name: simp_char.name.clone(),
            hp_info: simp_char.hp_info.clone(),
            horse_hp_info: simp_char.horse_hp_info.clone(),
            shield_info: simp_char.shield_info.clone(),
            level: simp_char.level,
            text: simp_char.text.clone(),
            attributes: Attributes::from(&((*simp_char).attributes)),
            background: simp_char.background.clone(),
            class: simp_char.class.clone(),
            proficiencies: vec![],
            feats: simp_char.feats.clone(),
            conditions: simp_char.conditions.clone(),
            tactics: simp_char.tactics.clone(),
            gear_list: simp_char.gear_list.clone(),
            override_prof: simp_char.override_prof.clone(),
            abp_data: simp_char.abp_data.clone(),
            flags: simp_char.flags.clone()
        };

        for skill_tuple in simp_char.proficiencies.clone() {
            let attribute = Character::get_attribute_and_lore_flag_from_skill_name(skill_tuple.0.as_str(), &skill_tuple.1);
            ret_val.proficiencies.push(CalculatedStat::new(skill_tuple.1, &attribute, skill_tuple.0.as_str(), skill_tuple.2))
        }

        return ret_val;
    }
}

impl From<Character> for SimpleCharacter{
    fn from(ref_char: Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: ref_char.name.clone(),
            hp_info: ref_char.hp_info.clone(),
            horse_hp_info: ref_char.horse_hp_info.clone(),
            shield_info: ref_char.shield_info.clone(),
            level: ref_char.level,
            text: ref_char.text,
            attributes: ref_char.attributes.as_number_vec(),
            background: ref_char.background.clone(),
            class: ref_char.class.clone(),
            proficiencies: vec![],
            feats: ref_char.feats.clone(),
            conditions: ref_char.conditions.clone(),
            tactics: ref_char.tactics.clone(),
            gear_list: ref_char.gear_list.clone(),
            override_prof: ref_char.override_prof.clone(),
            abp_data: ref_char.abp_data.clone(),
            flags: ref_char.flags.clone()
        };

        ret_val.proficiencies.extend(ref_char.proficiencies.into_iter().map(|s: CalculatedStat| return (s.name, s.p_type, s.proficiency)));
        return ret_val;
    }
}

impl From<&Character> for SimpleCharacter{
    fn from(ref_char: &Character) -> Self {
        let mut ret_val = SimpleCharacter {
            name: ref_char.name.clone(),
            hp_info: ref_char.hp_info.clone(),
            horse_hp_info: ref_char.horse_hp_info.clone(),
            shield_info: ref_char.shield_info.clone(),
            level: ref_char.level,
            text: ref_char.text.clone(),
            attributes: ref_char.attributes.as_number_vec(),
            background: ref_char.background.clone(),
            class: ref_char.class.clone(),
            proficiencies: vec![],
            feats: ref_char.feats.clone(),
            conditions: ref_char.conditions.clone(),
            tactics: ref_char.tactics.clone(),
            gear_list: ref_char.gear_list.clone(),
            override_prof: ref_char.override_prof.clone(),
            abp_data: ref_char.abp_data.clone(),
            flags: ref_char.flags.clone()
        };

        ret_val.proficiencies.extend(ref_char.proficiencies.clone().into_iter().map(|s: CalculatedStat| return (s.name, s.p_type, s.proficiency)));
        return ret_val;
    }
}

