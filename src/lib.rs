//! 1:1 Horizon Rust port of Cherished Worlds (Illusive Soulworks,
//! LGPL-3.0-or-later).
//! Upstream: https://github.com/illusivesoulworks/cherishedworlds
//!
//! Every Java class maps to one module below, same names, same logic, same
//! defaults, same control flow. Host seams (seams.rs) carry the Minecraft
//! client / SpectreLib / MixinExtras surfaces the engine binds; where an
//! upstream type is engine-bound, the seam mirrors its Java calls one to one.
//!
//! This root carries the mod API wiring on the pinned SDK (Horizon f00125f61f,
//! mixins e59da909, mod api 0.2):
//! - the original's config entry the settings page can hold (the backup world
//!   type, an enum, exposed as a three-option choice with the same default
//!   `All` and upstream's own lang key as the label);
//! - `client_setup` runs the original's entry 1:1 (`CherishedWorldsCommonMod.
//!   setup()` = `FavoritesList.load()`), then syncs the setting into the live
//!   config cell the mixin hooks read.
//!
//! Gaps, tracked: the world-list and screen hooks the mixins carry (the
//! favorites star on world and server entries, the backup-status upgrade
//! prompt, the swap guard, the favorite widgets on the create-world,
//! join-multiplayer and select-world screens) have no mod API surface; they
//! stay behind the port's host traits in `mixins`/`screen_event_hooks` until
//! one lands.
//!
//! Copyright (C) 2018-2026 Illusive Soulworks (upstream, LGPL-3.0-or-later);
//! Rust translation 2026 sparticle, under the same license.
//!
//! This program is free software: you can redistribute it and/or modify it
//! under the terms of the GNU Lesser General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or (at your
//! option) any later version. See LICENSE and LICENSE.GPL in this repository.

use mod_api::{ClientSetupEvent, ClientHost, RegistrationContext, Registrar, SettingSpec};

pub mod common_mod;
pub mod config;
pub mod constants;
pub mod favorites_list;
pub mod mixin_hooks;
pub mod mixins;
pub mod screen_event_hooks;
pub mod seams;
pub mod services;
pub mod widgets;

// Re-exports under the Java class names so call sites read 1:1.
pub use favorites_list::FavoritesList;

/// The `backupWorldType` choices in upstream enum order
/// (`BackupType.ALL`, `FAVORITED`, `NONE`), as the option lang keys.
const BACKUP_TYPE_OPTIONS: [&str; 3] = [
    "gui.cherishedworlds.config.backupWorldType.all",
    "gui.cherishedworlds.config.backupWorldType.favorited",
    "gui.cherishedworlds.config.backupWorldType.none",
];

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// 1:1 of the original's config entry the settings page can hold: the
    /// backup world type as a choice, default `All`, upstream's own lang key
    /// as the label (the entry's comment/tooltip upstream).
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::choice(
            "backupWorldType",
            "gui.cherishedworlds.config.backupWorldType",
            BACKUP_TYPE_OPTIONS,
            0,
        ));
    }

    /// 1:1 of the original's entry (`CherishedWorldsCommonMod.setup()` =
    /// `FavoritesList.load()`), then the setting syncs into the live config
    /// cell the mixin hooks read.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        crate::common_mod::CherishedWorldsCommonMod::setup();
        let choice = host.setting_choice("backupWorldType").unwrap_or(0);
        let backup_type = match choice {
            1 => crate::config::BackupType::Favorited,
            2 => crate::config::BackupType::None,
            _ => crate::config::BackupType::All,
        };
        crate::config::client().backup_world_type.set(backup_type);
    }
}
