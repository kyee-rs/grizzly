/* Copyright (C) 2023 Saputskyi Petro - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY-SA-4.0 license.
 *
 * ----------------------------------------------------------------------------------------------------
 * Commercial use - YES
 * Distribution - YES
 * Modification - YES
 * Private use - YES
 * ----------------------------------------------------------------------------------------------------
 * Liability - NO
 * Patent use - NO
 * Trademark use - NO
 * Warranty - NO
 * ----------------------------------------------------------------------------------------------------
 * A copy of the license and copyright notice must be included with the licensed material.
 * Modifications must be released under the same license when distributing the licensed material.
 * In some cases a similar or related license may be used.
 * Changes made to the licensed material must be documented.
 * ----------------------------------------------------------------------------------------------------
 *
 * You should have received a copy of the CC-BY-SA-4.0 license with
 * this file. If not, please write to: hello@lowt.live, or visit: https://github.com/12subnet
 */
use console::{style, Emoji};
use indicatif::HumanBytes;

pub struct Progress;

static FILE: Emoji<'_, '_> = Emoji("📁 ", "");
static ARCHIVE: Emoji<'_, '_> = Emoji("🗃️ ", "");
static COMPUTER: Emoji<'_, '_> = Emoji("💻 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

/// `Progress` - Struct with methods for notifying the user of current bundling stage.
impl Progress {
    pub(crate) fn allocating_space_pg() {
        println!(
            "{} {}Allocating space for a ZIP Archive...",
            style("[1/4]").bold().dim(),
            FILE
        );
    }
    pub(crate) fn creating_zip_pg() {
        println!(
            "{} {}Creating a zip-file...",
            style("[2/4]").bold().dim(),
            ARCHIVE
        );
    }
    pub(crate) fn insert_pg() {
        println!(
            "{} {}Inserting a zip-file inside your binary...",
            style("[3/4]").bold().dim(),
            COMPUTER
        );
    }
    pub(crate) fn zippo_pg() {
        println!(
            "{} {}Writing your binary using Zippo...",
            style("[4/4]").bold().dim(),
            COMPUTER
        );
    }
    pub(crate) fn done_pg(size: u64, size_binary: u64) {
        println!(
            "{} {}Successfully bundled. Archive size: {} ({} binary overhead).",
            style("DONE").bold().dim(),
            SPARKLE,
            HumanBytes(size),
            HumanBytes(size_binary)
        );
    }
}
