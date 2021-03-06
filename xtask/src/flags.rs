xflags::xflags! {
    src "./src/flags.rs"

    cmd xtask {
        /// How to use
        default cmd help {}

        /// Generate bindings with `bindgen`
        cmd bindgen {}
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Help(Help),
    Bindgen(Bindgen),
}

#[derive(Debug)]
pub struct Help;

#[derive(Debug)]
pub struct Bindgen;

impl Xtask {
    pub const HELP: &'static str = Self::HELP_;

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
