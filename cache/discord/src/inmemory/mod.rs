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

//! # The `inmemory` Module
//!
//! This module contains the in-memory backend for the cache.

use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    },
    marker::PhantomData,
    sync::{
        Arc,
        Mutex
    }
};

use dashmap::{
    DashMap,
    DashSet
};
use hartex_base::discord::model::{
    channel::message::sticker::{
        StickerId,
        StickerPackId
    },
    id::{
        AttachmentId,
        ChannelId,
        EmojiId,
        GuildId,
        MessageId,
        RoleId,
        UserId
    }
};
use hartex_cache_base::{
    backend::Backend,
    entity::Entity
};

use crate::{
    backend::DiscordBackend,
    entities::{
        channel::{
            attachment::AttachmentEntity,
            message::{
                sticker::{
                    StickerEntity,
                    StickerPackEntity
                },
                MessageEntity
            },
            thread::ThreadEntity,
            ChannelEntity
        },
        gateway::presence::PresenceEntity,
        guild::{
            emoji::EmojiEntity,
            member::MemberEntity,
            role::RoleEntity,
            GuildEntity
        },
        user::{
            current_user::CurrentUserEntity,
            UserEntity
        }
    },
    inmemory::repository::InMemoryRepository
};

pub mod repository;

/// # Struct `InMemoryBackend`
///
/// In-memory backend for the cache.
#[derive(Clone)]
pub struct InMemoryBackend(Arc<InMemoryBackendRef>);

impl InMemoryBackend {
    fn repository<T: Entity>(&self) -> InMemoryRepository<T> {
        InMemoryRepository(self.clone(), PhantomData)
    }
}

impl Backend for InMemoryBackend {
    type Error = InMemoryBackendError;
}

impl DiscordBackend for InMemoryBackend {
    type AttachmentRepository = InMemoryRepository<AttachmentEntity>;
    type ChannelRepository = InMemoryRepository<ChannelEntity>;
    type CurrentUserRepository = InMemoryRepository<CurrentUserEntity>;
    type EmojiRepository = InMemoryRepository<EmojiEntity>;
    type GuildRepository = InMemoryRepository<GuildEntity>;
    type MemberRepository = InMemoryRepository<MemberEntity>;
    type MessageRepository = InMemoryRepository<MessageEntity>;
    type PresenceRepository = InMemoryRepository<PresenceEntity>;
    type RoleRepository = InMemoryRepository<RoleEntity>;
    type UserRepository = InMemoryRepository<UserEntity>;
    type StickerRepository = InMemoryRepository<StickerEntity>;
    type StickerPackRepository = InMemoryRepository<StickerPackEntity>;
    type ThreadRepository = InMemoryRepository<ThreadEntity>;

    fn attachments(&self) -> Self::AttachmentRepository {
        self.repository::<AttachmentEntity>()
    }

    fn channels(&self) -> Self::ChannelRepository {
        self.repository::<ChannelEntity>()
    }

    fn current_user(&self) -> Self::CurrentUserRepository {
        self.repository::<CurrentUserEntity>()
    }

    fn emojis(&self) -> Self::EmojiRepository {
        self.repository::<EmojiEntity>()
    }

    fn guilds(&self) -> Self::GuildRepository {
        self.repository::<GuildEntity>()
    }

    fn members(&self) -> Self::MemberRepository {
        self.repository::<MemberEntity>()
    }

    fn messages(&self) -> Self::MessageRepository {
        self.repository::<MessageEntity>()
    }

    fn presences(&self) -> Self::PresenceRepository {
        self.repository::<PresenceEntity>()
    }

    fn roles(&self) -> Self::RoleRepository {
        self.repository::<RoleEntity>()
    }

    fn users(&self) -> Self::UserRepository {
        self.repository::<UserEntity>()
    }

    fn sticker_packs(&self) -> Self::StickerPackRepository {
        self.repository::<StickerPackEntity>()
    }

    fn stickers(&self) -> Self::StickerRepository {
        self.repository::<StickerEntity>()
    }

    fn threads(&self) -> Self::ThreadRepository {
        self.repository::<ThreadEntity>()
    }
}

/// # Struct `InMemoryBackendError`
///
/// Error returned from backend operations.
#[derive(Clone, Debug)]
pub struct InMemoryBackendError;

impl Display for InMemoryBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "this error cannot be created")
    }
}

impl Error for InMemoryBackendError {}

struct InMemoryBackendRef {
    attachments: DashMap<(MessageId, AttachmentId), AttachmentEntity>,
    channels: DashMap<ChannelId, ChannelEntity>,
    current_user: Mutex<Option<CurrentUserEntity>>,
    emojis: DashMap<EmojiId, EmojiEntity>,
    guilds: DashMap<GuildId, GuildEntity>,
    guild_emojis: DashMap<GuildId, DashSet<EmojiId>>,
    guild_members: DashMap<GuildId, DashSet<UserId>>,
    guild_roles: DashMap<GuildId, DashSet<RoleId>>,
    members: DashMap<(GuildId, UserId), MemberEntity>,
    messages: DashMap<MessageId, MessageEntity>,
    presences: DashMap<(GuildId, UserId), PresenceEntity>,
    roles: DashMap<RoleId, RoleEntity>,
    users: DashMap<UserId, UserEntity>,
    sticker_packs: DashMap<StickerPackId, StickerPackEntity>,
    stickers: DashMap<StickerId, StickerEntity>,
    threads: DashMap<ChannelId, ThreadEntity>
}
