/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_discord_entitycache_core::entity;

/// A role entity.
#[allow(clippy::module_name_repetitions)]
#[entity(
    from = "twilight_model::guild::Role",
    assume = ["NightlyCachedRoles"],
    id = ["guild_id", "id"],
    include = ["color", "flags", "hoist", "icon", "managed", "mentionable", "position"],
    extra = [
        "guild_id": "Id<GuildMarker>",
    ],
    overrides = [
        "RoleFlags": "twilight_model::guild::RoleFlags"
    ],
    relates = [
        unique "GuildEntity": via "guild_id" as "id",
    ],
)]
pub struct RoleEntity;
