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

use cornucopia::CodegenSettings;
use postgres::Client;
use postgres::NoTls;

pub fn main() {
    dotenvy::dotenv().unwrap();

    let queries_path = "queries";

    let url = env::var("POSTGRES_PGSQL_URL").unwrap();
    cornucopia::generate_live(
        &mut Client::connect(&url, NoTls).unwrap(),
        queries_path,
        Some("generated/queries.rs"),
        CodegenSettings {
            derive_ser: false,
            is_async: true,
        }
    ).unwrap();
}
