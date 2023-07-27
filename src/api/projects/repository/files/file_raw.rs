// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use derive_builder::Builder;

use crate::api::common::{self, NameOrId};
use crate::api::endpoint_prelude::*;

/// Get raw file from repository.
///
/// Note: This endpoint returns raw data, so [`crate::api::raw`] is recommended to avoid the normal
/// JSON parsing present in the typical endpoint handling.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct FileRaw<'a> {
    /// The project to get a file within.
    #[builder(setter(into))]
    project: NameOrId<'a>,
    /// The path to the file in the repository.
    ///
    /// This is automatically escaped as needed.
    #[builder(setter(into))]
    file_path: Cow<'a, str>,
    /// The ref to get a file from.
    #[builder(setter(into), default)]
    ref_: Option<Cow<'a, str>>,
    /// If `true`, resolve LFS pointers to their backing data.
    ///
    /// If the file is not an LFS pointer it is ignored.
    #[builder(default)]
    lfs: Option<bool>,
}

impl<'a> FileRaw<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> FileRawBuilder<'a> {
        FileRawBuilder::default()
    }
}

impl<'a> Endpoint for FileRaw<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/repository/files/{}/raw",
            self.project,
            common::path_escaped(&self.file_path),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("ref", self.ref_.as_ref())
            .push_opt("lfs", self.lfs);

        params
    }
}

#[cfg(test)]
mod tests {
    use http::Method;

    use crate::api::projects::repository::files::{FileRaw, FileRawBuilderError};
    use crate::api::{self, Query};
    use crate::test::client::{ExpectedUrl, SingleTestClient};

    #[test]
    fn all_parameters_are_needed() {
        let err = FileRaw::builder().build().unwrap_err();
        crate::test::assert_missing_field!(err, FileRawBuilderError, "project");
    }

    #[test]
    fn project_is_required() {
        let err = FileRaw::builder()
            .file_path("new/file")
            .build()
            .unwrap_err();
        crate::test::assert_missing_field!(err, FileRawBuilderError, "project");
    }

    #[test]
    fn file_path_is_required() {
        let err = FileRaw::builder().project(1).build().unwrap_err();
        crate::test::assert_missing_field!(err, FileRawBuilderError, "file_path");
    }

    #[test]
    fn sufficient_parameters() {
        FileRaw::builder()
            .project(1)
            .file_path("new/file")
            .build()
            .unwrap();
    }

    #[test]
    fn endpoint() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile/raw")
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = FileRaw::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_ref() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile/raw")
            .add_query_params(&[("ref", "branch")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = FileRaw::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .ref_("branch")
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }

    #[test]
    fn endpoint_lfs() {
        let endpoint = ExpectedUrl::builder()
            .method(Method::GET)
            .endpoint("projects/simple%2Fproject/repository/files/path%2Fto%2Ffile/raw")
            .add_query_params(&[("lfs", "true")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");

        let endpoint = FileRaw::builder()
            .project("simple/project")
            .file_path("path/to/file")
            .lfs(true)
            .build()
            .unwrap();
        api::ignore(endpoint).query(&client).unwrap();
    }
}
