use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskFinishParams {
    /// Unique id of the task with optional reference to parent task id
    pub task_id: TaskId,
    /// A unique identifier generated by the client to identify this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// Timestamp of when the event started in milliseconds since Epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i64>,
    /// Message describing the task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Task completion status.
    pub status: StatusCode,
    /// Optional metadata about the task.
    /// Objects for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskFinishData>,
}