// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::NameOrId;
use crate::api::endpoint_prelude::*;

/// Query for labels within a project.
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Labels<'a> {
    /// The project to query for labels.
    #[builder(setter(into))]
    project: NameOrId<'a>,

    /// Include issue and merge request counts..
    #[builder(default)]
    with_counts: Option<bool>,
    /// Include ancestor groups.
    ///
    /// Defaults to `true`.
    #[builder(default)]
    include_ancestor_groups: Option<bool>,
}

impl<'a> Labels<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> LabelsBuilder<'a> {
        LabelsBuilder::default()
    }
}

impl<'a> Endpoint for Labels<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/labels", self.project).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("with_counts", self.with_counts)
            .push_opt("include_ancestor_groups", self.include_ancestor_groups);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::projects::labels::Labels;
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn project_is_needed() {
        let err = Labels::builder().build().unwrap_err();
        assert_eq!(err, "`project` must be initialized");
    }

    #[test]
    fn project_is_sufficient() {
        Labels::builder().project(1).build().unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Labels::builder().project("simple/project").build().unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_with_counts() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels")
            .add_query_params(&[("with_counts", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Labels::builder()
            .project("simple/project")
            .with_counts(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_include_ancestor_groups() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("projects/simple%2Fproject/labels")
            .add_query_params(&[("include_ancestor_groups", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = Labels::builder()
            .project("simple/project")
            .include_ancestor_groups(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
