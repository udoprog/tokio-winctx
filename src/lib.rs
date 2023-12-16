//! [<img alt="github" src="https://img.shields.io/badge/github-udoprog/winctx-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/udoprog/winctx)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/winctx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/winctx)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-winctx-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/winctx)
//!
//! A minimally viable Windows context for Rust.
//!
//! This allows non-gui applications to:
//! * [Register and use a context menu, the icons you see in the bottom right
//!   for running applications][showcase].
//! * [Send notifcations, or balloons as Windows call them][showcase].
//! * [Monitor the clipboard for changes][clipboard].
//! * [Access the registry][registry].
//! * [Copy data to the current process remotely][copy-data].
//!
//! Note that crate is fairly opinionated, not everything that is possible
//! through the underlying APIs will be exposed.
//!
//! <br>
//!
//! ## Example
//!
//! ```no_run
//! use std::pin::pin;
//!
//! use tokio::signal::ctrl_c;
//! use winctx::{Event, Notification, ContextBuilder, MenuItem};
//!
//! # macro_rules! include_bytes { ($path:literal) => { &[] } }
//! const ICON: &[u8] = include_bytes!("tokio.ico");
//!
//! #[tokio::main]
//! async fn main() -> winctx::Result<()> {
//!     let mut builder = ContextBuilder::new("Example Application");
//!     builder.set_icon(ICON, 22, 22);
//!
//!     builder.push_menu_item(MenuItem::entry("Hello World", true));
//!     let notification = builder.push_menu_item(MenuItem::entry("Show notification", false));
//!     let notification_multiple = builder.push_menu_item(MenuItem::entry("Show multiple notifications", false));
//!     builder.push_menu_item(MenuItem::separator());
//!     let quit = builder.push_menu_item(MenuItem::entry("Quit", false));
//!
//!     let (sender, mut event_loop) = builder.build().await?;
//!
//!     let mut ctrl_c = pin!(ctrl_c());
//!     let mut shutdown = false;
//!
//!     loop {
//!         let event = tokio::select! {
//!             _ = ctrl_c.as_mut(), if !shutdown => {
//!                 sender.shutdown();
//!                 shutdown = true;
//!                 continue;
//!             }
//!             event = event_loop.tick() => {
//!                 event?
//!             }
//!         };
//!
//!         match event {
//!             Event::MenuItemClicked(token) => {
//!                 println!("Menu entry clicked: {:?}", token);
//!
//!                 if token == notification {
//!                     sender.notification(
//!                         Notification::new("And this is a body").with_title("This is a title"),
//!                     );
//!                     continue;
//!                 }
//!
//!                 if token == notification_multiple {
//!                     sender.notification(Notification::new("First"));
//!                     sender.notification(Notification::new("Second"));
//!                     continue;
//!                 }
//!
//!                 if token == quit {
//!                     sender.shutdown();
//!                 }
//!             }
//!             Event::NotificationClicked(token) => {
//!                 println!("Balloon clicked: {:?}", token);
//!             }
//!             Event::NotificationDismissed(token) => {
//!                 println!("Notification dismissed: {:?}", token);
//!             }
//!             Event::Shutdown => {
//!                 println!("Window shut down");
//!                 break;
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [showcase]: https://github.com/udoprog/winctx/blob/main/examples/showcase.rs
//! [clipboard]: https://github.com/udoprog/winctx/blob/main/examples/clipboard.rs
//! [registry]: https://github.com/udoprog/winctx/blob/main/examples/registry.rs
//! [copy-data]: https://github.com/udoprog/winctx/blob/main/examples/copy_data.rs

#![allow(clippy::module_inception)]
#![deny(missing_docs)]

mod clipboard;
mod convert;

pub use self::registry::{OpenRegistryKey, RegistryKey};
mod registry;

pub use self::window::Window;
pub mod window;

mod window_loop;

pub use self::notification::Notification;
mod notification;

pub use self::error::Error;
mod error;

pub use self::token::Token;
mod token;

pub use self::event_loop::{ClipboardEvent, Event, EventLoop, Sender};
mod event_loop;

pub use self::context_builder::ContextBuilder;
mod context_builder;

pub use self::autostart::AutoStart;
mod autostart;

pub mod tools;

pub use self::named_mutex::NamedMutex;
mod named_mutex;

pub use self::menu_item::MenuItem;
pub(crate) mod menu_item;

/// Result alias for winctx.
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg_attr(windows, path = "windows/real.rs")]
#[cfg_attr(not(windows), path = "windows/fake.rs")]
mod windows;
