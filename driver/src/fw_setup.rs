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

//! # The `fw_setup` Module
//!
//! Utility function for setting up the command framework.

use hartex_base::logging::tracing;
use hartex_cmdsys::framework::CommandFramework;
use hartex_eventsys::{
    emitter::EventEmitter,
    events::Events
};

/// # Function `framework_setup`
///
/// Sets up the command framework.
#[must_use]
pub fn framework_setup() -> (EventEmitter, Events) {
    tracing::trace!("setting up command framework");

    let framework = CommandFramework::default();

    let listeners = framework.clone().listeners();

    (EventEmitter::new(listeners), framework.events())
}
