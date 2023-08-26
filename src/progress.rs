use console::{style, Emoji};

pub struct Progress;

static FILE: Emoji<'_, '_> = Emoji("📁 ", "");
static ARCHIVE: Emoji<'_, '_> = Emoji("🗃️ ", "");
static COMPUTER: Emoji<'_, '_> = Emoji("💻 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");

/// `Progress` - Struct with methods for notifying the user of current bundling stage.
impl Progress {
    pub(crate) fn temp_storage_pg() {
        println!(
            "{} {}Creating temporary storage...",
            style("[1/4]").bold().dim(),
            FILE
        );
    }
    pub(crate) fn creating_tarball() {
        println!(
            "{} {}Creating a tarball...",
            style("[2/4]").bold().dim(),
            ARCHIVE
        );
    }
    pub(crate) fn generating_code_pg() {
        println!(
            "{} {}Generating the code...",
            style("[3/4]").bold().dim(),
            COMPUTER
        );
    }
    pub(crate) fn compile_pg() {
        println!(
            "{} {}Compiling the code...",
            style("[4/4]").bold().dim(),
            COMPUTER
        );
    }
    pub(crate) fn done_pg() {
        println!("{} {}Done!", style("READY").bold().dim(), SPARKLE);
    }
}
