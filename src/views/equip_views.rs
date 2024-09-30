use crate::char_data::character::*;
use crate::char_data::gear::{Gear, GearType};
use crate::char_data::proficiency::ProficiencyLevel;
use crate::char_data::tactics::Tactic;
use crate::views::action_view::ActionView;
use super::stats_views::TraitView;
use ev::Event;
use leptos::*;
use leptos::logging::log;

#[component]
pub fn EquipView() -> impl IntoView {
    let character_data = use_context::<ReadSignal<Character>>().expect("WeaponView: Character should be set at this point");
    view! {
        <For
            each=move ||character_data.with(|k| k.gear_list.clone())
            key=|gear_item| gear_item.name.clone()
            children=move |gear_item| {
                let item_name = gear_item.name.clone();
                let collapse = create_rw_signal(false);
                view! {
                    <div class="flex-col align-flex-start bright-bg" 
                        on:click=move |_| collapse.update(|c| *c = !*c)
                    >
                        <h4 style="margin:unset">{
                            move || format!("{item_name}")
                        }</h4>
                        <Show when=move || collapse.get()>
                            <TraitView trait_names=gear_item.traits.clone()/>
                            <hr/>
                            <div class="tiny-text" inner_html={let desc = gear_item.description.clone(); move || desc.clone()}></div>
                        </Show>
                    </div>
                }
            }
        />
    }
}

#[component]
pub fn WeaponView(
    item: Gear
) -> impl IntoView {
    let character_data = use_context::<ReadSignal<Character>>().expect("WeaponView: Character should be set at this point");
    if item.g_type != GearType::Weapon {
        panic!("WeaponView: This item is not a weapon");
    }

    if item.damage.is_none() {
        panic!("WeaponView: This item does not have a damage die");
    }

    if item.proficiency.is_none() {
        panic!("WeaponView: This item does not have a proficiency");
    }

    let get_view = move || character_data.with(|c| {
        let name_clone = item.name.clone();
        let weapon_Item = c.gear_list.iter().find(|i| i.name == name_clone.clone()).cloned();
        match weapon_Item {
            Some(weapon) => {
                let prof_copy = weapon.proficiency.unwrap().clone();
                let stat_opt = c.get_prof_obj_from_name(&prof_copy);
                match stat_opt {
                    Some(prof) => {
                        let attack_bonus = prof.calculate_stat(c);
                        let bonus_progression_proficiency = 1; //TODO here automatic bonus progression
                        let prefix =String::from(
                            if attack_bonus + bonus_progression_proficiency > 0 {
                                "+"
                            } else {""}
                        );
                        let dice_amount = 1; //TODO here automatic bonus progression
                        let dice_size = weapon.damage.unwrap();
                        let damage = format!("{dice_amount}d{dice_size}");
                        let total_bonus = format!("{0}{1} {2}", prefix, attack_bonus + bonus_progression_proficiency, damage); //TODO str bonus
                        view!{
                            <div class="flex-row">
                                <h4>{move|| weapon.name.clone()}</h4>
                                <p>{
                                    move || total_bonus.clone()
                                }</p>
                                <p inner_html={move|| weapon.description.clone()}/>
                            </div>
                        }.into_view()
                    },
                    None => view!{
                        <p class="error">{let prof_name = prof_copy.clone(); format!("WeaponView: Could not find a proficiency with name {prof_name}")}</p>
                    }.into_view(),
                }
            },
            None => {
                let second_clone = name_clone.clone();
                view!{
                    <p class="error">{format!("WeaponView: Could not find a weapon with name: {second_clone}. weird synchro error")}</p>
                }.into_view()
            }
        }
    });
    view! {{get_view}}
}

#[component]
pub fn TacticsView() -> impl IntoView {
    let character_write = use_context::<WriteSignal<Character>>().expect("TacticsView: Character should be set at this point");
    let character_data = use_context::<ReadSignal<Character>>().expect("TacticsView: Character should be set at this point");
    let max_tactics = 2;
    let count_tactics = {
        move || character_data.with(|c| {
            c.tactics.iter().filter(|tactic| tactic.selected).count()
        })
    };
    let tactics_header = move || {
        let count = count_tactics();
        format!("Tactics [{count} / {max_tactics}]")
    };
    view! {
        <div>
            <h4>{tactics_header}</h4>
            <div class="flex-col no-grow" style="gap:10px">
                <For
                    each=move ||character_data.with(|k| k.tactics.clone())
                    key=|tactic| tactic.name.clone()
                    children=move |tactic| {
                        let tac_name = tactic.name.clone();
                        let collapse = create_rw_signal(false);
                        let get_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            move || character_data.with(|c|{
                                c.tactics.iter().find(|val| val.name == tac_name2).unwrap().selected
                            })
                        };
                        let update_selected_on_tactic = {
                            let tac_name2 = tactic.name.clone();
                            let get_selected_on_tactic_clone = get_selected_on_tactic.clone();
                            move |_| {
                                if count_tactics() >= max_tactics && !get_selected_on_tactic_clone() {
                                    return;
                                }
                                character_write.update(|c|{
                                    let mut_ref: &mut Tactic = c.tactics.iter_mut().find(|val| val.name == tac_name2).unwrap(); 
                                    mut_ref.selected = !mut_ref.selected
                                });
                            }
                        };
                        view! {
                            <div class="flex-col align-flex-start bright-bg" 
                                on:click=move |_| collapse.update(|c| *c = !*c)
                                on:contextmenu=update_selected_on_tactic
                                class:selected-tactic=get_selected_on_tactic
                            >
                                <div class="flex-row feat-title-row">
                                    <h4>{
                                        move || format!("{tac_name}")
                                    }</h4>
                                    <Show when=move || tactic.actions != 0>
                                        <ActionView number=tactic.actions/>
                                    </Show>
                                </div>
                                <Show when=move || collapse.get()>
                                    <TraitView trait_names=tactic.traits.clone()/>
                                    <hr/>
                                    <div class="tiny-text" inner_html={let desc = tactic.description.clone(); move || desc.clone()}></div>
                                </Show>
                            </div>
                        }.into_view()
                    }
                />
            </div>
        </div>
    }
}