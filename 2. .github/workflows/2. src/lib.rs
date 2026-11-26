#![feature(concat_idents, proc_macro_hygiene)]
use smashline::{
    *,
    api::*,
    phx::Hash40,
};
use smashline::smash::{
    app::lua_bind::*,
    lib::lua_const::*,
};
use smashline::smash_script::macros::*;

static mut PREV_DAMAGE: [f32; 8] = [0.0; 8];

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if entry_id >= 8 {
        return;
    }

    let current_damage = DamageModule::damage(fighter.module_accessor, 0);

    if current_damage > PREV_DAMAGE[entry_id] {
        PLAY_SE(fighter, Hash40::new("vc_mario_damage01"));
    }

    PREV_DAMAGE[entry_id] = current_damage;
}

#[skyline::main(name = "smash_damage_voice")]
pub fn main() {
    Agent::new("mario")
        .on_line(Main, fighter_frame)
        .install();
}
