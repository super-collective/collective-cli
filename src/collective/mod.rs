// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>

pub mod fellowship;
pub mod potoc;

/// Extract the inner enum variant and make it available to an expression.
#[macro_export]
macro_rules! using_collective {
	( $c:expr, $inner:ident, $e:expr ) => {
		match $c {
			Self::Fellowship($inner) => $e,
			Self::Potoc($inner) => $e,
		}
	};
}
