use eyre::{eyre, Result, WrapErr};
use std::{fs::File, path::PathBuf, process::Command};

pub struct RunQemuRiscV {
    pub opensbi: PathBuf,
    pub seed: PathBuf,
    pub kernel: PathBuf,
    pub disk_image: Option<PathBuf>,

    pub open_display: bool,
    pub debug_int_firehose: bool,
}

impl RunQemuRiscV {
    pub fn new(seed: PathBuf, kernel: PathBuf, disk_image: Option<PathBuf>) -> RunQemuRiscV {
        RunQemuRiscV {
            opensbi: PathBuf::from("lib/opensbi/build/platform/generic/firmware/fw_jump.elf"),
            seed,
            kernel,
            disk_image,

            open_display: false,
            debug_int_firehose: false,
        }
    }

    pub fn opensbi(self, opensbi: PathBuf) -> Self {
        Self { opensbi, ..self }
    }

    pub fn open_display(self, open_display: bool) -> Self {
        Self { open_display, ..self }
    }

    pub fn debug_int_firehose(self, enabled: bool) -> Self {
        Self { debug_int_firehose: enabled, ..self }
    }

    pub fn run(self) -> Result<()> {
        let mut qemu = Command::new("qemu-system-riscv64");

        qemu.args(&["-M", "virt"]);
        qemu.args(&["-m", "1G"]);
        qemu.args(&["-bios", self.opensbi.to_str().unwrap()]);
        qemu.args(&["-kernel", self.seed.to_str().unwrap()]);
        if self.debug_int_firehose {
            qemu.args(&["-d", "int"]);
        }
        // qemu.args(&["-fw_cfg", &format!("opt/poplar.kernel,file={}", self.kernel.to_str().unwrap())]);
        let kernel_size =
            File::open(self.kernel.clone()).expect("Failed to open kernel ELF").metadata().unwrap().len();
        qemu.args(&["-device", &format!("loader,addr=0xb0000000,data={},data-len=4", kernel_size)]);
        // TODO: get rid of this and the infra once we can load the kernel from the disk image
        qemu.args(&[
            "-device",
            &format!("loader,file={},addr=0xb0000004,force-raw=on", self.kernel.to_str().unwrap()),
        ]);

        // Emit serial on both stdio and to a file
        qemu.args(&["-chardev", "stdio,id=char0,logfile=qemu_serial_riscv.log"]);
        qemu.args(&["-serial", "chardev:char0"]);

        if let Some(disk_image) = self.disk_image {
            // Add the disk image as an NVME device
            qemu.args(&[
                "-drive",
                &format!("id=disk0,format=raw,if=none,file=fat:rw:{}", disk_image.to_str().unwrap()),
            ]);
            qemu.args(&["-device", "nvme,serial=deadbeef,drive=disk0"]);
        }

        if !self.open_display {
            qemu.args(&["-display", "none"]);
            // If we're not opening a display, allow connections to the monitor over TCP (open with `nc 127.0.0.1 55555`)
            qemu.args(&["-monitor", "tcp:127.0.0.1:55555,server,nowait"]);
        }

        println!("QEMU command: {:?}", qemu);
        qemu.status()
            .wrap_err("Failed to invoke qemu-system-riscv")?
            .success()
            .then_some(())
            .ok_or(eyre!("Qemu returned an error code"))
    }
}
