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

use tower_http::classify::MakeClassifier;
use tower_layer::Layer;

use crate::log4rs::make_metadata::DefaultMakeMetadata;
use crate::log4rs::Log4rs;

#[derive(Clone, Copy, Debug)]
pub struct Log4rsLayer<M,
    MakeMetadataT = DefaultMakeMetadata,
> {
    pub(crate) make_classifier: M,
    pub(crate) make_metadata: MakeMetadataT,
}

impl<M> Log4rsLayer<M> {
    pub fn new(make_classifier: M) -> Self
        where
            M: MakeClassifier,
    {
        Self {
            make_classifier,
            make_metadata: DefaultMakeMetadata::new(),
        }
    }
}

impl<S, M, MakeMetadataT> Layer<S> for Log4rsLayer<M, MakeMetadataT>
where
    M: Clone,
    MakeMetadataT: Clone,
{
    type Service = Log4rs<S, M, MakeMetadataT>;

    fn layer(&self, inner: S) -> Self::Service {
        Log4rs {
            inner,
            make_classifier: self.make_classifier.clone(),
            make_metadata: self.make_metadata.clone(),
        }
    }
}