#[allow(dead_code)]

/// This enum holds information about whether there is a debugger present
pub enum DebugStatus {
    /// The PEB flag indicating the presence of a debugger is set. The presence of a debugger is highly likely
    BeingDebuggedPeb,

    /// A call to [IsDebuggerPresent](winapi::um::debugapi::IsDebuggerPresent) indicates the presence of a debugger
    IsDebuggerPresent,

    /// The [NtGlobalFlag](https://www.aldeid.com/wiki/PEB-Process-Environment-Block/NtGlobalFlag) of the PEB indicates the presence of a debugger
    NtGlobalFlagPeb,

    /// A call to [CheckRemoteDebuggerPresent](winapi::um::debugapi::CheckRemoteDebuggerPresent) indicates the presence of a remote debugger
    RemoteDebuggerPresent,

    /// An open GUI program's name matches a known debugger. This likely means that a debugger is present 
    FoundOpenWindow,

    /// A running process' name matches a known debugger. This likely means that a debugger is present
    DebuggerProcessFilename,

    /// No debugger was found
    None
}

