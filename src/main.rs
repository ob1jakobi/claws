use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::Command;
use crate::claws::Claw;
#[derive(Debug)]
struct ClawError {
    msg: String,
}

impl Display for ClawError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClawError: {}", self.msg)
    }
}

impl Error for ClawError {}

pub mod claws {
    use std::fs;
    use std::path::PathBuf;
    use cursive::{Cursive, CursiveExt};
    use cursive::traits::*;
    use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView};

    const LOGO: &str = "

 ██████╗██╗      █████╗ ██╗    ██╗███████╗
██╔════╝██║     ██╔══██╗██║    ██║██╔════╝
██║     ██║     ███████║██║ █╗ ██║███████╗
██║     ██║     ██╔══██║██║███╗██║╚════██║
╚██████╗███████╗██║  ██║╚███╔███╔╝███████║
 ╚═════╝╚══════╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝

    ";


    /// The TUI facilitated by the Claw.
    pub struct Claw {
        siv: Cursive,
    }

    impl Claw {
        /// Creates a new Claw
        pub fn new() -> Self {
            Self::claw_builder()
        }
        /// Helper function for building Claw objects
        fn claw_builder() -> Claw {
            Claw {
                siv: Cursive::default()
            }
        }

        pub fn main_menu(&mut self) {
            let capture_items: Vec<(&str, &str)> = vec![
                ("eth0", "eth0"),
                ("eth1", "eth1"),
                ("any", "any"),
                ("Loopback", "Loopback"),
            ];
            let mut s: &mut Cursive = &mut self.siv;

            let logo_view = TextView::new(LOGO);
            let mut cap_from_file_sel_view = SelectView::<PathBuf>::new()
                .on_submit(Self::read_file);

            let mut new_capture_sel_view = SelectView::<&str>::new()
                .on_submit(Self::new_capture);
            new_capture_sel_view.add_all(capture_items);

            if let Some(cap_files) = Self::get_pcap_files() {
                for (f_name, path) in cap_files.into_iter() {
                    cap_from_file_sel_view.add_item(f_name, path);
                }
            } else {
                cap_from_file_sel_view.add_item("No Packet Capture Files Available", PathBuf::default());
            }

            let main_layout = LinearLayout::vertical()
                .child(logo_view)
                .child(cap_from_file_sel_view)
                .child(DummyView)
                .child(new_capture_sel_view)
                .child(DummyView)
                .child(Button::new("Quit", Cursive::quit));
            s.add_layer(main_layout);
            s.run();
        }

        /// Finds `pcap` and `cap` files and returns a vector where each element consists of a
        /// tuple containing the file's name and the path to the file.
        fn get_pcap_files() -> Option<Vec<(String, PathBuf)>> {
            std::env::current_dir()
                .ok()
                .and_then(|asset_dir| {
                    fs::read_dir(&asset_dir)
                        .ok()
                        .map(|dir| {
                            dir.filter_map(|entry| {
                                entry.ok().and_then(|e| {
                                    if e.file_type().map_or(false, |ft| ft.is_file()) {
                                        e.file_name()
                                            .to_str()
                                            .filter(|file_name| file_name.ends_with("pcap") || file_name.ends_with("cap"))
                                            .map(|file_name| (file_name.to_string(), e.path()))
                                    } else {
                                        None
                                    }
                                })
                            })
                                .collect::<Vec<(String, PathBuf)>>()
                        })
                })
                .filter(|n_p| !n_p.is_empty())
        }

        /// Takes the file that's provided and parses the contents of the packet capture file.
        fn read_file(s: &mut Cursive, file: &PathBuf) {
            let msg: String = format!("You've chosen the read_file function with path: {:?}", file);
            s.add_layer(Dialog::info(msg))
        }

        /// Makes a new capture on a given interface.
        fn new_capture(s: &mut Cursive, capture_type: &str) {
            let msg: String = format!("You've chosen the new_capture function on capture type: {}", capture_type);
            s.add_layer(Dialog::info(msg))
        }
    }
}

fn main() {
    //install_dependencies();
    let mut claw: Claw = Claw::new();
    claw.main_menu();
}

/// Installs the dependencies for the project via command execution.
fn _install_dependencies() {
    let program_names: Vec<&str> = vec![
        "libpcap-dev",
    ];
    program_names.iter().for_each(|program| {
        if !is_program_installed(program) {
            match install_program(program) {
                Ok(_) => println!("Dependencies '{}' installed", program),
                Err(e) => eprintln!("{}", e.msg),
            }
        } else {
            println!("Program '{}' already installed...", program);
        }
    })
}

/// Checks to see if the program named `program_name` is installed on the machine.
fn is_program_installed(program_name: &str) -> bool {
    Command::new("which")
        .arg(program_name)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Installs the program via the `sudo` command using the `apt` program manager.
fn install_program(program_name: &str) -> Result<(), ClawError> {
    match Command::new("sudo").arg("apt").arg("install").arg("-y").arg(program_name).status() {
        Ok(_) => Ok(()),
        Err(e) => Err(ClawError {msg: format!("unable to install program '{}'\tError: {}", program_name, e)}),
    }
}