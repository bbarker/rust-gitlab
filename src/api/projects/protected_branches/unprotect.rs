// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Create a new file in a project.
#[derive(Debug, Builder)]
pub struct UnprotectBranch<'a> {
    /// The project to create a file within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The name or glob of the branch to unprotect.
    #[builder(setter(into))]
    name: Cow<'a, str>,
}

impl<'a> UnprotectBranch<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnprotectBranchBuilder<'a> {
        UnprotectBranchBuilder::default()
    }
}

impl<'a> Endpoint for UnprotectBranch<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/protected_branches/{}",
            self.project,
            common::path_escaped(&self.name),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::protected_branches::UnprotectBranch;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_and_name_are_needed() {
        let err = UnprotectBranch::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_required() {
        let err = UnprotectBranch::builder()
            .name("master")
            .build()
            .unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn name_is_required() {
        let err = UnprotectBranch::builder().project(1).build().unwrap_err();
        assert_eq!(err, "`name` must be initialized");
    }

    #[test]
    fn project_and_name_are_sufficient() {
        UnprotectBranch::builder()
            .project(1)
            .name("master")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::DELETE)
            .endpoint("projects/simple%2Fproject/protected_branches/branch%2Fname")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = UnprotectBranch::builder()
            .project("simple/project")
            .name("branch/name")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
