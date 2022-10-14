/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

import type {AstroGlobal} from 'astro'

import type {TranslationDictionary} from './translationDictionary'
import {getLanguageFromUri} from '../language'

const translationDictonary = mapExports(import.meta.glob("./*/translations.ts", { eager: true }))

export async function getTranslations(astroGlobals: AstroGlobal): Promise<TranslationDictionary> {
    const language = getLanguageFromUri(astroGlobals.url.pathname) || "en"
    return await buildContents(language, translationDictonary[language])
}

function mapExports<T>(modules: Record<string, { default: T }>) {
    const exportMap: Record<string, T> = {}
    for (const [path, module] of Object.entries(modules)) {
        const [_, lang] = path.split('/')
        exportMap[lang] = module.default
    }

    return exportMap
}

async function buildContents(language: string, translationDictionary: TranslationDictionary) {
    throw Error("todo")
}
