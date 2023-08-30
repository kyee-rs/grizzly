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
