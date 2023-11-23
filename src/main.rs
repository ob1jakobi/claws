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
                for e in pcap_files.into_iter() {
                    let f_name = e.file_name().unwrap().to_str().unwrap_or("BadName").to_string();
                    sel_view.add_item(f_name, e);
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

        fn get_pcap_files() -> Option<Vec<PathBuf>> {
            //TODO
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
