// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

mod collective;
mod evidence;
mod join_request;
mod member;
mod traits;
mod wish;

pub mod prelude {
	pub use super::{collective::*, evidence::*, join_request::*, member::*, traits::*, wish::*};
}
