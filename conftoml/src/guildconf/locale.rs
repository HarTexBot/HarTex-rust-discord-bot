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
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `locale` Module
//!
//! This module contains configuration for the bot interface locale (and language).

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use serde::de::{
    Error,
    Visitor
};

/// # Enum `Locale`
///
/// Represents a locale (and language).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    /// # Enum Variant `Locale::EnAu`
    ///
    /// The "en_AU" locale.
    EnAu,

    /// # Enum Variant `Locale::EnGb`
    ///
    /// The "en_GB" locale.
    EnGb,

    /// # Enum Variant `Locale::EnUs`
    ///
    /// The "en_US" locale.
    EnUs,

    /// # Enum Variant `Locale::ZhHk`
    ///
    /// The `zh_HK`
    ZhHk
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::EnAu => write!(f, "en_AU"),
            Self::EnGb => write!(f, "en_GB"),
            Self::EnUs => write!(f, "en_US"),
            Self::ZhHk => write!(f, "zh_HK")
        }
    }
}

/// # Struct `GuildConfigLocaleDeserializerRefStrVisitor`
///
/// A `&str` visitor for deserializing a `Locale` for `GuildConfig`.
pub struct GuildConfigLocaleDeserializerRefStrVisitor;

impl<'visitor> Visitor<'visitor> for GuildConfigLocaleDeserializerRefStrVisitor {
    type Value = Locale;

    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "a string representing a locale")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error {
        Ok(match v {
            "en_AU" => Locale::EnAu,
            "en_GB" => Locale::EnGb,
            "en_US" => Locale::EnUs,
            "zh_HK" => Locale::ZhHk,
            _ => return Err(Error::custom("invalid locale"))
        })
    }
}
