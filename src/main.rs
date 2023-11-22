use crate::claws::Claw;

pub mod claws {
    use cursive::{Cursive, CursiveExt};
    use cursive::views::Dialog;

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
            s.add_layer(
                Dialog::text("Can you see this dialog window?")
                    .button("Yes", |s| {
                        s.add_layer(Dialog::info("YAY!"))
                    })
                    .button("No!", |s| {
                        s.add_layer(Dialog::info("Rut row..."))
                    })
                    .button("Quit", |s| s.quit())
            );
            s.run();
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
