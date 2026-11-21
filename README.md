# Cherished Worlds (Horizon port)

Exact 1:1 Rust port of [Cherished Worlds](https://github.com/illusivesoulworks/cherishedworlds)
(Illusive Soulworks, LGPL-3.0-or-later) for the Horizon client.

Every Java class maps to one module: `common_mod`, `config`, `constants`,
`favorites_list`, `mixin_hooks`, `mixins`, `screen_event_hooks`, `services`,
`widgets/*`, with `seams.rs` carrying the engine-bound surfaces (Minecraft
client types, SpectreLib config, MixinExtras) the engine binds.

The mod API wires the original's config entry as a setting (the backup world
type, default `All`, upstream's own lang key) and runs the original's entry
(`FavoritesList.load()`) on client setup.

Engine-stage seams: the world-list and screen hooks (favorites star, backup
status, swap guard, favorite widgets) have no mod API surface yet and stay
behind the port's host traits.

## License

Upstream is Copyright (C) 2018-2026 Illusive Soulworks, LGPL-3.0-or-later.
This port carries the upstream license and credit per the port policy:
LICENSE holds the LGPL-3.0 text, LICENSE.GPL the GPL-3.0 text it incorporates.
