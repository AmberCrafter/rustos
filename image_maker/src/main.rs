use std::{
    path::{Path, PathBuf},
    process::Command,
};

// -nographic can't combine with -serial stdio, need to use -serial mon:stdio

// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-nographic", "-monitor", "telnet::45454,server,nowait", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-nographic", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s"];
// const RUN_ARGS: &[&str] = &["--no-reboot"];

// This use on non-UI environment
const RUN_ARGS: &[&str] = &["--no-reboot", "-display", "none", "-serial", "stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-display", "none", "-serial", "stdio"];
// const RUN_ARGS: &[&str] = &["--no-reboot", "-s", "-display", "none", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["-d", "int", "--no-reboot", "-s", "-display", "none", "-serial", "mon:stdio"];
// const RUN_ARGS: &[&str] = &["-d", "int", "--no-reboot", "-s", "-S", "-display", "none", "-serial", "mon:stdio"];

fn main() {
    let mut args = std::env::args().skip(1); // skip executable name

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };
    let no_boot = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--no-run" => true,
            other => panic!("unexpected argument `{}`", other),
        }
    } else {
        false
    };

    let bios = create_disk_images(&kernel_binary_path);

    if no_boot {
        println!("Created disk image at `{}`", bios.display());
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    run_cmd
        .arg("-drive")
        .arg(format!("format=raw,file={}", bios.display()));
    run_cmd.args(RUN_ARGS);

    println!("{:?}", run_cmd.get_args());

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(1));
    }
}

pub fn create_disk_images(kernel_binary_path: &Path) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));
    if !disk_image.exists() {
        panic!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        );
    }
    disk_image
}
