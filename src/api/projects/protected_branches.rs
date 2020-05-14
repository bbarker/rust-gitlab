// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Project repository protected branches API endpoints.
//!
//! These endpoints are used for querying a project's protected branches.

mod protect;
mod unprotect;

pub use self::protect::ProtectBranch;
pub use self::protect::ProtectBranchBuilder;
pub use self::protect::ProtectedAccess;
pub use self::protect::ProtectedAccessLevel;

pub use self::unprotect::UnprotectBranch;
pub use self::unprotect::UnprotectBranchBuilder;