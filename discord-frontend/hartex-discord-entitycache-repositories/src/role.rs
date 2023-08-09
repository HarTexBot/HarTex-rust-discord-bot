/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use std::env;

use hartex_discord_entitycache_core::error::CacheResult;
use hartex_discord_entitycache_core::traits::Entity;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_entities::role::RoleEntity;
use redis::AsyncCommands;
use redis::Client;

pub struct CachedRoleRepository;

impl Repository<RoleEntity> for CachedRoleRepository {
    async fn get(&self, (guild_id, id): <RoleEntity as Entity>::Id) -> CacheResult<RoleEntity> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;

        let name = connection
            .get::<String, String>(format!("guild:{guild_id}:role:{id}"))
            .await?;

        Ok(RoleEntity {
            guild_id,
            id,
            name,
        })
    }

    async fn upsert(&self, entity: RoleEntity) -> CacheResult<()> {
        let pass = env::var("DOCKER_REDIS_REQUIREPASS")?;
        let client = Client::open(format!("redis://:{pass}@127.0.0.1/"))?;
        let mut connection = client.get_tokio_connection().await?;
        connection
            .set(
                format!(
                    "guild:{}:role:{}:name",
                    entity.guild_id, entity.id
                ),
                entity.name,
            )
            .await?;

        Ok(())
    }
}
