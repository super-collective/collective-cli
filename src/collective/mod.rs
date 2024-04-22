pub mod fellowship;
pub mod potoc;

#[macro_export]
macro_rules! using_collective {
	( $c:expr, $inner:ident, $e:expr ) => {
		match $c {
			Self::Fellowship($inner) => $e,
			Self::Potoc($inner) => $e,
		}
	};
}
