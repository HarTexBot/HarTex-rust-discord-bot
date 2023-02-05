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

use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_kafka_utils::traits::ClientConfigUtils;
use hartex_kafka_utils::types::CompressionType;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "uptime")]
pub struct Uptime;

impl Command for Uptime {
    async fn execute(&self, _: Interaction) -> hartex_eyre::Result<()> {
        let bootstrap_servers = env::var("KAFKA_BOOTSTRAP_SERVERS")?
            .split(';')
            .map(String::from)
            .collect::<Vec<_>>();
        let _ = ClientConfig::new()
            .bootstrap_servers(bootstrap_servers.into_iter())
            .compression_type(CompressionType::Lz4)
            .delivery_timeout_ms(30000)
            .create::<FutureProducer>()?;

        todo!()
    }
}
