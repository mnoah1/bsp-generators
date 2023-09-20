use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmEnvironmentItem {
    pub target: BuildTargetIdentifier,
    pub classpath: Vec<String>,
    pub jvm_options: Vec<String>,
    pub working_directory: String,
    pub environment_variables: EnvironmentVariables,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_classes: Vec<JvmMainClass>,
}