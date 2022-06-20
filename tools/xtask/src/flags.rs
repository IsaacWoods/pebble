use std::convert::TryFrom;

xflags::xflags! {
    src "./src/flags.rs"

    cmd task {
        default cmd help {
            optional -h,--help
        }

        cmd dist {
            optional --release
            optional -a,--arch arch: String
            optional --kernel_features kernel_features: String
        }

        cmd qemu {
            // XXX: shared with dist command. Should be the same.
            optional --release
            optional -a,--arch arch: String
            optional --kernel_features kernel_features: String

            optional --display
            optional --debug_int_firehose
            optional --debug_mmu_firehose
            optional --debug_cpu_firehose
        }

        cmd clean {}
    }
}

#[allow(dead_code)]
pub enum Arch {
    X64,
    RiscV,
}

impl TryFrom<&String> for Arch {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "x64" => Ok(Arch::X64),
            "riscv" => Ok(Arch::RiscV),
            _ => Err("Unrecognised arch string. Accepted values are `x64` and `riscv`."),
        }
    }
}

const DEFAULT_ARCH: Arch = Arch::RiscV;

pub struct DistOptions {
    // TODO: method to set persistent default and control this from flags
    pub arch: Arch,
    pub release: bool,
    pub kernel_features: Option<String>,
}

impl From<&Dist> for DistOptions {
    fn from(flags: &Dist) -> DistOptions {
        DistOptions {
            release: flags.release,
            kernel_features: flags.kernel_features.clone(),
            arch: flags.arch.as_ref().map(|s| Arch::try_from(s).unwrap()).unwrap_or(DEFAULT_ARCH),
        }
    }
}

impl From<&Qemu> for DistOptions {
    fn from(flags: &Qemu) -> DistOptions {
        DistOptions {
            release: flags.release,
            kernel_features: flags.kernel_features.clone(),
            arch: flags.arch.as_ref().map(|s| Arch::try_from(s).unwrap()).unwrap_or(DEFAULT_ARCH),
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct Task {
    pub subcommand: TaskCmd,
}

#[derive(Debug)]
pub enum TaskCmd {
    Help(Help),
    Dist(Dist),
    Qemu(Qemu),
    Clean(Clean),
}

#[derive(Debug)]
pub struct Help {
    pub help: bool,
}

#[derive(Debug)]
pub struct Dist {
    pub release: bool,
    pub arch: Option<String>,
    pub kernel_features: Option<String>,
}

#[derive(Debug)]
pub struct Qemu {
    pub release: bool,
    pub arch: Option<String>,
    pub kernel_features: Option<String>,
    pub display: bool,
    pub debug_int_firehose: bool,
    pub debug_mmu_firehose: bool,
    pub debug_cpu_firehose: bool,
}

#[derive(Debug)]
pub struct Clean;

impl Task {
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
