use std::collections::HashMap;

use crate::char_data::feats::Feat;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::ProficiencyType;
use super::action_view::ActionView;
use super::view_helpers::get_base_context;
use leptos::ev::Event;
use leptos::*;
use leptos::logging::log;




#[component]
pub fn MainStatsView() -> impl IntoView {
    let unlocked = create_rw_signal(false);
    let (read_char, write_char) = get_base_context("MainStatsView");
    let update_stat = move |id: String, offset: i32| write_char.update(|f| {
        f.attributes.set_stat(&id, f.attributes.get_stat_val(&id).expect("MainStatsView - update_stat: There should be an attribute of the same name in the char") + offset);
        f.hp_info.calculate_max_hp(f.level, f.attributes.get_stat_val("con").expect("There should be a con stat"));
    });
    view! {
        <div style="display: flex; flex-direction: row; gap: 10px">
            <For
                each=move || {
                    read_char.with(|c|
                        c.attributes
                        .as_vec()
                        .into_iter()
                        .map(|f| (String::from(f.get_id()), String::from(f.get_abbr())))
                    )
                }
                key=|(id, _)| id.clone()
                children=move |(id, abbr)| {
                    let id_for_left_click = id.clone();
                    let id_for_right_click = id.clone();
                    let val = create_memo(move |_| {
                        let id_clone = id.clone();
                        read_char.with(move |cha_obj| cha_obj.attributes.get_stat_val(&id_clone).expect("MainStatsView - View: There should be an attribute of the same name in the char"))
                    });
                    view! { 
                        <div 
                            on:click=move |_| {
                                if unlocked.get() {
                                    update_stat(id_for_left_click.clone(), 1)
                                }
                            }
                            on:contextmenu=move |_| {
                                if unlocked.get() {
                                    update_stat(id_for_right_click.clone(), -1)
                                }
                            }
                        >
                            {abbr}: {val}
                        </div> }
                }
            />
            <button
                on:click=move |_| unlocked.update(|l| *l = !*l)
            >
            {
                move || if unlocked.get() {
                    "Unlocked"
                } else {
                    "Locked"
                }
            }
            </button>
        </div>
    }
}


#[component]
pub fn HpView(
    horse: bool
) -> impl IntoView {
    let (read_char, write_char) = get_base_context("HpView");
    let reset_input = create_rw_signal(false);
    let temp_hp_switch = create_rw_signal(false);
    let get_hp_info = move || read_char.with(|c| {
        if horse {
            c.horse_hp_info.clone()
        } 
        else {
            c.hp_info.clone()
        }
    });
    let change_hp = move |val: i32| {
        write_char.update(|c| {
            if horse {
                c.horse_hp_info.change_hp(val)
            }
            else {
                c.hp_info.change_hp(val)
            }
        });
    }; 
    let hp_view = move || {
        let hp = get_hp_info().get_hp();
        let maxhp = get_hp_info().get_max_hp();
        format!("{hp}/{maxhp}")
    };
    let flip_temp_switch = {
        move || temp_hp_switch.update(|active| *active = !*active)
    };
    view! {
        <div class="flex-col align-stretch">
            <div class="flex-row">
                <label name="hp_view" id="hp_view"
                    on:click=move |_| change_hp(1)
                    on:contextmenu=move |_| change_hp(-1)
                >
                    {move || hp_view()}
                </label>
                {move || 
                    if temp_hp_switch.get() {
                        view! {
                            <input 
                                type="number" 
                                name="temphp_inp" 
                                id="temphp_inp" 
                                class="hp-input" 
                                prop:value="" 
                                on:contextmenu=move |_| flip_temp_switch()
                                on:change=move |event: Event| {
                                    write_char.update(|c|{
                                        match event_target_value(&event).parse::<i32>() {
                                            Ok(number) => {
                                                if horse {c.horse_hp_info.set_temp(number)}
                                                else {c.hp_info.set_temp(number)}
                                            },
                                            Err(err) => {log!("HpView/tempHP error getting target value: {err}")},
                                        }
                                    });
                                    temp_hp_switch.update(|active| *active = !*active);
                                }
                            />
                        }.into_view()
                    }
                    else {
                        view! {
                            <label style="color: blue" name="temphp" id="temphp"
                                on:contextmenu=move |_| flip_temp_switch()
                            >
                                {move || get_hp_info().get_temp()}
                            </label>
                        }.into_view()
                    }
                }
            </div>
            <input 
                type="number" 
                id="hp_inp" 
                class="hp-input"
                placeholder={move || if horse {"Horse"}else{"HP Change"}}
                prop:value=move || {let _ = reset_input.get(); return String::from("")} 
                on:change=move |event: Event|{ 
                    match event_target_value(&event).parse::<i32>() {
                        Ok(number) => change_hp(number),
                        Err(err) => {log!("HpView/hpInput error getting target value: {err}")},
                    }
                    reset_input.update(|f| *f=!*f);
                }
            />
        </div>
    }

}

#[component]
pub fn ShieldView() -> impl IntoView {
    let (read_char, write_char) = get_base_context("ShieldView");
    let reset_input = create_rw_signal(false);
    let get_shield_info = move || read_char.with(|c| {
        c.shield_info.clone()
    });
    let shield_view = move || {
        let hp = get_shield_info().get_hp();
        let maxhp = get_shield_info().get_max_hp();
        format!("{hp}/{maxhp}")
    };
    let check_broken = move || {
        let info = get_shield_info();
        info.get_hp() <= info.get_max_hp()/2
    };
    let update_health = move |val: i32, ignore: bool| {
        write_char.update(|c| c.shield_info.change_hp(val, ignore));
    };
    let update_hardness = move |val: i32| {
        write_char.update(|c| c.shield_info.hardness += val);
    };
    view! {
        <div class="flex-col align-stretch">
            <div class="flex-row">
                <label name="hp_view" id="hp_view"
                    class:broken=move || check_broken()
                    on:click=move |_| update_health(1,true)
                    on:contextmenu=move |_| update_health(-1,true)
                >
                    {move || shield_view()}
                </label>
                <label style="color: green" name="hardness" id="hardness"
                    on:click=move |_| update_hardness(1)
                    on:contextmenu=move |_| update_hardness(-1)
                >
                    {move || get_shield_info().hardness}
                </label>
            </div>
            <input 
                type="number" 
                id="sh_inp" 
                class="hp-input"
                placeholder="SH Change"
                prop:value=move || {let _ = reset_input.get(); return String::from("")} 
                on:change=move |event: Event|{ 
                    write_char.update(|c|{
                        match event_target_value(&event).parse::<i32>() {
                            Ok(number) => c.shield_info.change_hp(number, false),
                            Err(err) => {log!("ShieldView error getting target value: {err}")},
                        }
                    });
                    reset_input.update(|f| *f=!*f);
                }
            />
        </div>
    }

}


#[component]
pub fn EditProfListView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let (read_char, write_char) = get_base_context("EditProfListView");
    view! {
        <div class="skill-grid skill-grid-edit">
            <For
                each=move || {
                    let types_clone = types.clone();
                    read_char.with(
                        |k| k.proficiencies.clone().into_iter().filter(move |s| types_clone.contains(&s.p_type.clone()))
                    )
                }
                key=|skill| skill.name.clone()
                children=move |skill| {
                    let name = skill.name.clone();
                    let name2 = skill.name.clone();
                    let skill_prof = skill.proficiency.clone();
                    let skill_value = create_memo({move |_| read_char.with(|c| c.get_prof_obj_from_name(&(name.clone())).expect("should be able to find proficiency").calculate_stat(&c))});
                    let options = vec![ProficiencyLevel::Untrained, ProficiencyLevel::Trained, ProficiencyLevel::Expert, ProficiencyLevel::Master, ProficiencyLevel::Legendary];
                    let change_proficiency = move |event: Event| {
                        write_char.update(|character| {
                            let val: String = event_target_value(&event);
                            let indx = character.get_prof_indx_from_name(&skill.name);
                            let panic_name = skill.name.clone();
                            match indx {
                                Some(i) => {character.proficiencies[i].proficiency = ProficiencyLevel::from(val)},
                                None => {panic!("Could not get index for {panic_name}")}
                            };
                            
                        })
                    };
                    view! {
                        <div style="display:flex; flex: 1 0 0">
                            {move || name2.clone()}
                        </div>
                        <select name="proficiency" 
                            id="profs"
                            on:change=change_proficiency
                        >
                            {
                                options.into_iter().map(|lvl| view!{
                                    <option selected=lvl.to_string()==skill_prof.clone().to_string() value=lvl.to_string()>{lvl.to_string()}</option>
                                }).collect::<Vec<_>>()
                            }
                        </select>
                        <div style="margin-left=10px">
                            {move || skill_value}
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn ProficiencyListView(
    types: Vec<ProficiencyType> 
) -> impl IntoView {
    let (character_data, _) = get_base_context("ProficiencyListView");
    view! {
        <div class="skill-grid">
            <For
                each=move || {
                    let types_clone = types.clone();
                    character_data.with(
                        |k| k.proficiencies.clone().into_iter().filter(move |s| types_clone.contains(&s.p_type.clone()))
                    )
                }
                key=|skill| skill.name.clone()
                children=move |skill| {
                    let name_clone = skill.name.clone();
                    let skill_name_clone = move || skill.name.clone();
                    let get_skill_data = move || (&character_data).with(|c| c.proficiencies[c.get_prof_indx_from_name(&skill_name_clone()).expect("Expected an index for the proficiency")].clone());
                    let get_skill_prof = {
                        let data = get_skill_data.clone();
                        move || data().proficiency.to_string()[..1].to_string()
                    };
                    let get_skill_val = {
                        let data = get_skill_data.clone();
                        move || character_data.with(|c| data().calculate_stat(c))
                    };
                    let is_proficient = {
                        let get_prof = get_skill_prof.clone();
                        move || get_prof() != String::from("U")
                    };
                    view! {
                        <div>{move || name_clone.clone()}</div>
                        <div class="proficiency-letter" class:proficiency-letter-trained=is_proficient>{get_skill_prof}</div>
                        <div>{get_skill_val}</div>
                    }.into_view()
                }
            />
        </div>
    }
}

#[component]
pub fn SwitchProfView(
    types: Vec<ProficiencyType>,
    show_edit_stats: RwSignal<bool>
) -> impl IntoView {
    view!{
        <div class="flex-col" style="flex-grow: 0">
            {
                let t_clone = types.clone();
                move || if !show_edit_stats.get() {
                    view! {<ProficiencyListView types=t_clone.clone()/>}
                }
                else {
                    view! {
                        <EditProfListView types=t_clone.clone()/>
                    }.into_view()
                } 
            }
        </div>
    }
}

#[component]
pub fn DefenseView() -> impl IntoView {
    let (read_character, write_character) = get_base_context("DefenseView");

    let shield_raised = move || read_character.with(|c| c.shield_info.raised);
    let calc_ac = move || read_character.with(|c| c.calculate_ac());
    let switch_shield_pos = move |_| write_character.update(|c| c.shield_info.raised=!c.shield_info.raised);

    view!{
        <div class="flex-col" style="align-items: stretch">
            <h3 style="margin: 0; white-spacce:nowrap" on:click=switch_shield_pos class:boosted-stat=shield_raised.clone()>
                AC: {calc_ac}
            </h3>
            <button on:click=switch_shield_pos style="justify-content:center">
                {
                    move || if shield_raised() {
                        "Lower"
                    }
                    else {
                        "Raise"
                    }
                }
            </button>
        </div>
    }
}


#[component]
pub fn FeatView() -> impl IntoView {
    let full_feat_map = use_context::<HashMap<String, Feat>>().expect("FeatView: Expected full feat list to be set");
    let (read_character, _) = get_base_context("FeatView");
    
    let get_feat_list = move || {
        let mut ret_list = vec![];
        let mut missing_feats = vec![];
        read_character.with(|c| {
            c.feats.iter().for_each(|feat_name|{
                match full_feat_map.get(feat_name) {
                    Some(feat) => ret_list.push(feat.clone()),
                    None => missing_feats.push(feat_name.clone()),
                }
            });
        });
        ret_list
    };
    view!{
        <div class="flex-col">
            <h4>Feats</h4>
            <For
                each={move || get_feat_list()}
                key={move |feat| feat.name.clone()}
                children=move |feat| {
                    let collapse = create_rw_signal(false);
                    view!{
                        <div class="flex-col bright-bg" on:click=move |_| collapse.update(|c| *c = !*c)>
                            <div class="flex-row feat-title-row ">
                                <h4>{move || feat.name.clone()}</h4>
                                <Show when=move || feat.actions != 0>
                                    <ActionView number=feat.actions/>
                                </Show>
                            </div>
                            <Show when=move || collapse.get()>
                                <TraitView trait_names=feat.traits.clone()/>
                                <hr/>
                                <p class="tiny-text" inner_html={let desc = feat.description.clone(); move || desc.clone()}></p>
                            </Show>
                        </div>
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn TraitView(
    trait_names: Vec<String>
) -> impl IntoView {
    let traitMap = use_context::<HashMap<String, String>>().expect("Trait Hashmap should be set by now");
    view!{
        <div class="flex-row">{
            trait_names.into_iter().map(|t| {
                let tooltip = if t.is_empty() {
                    log!("An empty trait was set somewhere"); String::from("No tooltip") 
                }
                else {
                    let found_val = traitMap.get(&t).clone();
                    match found_val {
                        Some(description) => String::from(description),
                        None => {
                            //some traits vary after the first word, so we reattempt after a splice
                            let first_word = traitMap.get(t.split_whitespace().next().expect("TraitView: there should be at least one word in this trait by now"));
                            match first_word {
                                Some(description) => String::from(description),
                                None => {log!("No tooltip was set for {0}", t); String::from("No tooltip") }
                            }
                        },
                    }
                };
                
                view!{
                    <div class="trait tiny-text" title=tooltip>{t}</div>
                }
            }).collect::<Vec<_>>()
        }</div>
    }
}