// Copyright 2018 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! External Error trait for extrinsic pool.

use txpool;

/// Extrinsic pool error.
pub trait IntoPoolError: ::std::error::Error + Send + Sized {
	/// Try to extract original `txpool::Error`
	///
	/// This implementation is optional and used only to
	/// provide more descriptive error messages for end users
	/// of RPC API.
	fn into_pool_error(self) -> Result<txpool::Error, Self> { Err(self) }
}

impl IntoPoolError for txpool::Error {
	fn into_pool_error(self) -> Result<txpool::Error, Self> { Ok(self) }
}
