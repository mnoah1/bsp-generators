use crate::*;

#[derive(Debug)]
pub enum OnBuildShowMessage {}

/// The show message notification is sent from a server to a client to ask the client to display a particular message in the user interface.
///
/// A build/showMessage notification is similar to LSP's window/showMessage, except for a few additions like id and originId.
impl Notification for OnBuildShowMessage {
    type Params = ShowMessageParams;
    const METHOD: &'static str = "build/showMessage";
}

#[derive(Debug)]
pub enum OnBuildLogMessage {}

/// The log message notification is sent from a server to a client to ask the client to log a particular message in its console.
///
/// A build/logMessage notification is similar to LSP's window/logMessage, except for a few additions like id and originId.
impl Notification for OnBuildLogMessage {
    type Params = LogMessageParams;
    const METHOD: &'static str = "build/logMessage";
}

#[derive(Debug)]
pub enum OnBuildPublishDiagnostics {}

/// The Diagnostics notification are sent from the server to the client to signal results of validation runs.
///
/// Diagnostic is defined as it is in the LSP.
///
/// When reset is true, the client must clean all previous diagnostics associated with the same textDocument and
/// buildTarget and set instead the diagnostics in the request. This is the same behaviour as PublishDiagnosticsParams
/// in the LSP. When reset is false, the diagnostics are added to the last active diagnostics, allowing build tools to
/// stream diagnostics to the client.
///
/// It is the server's responsibility to manage the lifetime of the diagnostics by using the appropriate value in the reset field.
/// Clients generate new diagnostics by calling any BSP endpoint that triggers a buildTarget/compile, such as buildTarget/compile, buildTarget/test and buildTarget/run.
///
/// If the computed set of diagnostic is empty, the server must push an empty array with reset set to true, in order to clear previous diagnostics.
///
/// The optional originId field in the definition of PublishDiagnosticsParams can be used by clients to know which request originated the notification.
/// This field will be defined if the client defined it in the original request that triggered this notification.
impl Notification for OnBuildPublishDiagnostics {
    type Params = PublishDiagnosticsParams;
    const METHOD: &'static str = "build/publishDiagnostics";
}

#[derive(Debug)]
pub enum OnBuildTargetDidChange {}

/// The build target changed notification is sent from the server to the client to
/// signal a change in a build target. The server communicates during the initialize
/// handshake whether this method is supported or not.
impl Notification for OnBuildTargetDidChange {
    type Params = DidChangeBuildTarget;
    const METHOD: &'static str = "buildTarget/didChange";
}

#[derive(Debug)]
pub enum OnBuildTaskStart {}

/// The BSP server can inform the client on the execution state of any task in the
/// build tool. The execution of some tasks, such as compilation or tests, must
/// always be reported by the server.
///
/// The server may also send additional task notifications for actions not covered
/// by the protocol, such as resolution or packaging. BSP clients can then display
/// this information to their users at their discretion.
///
/// When beginning a task, the server may send `build/taskStart`, intermediate
/// updates may be sent in `build/taskProgress`.
///
/// If a `build/taskStart` notification has been sent, the server must send
/// `build/taskFinish` on completion of the same task.
///
/// `build/taskStart`, `build/taskProgress` and `build/taskFinish` notifications for
/// the same task must use the same `taskId`.
///
/// Tasks that are spawned by another task should reference the originating task's
/// `taskId` in their own `taskId`'s `parent` field. Tasks spawned directly by a
/// request should reference the request's `originId` parent.
impl Notification for OnBuildTaskStart {
    type Params = TaskStartParams;
    const METHOD: &'static str = "build/taskStart";
}

#[derive(Debug)]
pub enum OnBuildTaskProgress {}

/// After a `taskStart` and before `taskFinish` for a `taskId`, the server may send
/// any number of progress notifications.
impl Notification for OnBuildTaskProgress {
    type Params = TaskProgressParams;
    const METHOD: &'static str = "build/taskProgress";
}

#[derive(Debug)]
pub enum OnBuildTaskFinish {}

/// A `build/taskFinish` notification must always be sent after a `build/taskStart`
/// with the same `taskId` was sent.
impl Notification for OnBuildTaskFinish {
    type Params = TaskFinishParams;
    const METHOD: &'static str = "build/taskFinish";
}