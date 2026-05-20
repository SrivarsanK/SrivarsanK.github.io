//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

mod boot_sequence;
pub use boot_sequence::BootSequence;

mod login_screen;
pub use login_screen::LoginScreen;

mod taskbar;
pub use taskbar::Taskbar;

mod desktop_icons;
pub use desktop_icons::DesktopIcons;

mod terminal;
pub use terminal::Terminal;
