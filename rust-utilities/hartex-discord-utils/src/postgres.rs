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

//! # Postgres Utilities

use tokio_postgres::error::Error;
use tokio_postgres::error::SqlState;

/// Extension trait for `PostgreSQL` errors.
#[allow(clippy::module_name_repetitions)]
pub trait PostgresErrorExt {
    /// Returns whether the `SqlState` within the error is the value specified.
    fn is(&self, state: SqlState) -> bool;
}

impl PostgresErrorExt for Error {
    fn is(&self, state: SqlState) -> bool {
        let Some(sql_state) = self.code() else {
            return false;
        };

        *sql_state == state
    }
}
