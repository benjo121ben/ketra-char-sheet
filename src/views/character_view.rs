use std::collections::HashMap;

use crate::char_data::character::*;
use crate::char_data::conditions::Condition;
use crate::char_data::feats::Feat;
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::stats::ProficiencyType;
use crate::error_template::SheetError;
use crate::server_side::server_functions::*;
use super::view_helpers::*;
use super::stats_views::*;
use super::equip_views::*;

use leptos::*;
use leptos::logging::log;

#[component]
pub fn BaseView(
    char: Character,
    feats: Vec<Feat>,
    conditions: Vec<Condition>,
    trait_data: HashMap<String, String>
) -> impl IntoView {
    //log!("Char on init {char:#?}");
    let (read_ketra, write_ketra) = create_signal(char);
    let sheet_error = create_rw_signal(SheetError::new(""));
    let upload_ketra = create_action( move |_:&i32| async move {
        let ketra = read_ketra.get_untracked();
        sheet_error.set(SheetError::new(""));
        let ret_val = set_char(ketra).await;
        match ret_val {
            Ok(_) => {},
            Err(err) => {log!("Error saving "); sheet_error.set(SheetError::new(&err.to_string()))},
        }
    });
    create_effect(move |prev| {
        let _getUp = read_ketra.with(|c| c.name.clone());
        match prev {
            Some(_) => {
                upload_ketra.dispatch(0);
                return Some(0);
            },
            None => Some(0)
        }
        
    });
    provide_context(read_ketra);
    provide_context(sheet_error);
    provide_context(write_ketra);
    provide_context(conditions.clone());
    provide_context(trait_data.clone());
    let feat_map: HashMap<String, Feat> = feats.into_iter().map(|feat: Feat| (feat.name.clone(), feat)).collect();
    provide_context(feat_map);
    view!{
        <CharView/>
        <HorseSection/>
    }
}

#[component]
pub fn CharView() -> impl IntoView {
    let (read_ketra, write_ketra) = get_base_context("CharView");
    let sheet_error = get_sheet_error_context("CharView");
    let send_debug = create_action( move |_:&i32| async move {
        sheet_error.set(SheetError::new(""));
        let ret_val = ping_server().await;
        match ret_val {
            Ok(val) => {log!("Arrived successfully "); sheet_error.set(SheetError::new(&format!("{val}")))},
            Err(err) => {log!("Error saving "); sheet_error.set(SheetError::new(&err.to_string()))},
        }
    });
    view! {
        <h2
            on:click=move|_|send_debug.dispatch(0)
        >{move || read_ketra.with(|k| k.name.clone())}</h2>
        <TopCharViewSection/>
        <Show
            when=move || sheet_error.get().msg != String::from("")
        >
            <p style="color: red;">{move || sheet_error.get().msg.clone()}</p>
        </Show>
        <div class="flex-row space-between">
            <ProficiencySidebar/>
            <section class="flex-col center-col-layout">
                <textarea 
                    class="center-text-area" 
                    id="test"
                    on:change=move |event| {
                        write_ketra
                        .update(|c| {
                            let val: String = event_target_value(&event);
                            c.text = val;
                        })
                    }
                    prop:value={move || read_ketra.with(|c| c.text.clone())}
                />
            </section>
            <section class="flex-col equip-section">
                <EquipView/>
            </section>
            <section class="flex-col right-side-col">
                <TacticsView/>
                <FeatView/>
            </section>
        </div>
    }
}

#[component]
pub fn TopCharViewSection() -> impl IntoView {
    let (read_ketra, write_ketra) = get_base_context("TopCharView");
    view!{
        <div id="top_div" class="flex-row flex-wrap no-grow-children">
            <section>
                <div class="flex-row align-center no-grow-children">   
                    <button
                        on:click=move |_| {write_ketra.update(move |c| {
                            c.level += 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                            c.horse_hp_info.calculate_max_hp(c.level, 2);
                        })}
                        on:contextmenu=move |_| {write_ketra.update(move |c| {
                            c.level -= 1;
                            c.hp_info.calculate_max_hp(c.level, c.attributes.get_stat_val("con").expect("There should be a con stat"));
                            c.horse_hp_info.calculate_max_hp(c.level, 2);
                        })}
                    >
                        Level {move || read_ketra.with(|k| k.level)}
                    </button>
                    <div>SIZE<br/>Medium</div>
                    <div>SPEED<br/>30ft.</div>
                </div>
            </section>
            <section class="align-center">
                <DefenseView/>
            </section>
            <section class="align-center">
                <MainStatsView/>
            </section>
            <section class="align-center" id="hp_section">
                <HpView horse=false/>
            </section>
            <section class="align-center" id="shield_section">
                <ShieldView/>
            </section>
        </div>
    }
}

#[component]
pub fn ProficiencySidebar(
) -> impl IntoView {
    let (read_char, _): (ReadSignal<Character>, WriteSignal<Character>) = get_base_context("ProficiencySidebar");
    let show_edit_stats = create_rw_signal(false);
    let has_incr_init = move || {
        read_char.with(|c| {
            check_character_flag(c, "incred_init")
        })
    };
    view! {
        <section class="flex-col flex-wrap" style="flex-grow: 0; flex-shrink: 0">
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::ClassDC]/></b>
            <b><SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Perception]/></b>
            <Show when=has_incr_init>
                <div class="skill-grid">
                    <div style="display:flex; flex: 1 0 0">initiative</div>
                    <div></div>
                    <div>+2</div>
                
                </div>
            </Show>
            <div class="flex-col">
                <h5>Saves</h5>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Save]/>
            </div>
            <div class="flex-col">
                <h5>Skills</h5>
                <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Skill, ProficiencyType::Lore]/>
            </div>
            <Show when=move || show_edit_stats.get()>
                <div class="flex-col">
                    <h5>Attack</h5>
                    <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Weapon]/>
                </div>
                <div class="flex-col">
                    <h5>Armor</h5>
                    <SwitchProfView show_edit_stats=show_edit_stats types=vec![ProficiencyType::Armor]/>
                </div>
            </Show>
            <button on:click=move |_| show_edit_stats.update(|b| *b=!*b) style="justify-content:center">Edit</button>
        </section>
    }
}

#[component]
pub fn HorseSection(
) -> impl IntoView {
    let (read_c, _) = get_base_context("HorseSection");
    let get_bonus = move || {
        let level = read_c.with(|c| c.level);
        ProficiencyLevel::Trained.get_bonus(level)
    };
    let get_att = move || {
        get_bonus() + 3
    };
    let get_ac = move || {
        get_bonus() + 2 + 10
    };
    view! {
        <div class="flex-row">
            <section class="align-center">
                <HpView horse=true/>
            </section>
            <section class="align-center">
                <label>AC: {get_ac}</label>
            </section>
            <section class="align-center">
                <label>Attack: {get_att}</label>
            </section>
        </div>
        <img src="horse.png" style="display:flex"/>
        <img src="support.png" style="display:flex"/>
        <img src="mature_horse.png" style="display:flex"/>
    }
}


/* <section class="flex-row flex-grow-1 flex-shrink" style="justify-content:center">
    {
        move || {
            let condition_clone: Condition = conditions[0].clone();
            view!{
                <h3 style="no-grow">{move || condition_clone.name.clone()}</h3>
                <p style="flex-grow-1">{move || condition_clone.description.clone()}</p>
            }
        }
    }
</section> */