use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunParams {
    /// The build target to run.
    pub target: BuildTargetIdentifier,
    /// A unique identifier generated by the client to identify this request.
    /// The server may include this id in triggered notifications or responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// Optional arguments to the executed application.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
    /// Language-specific metadata for this execution.
    /// See ScalaMainClass as an example.
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<RunParamsData>,
}