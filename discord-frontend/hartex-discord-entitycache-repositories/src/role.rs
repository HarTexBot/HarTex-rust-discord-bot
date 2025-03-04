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

use std::str::FromStr;

use hartex_database_queries::queries::discord_frontend::cached_role_select_by_guild_id::CachedRoleSelectByGuildId;
use hartex_database_queries::queries::discord_frontend::cached_role_select_by_id_and_guild_id::CachedRoleSelectByIdAndGuildId;
use hartex_database_queries::queries::discord_frontend::cached_role_upsert::CachedRoleUpsert;
use hartex_discord_core::discord::model::id::Id;
use hartex_discord_core::discord::model::id::marker::GuildMarker;
use hartex_discord_core::discord::model::id::marker::RoleMarker;
use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::role::RoleEntity;

/// Repository for role entities.
pub struct CachedRoleRepository;

impl CachedRoleRepository {
    // todo: add relationship to get all roles from a guild
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn role_ids_in_guild(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> CacheResult<Vec<Id<RoleMarker>>> {
        let roles = CachedRoleSelectByGuildId::bind(guild_id.to_string())
            .executor()
            .await?
            .many()
            .await?;

        Ok(roles
            .into_iter()
            .map(|role| Id::<RoleMarker>::from_str(role.id()).unwrap())
            .collect())
    }
}

impl Repository<RoleEntity> for CachedRoleRepository {
    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    async fn get(&self, (guild_id, id): <RoleEntity as Entity>::Id) -> CacheResult<RoleEntity> {
        let data = CachedRoleSelectByIdAndGuildId::bind(id.to_string(), guild_id.to_string())
            .executor()
            .await?
            .one()
            .await?;

        Ok(RoleEntity::from(data))
    }

    #[allow(clippy::cast_lossless)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    async fn upsert(&self, entity: RoleEntity) -> CacheResult<()> {
        CachedRoleUpsert::bind(
            entity.color as i64,
            entity.icon.map(|hash| hash.to_string()),
            entity.id.to_string(),
            entity.guild_id.to_string(),
            entity.flags.bits() as i32,
            entity.hoist,
            entity.managed,
            entity.mentionable,
            entity.position as i32,
        )
        .executor()
        .await?
        .execute()
        .await?;

        Ok(())
    }
}
