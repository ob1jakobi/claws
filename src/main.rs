use crate::claws::Claw;

pub mod claws {
    use std::fs;
    use std::path::PathBuf;
    use cursive::{Cursive, CursiveExt};
    use cursive::views::{Dialog, DummyView, LinearLayout, SelectView};

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
            let mut s = &mut self.siv;

            if let Some(pcap_files) = Self::get_pcap_files() {
                let mut sel_view = SelectView::<PathBuf>::new();
                for (f_name, path) in pcap_files.into_iter() {
                    sel_view.add_item(f_name, path);
                }
                let main_layout = LinearLayout::vertical()
                    .child(sel_view)
                    .child(DummyView); // TODO: update with ability to run from select options

                s.add_layer(main_layout);
            } else {
                s.add_layer(
                    Dialog::info("No pcap Files Found")
                );
            }
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

        fn read_file(&mut self) {
            // TODO: Implement reading from file; possibly by issuing system commands to tcpdump -r
        }
    }
}

fn main() {
    let mut claw: Claw = Claw::new();
    claw.main_menu();
}
