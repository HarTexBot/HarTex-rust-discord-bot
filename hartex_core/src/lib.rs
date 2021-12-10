/* SPDX-License-Identifier: GPL-2.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # `hartex_core` - The `HarTex` Core Library
//!
//! The `hartex_core` library contains the core functionality for the `HarTex` Discord bot.
//!
//! ## Features
//!
//! - `twilight-bundled`: bundles most of the `twilight` ecosystem of crates with the library,
//!                       removes the need to include the dependencies repeatedly across the
//!                       `HarTex` crates.
//!
//! - `tracing-bundled`: bundles tracing, a logging library for use within the `HarTex` crates.

#![deny(clippy::pedantic, warnings, unsafe_code)]

pub use ctrlc;

#[cfg(feature = "twilight-bundled")]
pub mod discord;
pub mod error;
pub mod events;
#[cfg(feature = "tracing-bundled")]
pub mod logging;
pub mod time;

/// # Static `HARTEX_BUILD`
///
/// Represents the current version and build of `HarTex` Discord bot.
pub static HARTEX_BUILD: &str = "Version 1.24.0-nightly-10-12-2021 (Build 626)";

/// # Static `IS_STABLE`
///
/// Determines whether this version of the bot is stable.
pub static STABLE: bool = false;
