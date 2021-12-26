/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_base` - The `HarTex` Core Library
//!
//! The `hartex_base` library contains the core functionality for the `HarTex` Discord bot.
//!
//! ## Features
//!
//! - `twilight-bundled`: bundles most of the `twilight` ecosystem of crates with the library,
//!                       removes the need to include the dependencies repeatedly across the
//!                       `HarTex` crates.
//!
//! - `tracing-bundled`: bundles tracing, a logging library for use within the `HarTex` crates.

#![deny(clippy::pedantic, warnings)]
#![forbid(unsafe_code)]

pub use ctrlc;

#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
#[cfg(feature = "tracing-bundled")]
pub mod logging;
pub mod stdext;
pub mod time;

/// # Function `hartex_version`
///
/// Returns the current version of `HarTex` Discord bot.
#[must_use]
pub fn hartex_version() -> &'static str {
    env!("CFG_VERSION_STR")
}

/// # Function `is_stable`
///
/// Returns whether this version of the bot is stable.
#[must_use]
pub fn is_stable() -> bool {
    matches!(env!("CFG_IS_STABLE"), "true")
}
