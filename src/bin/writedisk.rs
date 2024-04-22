#![warn(clippy::pedantic)]

use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, process};
use sysinfo::{Disks, System};


#[cfg(any(target_os = "linux", target_os = "macos"))]
const RUN_ELEVATED : &str = "sudo";
#[cfg(target_os = "windows")]
const RUN_ELEVATED : &str = "runas /user:Administrator";

#[derive(Clone, Debug)]
struct UsbBlockDevice {
    mount_point: PathBuf,
    description: String,
    size: u64,
}

impl UsbBlockDevice {
    fn get_all() -> io::Result<Vec<UsbBlockDevice>> {
        let mut result = Vec::new();
        let mut sys = System::new_all();
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();

        //Get all (removable) disks and saves their mount points, along with a name and a size;
        for entry in &disks {
            // Skip non-removable devices
            if entry.is_removable() {
                result.push(UsbBlockDevice {
                    mount_point: entry.mount_point().to_path_buf(),
                    description: entry.name().to_string_lossy().into_owned(),
                    size: entry.total_space(),
                });
            }
        }
        
        Ok(result)
    }

    // It's ugly but it works.
    fn size_display(&self) -> String {
        let size = self.size;
        if size < 1_000_000_000 {
            format!("{} MB", size / 1_000_000)
        } else {
            format!("{} GB", size / 1_000_000_000)
        }
    }

    fn summary(&self) -> String {
        format!(
            "[{}] {} {}",
            self.mount_point.display(),
            self.description,
            self.size_display(),
        )
    }
}

fn choose_device() -> UsbBlockDevice {
    let devices = UsbBlockDevice::get_all().unwrap();

    if devices.is_empty() {
        println!("no devices found");
        process::exit(1);
    }

    for (index, device) in devices.iter().enumerate() {
        println!("{index}: {}", device.summary());
    }

    print!("select device: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index = match input.trim().parse::<usize>() {
        Ok(i) => i,
        Err(_e) => {
            println!("invalid input");
            process::exit(1);
        }
    };

    if index >= devices.len() {
        println!("invalid index");
        process::exit(1);
    }

    devices[index].clone()
}

#[derive(Debug, Parser)]
#[command(about = "Write a disk image to a USB disk.", version)]
struct Opt {
    /// Disk image
    input: PathBuf,
}

fn main() {
    let opt = Opt::parse();

    // Check if the input file exists before doing anything else.
    if !opt.input.exists() {
        eprintln!("file not found: {}", opt.input.display());
        process::exit(1);
    }

    let device = choose_device();

    let copier_path = env::current_exe()
        .expect("failed to get current exe path")
        .parent()
        .expect("failed to get current exe directory")
        .join("wd_copier");

    println!(
        "sudo {} {} {}",
        copier_path.display(),
        opt.input.display(),
        device.mount_point.display()
    );
    let status = process::Command::new(RUN_ELEVATED)
        .args([&copier_path, &opt.input, &device.mount_point])
        .status()
        .expect("failed to run command");
    if !status.success() {
        println!("copy failed");
        process::exit(1);
    }
}
