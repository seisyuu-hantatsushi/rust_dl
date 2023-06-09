#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cuda_sys::*;
use std::fmt::Display;

#[derive(Debug,PartialEq)]
pub enum CUDAError {
    Success,
    InvalidValue,
    OutOfMemory,
    NotInitalized,
    DeInitialized,
    ProfilerDisabled,
    ProfilerNotInitialized,
    ProfilerAlreadyStarted,
    ProfilerAlreadyStopped,
    StubLibrary,
    DeviceUnavailable,
    NoDevice,
    InvalidDevice,
    DeviceNotLicensed,
    InvalidImage,
    InvalidContext,
    ContextAlreadyCurrent,
    MapFailed,
    UnmapFailed,
    ArrayIsMapped,
    AlreadyMapped,
    NoBinaryForGPU,
    AlreadyAcquired,
    NotMapped,
    NotMappedAsArray,
    NotMappedAsPointer,
    EccUncorrectable,
    UnsupportedLimit,
    ContextAlreadyInUse,
    PeerAccessUnsuppoted,
    InvalidPtx,
    InvalidGraphicsContext,
    NvlinkUncorrectable,
    JitCompilerNotFound,
    UnsupportedPtxVersion,
    JitCompilationDisabled,
    UnsupportedExecAffinity,
    UnsupportedDevSideSync,
    InvalidSource,
    FileNotFound,
    SharedObjectSymbolNotFound,
    SharedObjectInitFailed,
    OperatingSystem,
    InvalidHandle,
    IllegalState,
    NotFound,
    NotReady,
    IllegalAddress,
    LaunchOutOfResources,
    LaunchTimeout,
    LaunchIncompatibleTexturing,
    PeerAccessAlreadyEnabled,
    PeerAccessNotEnabled,
    PrimaryContextActive,
    ContextIsDestroyed,
    Assert,
    TooManyPeers,
    HostMemoryAlreadyRegistered,
    HostMemoryNotRegistered,
    HardwareStackError,
    IllegalInstruction,
    MisalignedAddress,
    InvalidAddressSpace,
    InvalidPC,
    LaunchFailed,
    CooperativeLaunchTooLarge,
    NotPermitted,
    NotSupported,
    SystemNotReady,
    SystemDriverMismatch,
    CompatNotSupportedOnDevice,
    MPSConnectionFailed,
    MPSRPCFailure,
    MPSServerNotReady,
    MPSMaxClientsReached,
    MPSClientTerminated,
    CDPNotSupported,
    CDPVersionMismatch,
    StreamCaptureUnsupported,
    StreamCaptureInvalidated,
    StreamCaptureMerge,
    StreamCaptureUnmatched,
    StreamCaptureUnjoined,
    StreamCaptureIsolation,
    StreamCaptureImplicit,
    CaptureEvent,
    StreamCaptureWrongThread,
    Timeout,
    GraphExecUpdateFailure,
    ExternalDevice,
    InvalidClusterSize,
    Unknown
}

impl std::error::Error for CUDAError {}

impl Display for CUDAError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	use self::CUDAError::*;
	match self {
	    Success      => write!(f,"CUDAError::Success"),
	    InvalidValue => write!(f,"CUDAError::InvalidValue"),
	    OutOfMemory  => write!(f,"CUDAError::OutOfMemory"),
	    NotInitalized => write!(f,"CUDAError::NotInitalized"),
	    DeInitialized => write!(f,"CUDAError::DeInitialized"),
	    ProfilerDisabled => write!(f,"CUDAError::ProfilerDisabled"),
	    ProfilerNotInitialized => write!(f,"CUDAError::ProfilerNotInitialized"),
	    ProfilerAlreadyStarted => write!(f,"CUDAError::ProfilerAlreadyStarted"),
	    ProfilerAlreadyStopped => write!(f,"CUDAError::ProfilerAlreadyStopped"),
	    StubLibrary => write!(f,"CUDAError::StubLibrary"),
	    DeviceUnavailable => write!(f,"CUDAError::DeviceUnavailable"),
	    NoDevice => write!(f,"CUDAError::NoDevice"),
	    InvalidDevice => write!(f,"CUDAError::InvalidDevice"),
	    DeviceNotLicensed => write!(f,"CUDAError::DeviceNotLicensed"),
	    InvalidImage => write!(f,"CUDAError::InvalidImage"),
	    InvalidContext => write!(f,"CUDAError::InvalidContext"),
	    ContextAlreadyCurrent => write!(f,"CUDAError::ContextAlreadyCurrent"),
	    MapFailed => write!(f,"CUDAError::MapFailed"),
	    UnmapFailed => write!(f,"CUDAError::UnmapFailed"),
	    ArrayIsMapped => write!(f,"CUDAError::ArrayIsMapped"),
	    AlreadyMapped => write!(f,"CUDAError::AlreadyMapped"),
	    NoBinaryForGPU => write!(f,"CUDAError::NoBinaryForGPU"),
	    AlreadyAcquired => write!(f,"CUDAError::AlreadyAcquired"),
	    NotMapped => write!(f,"CUDAError::NotMapped"),
	    NotMappedAsArray => write!(f,"CUDAError::NotMappedAsArray"),
	    NotMappedAsPointer => write!(f,"CUDAError::NotMappedAsPointer"),
	    EccUncorrectable => write!(f,"CUDAError::EccUncorrectable"),
	    UnsupportedLimit => write!(f,"CUDAError::UnsupportedLimit"),
	    ContextAlreadyInUse => write!(f,"CUDAError::ContextAlreadyInUse"),
	    PeerAccessUnsuppoted => write!(f,"CUDAError::PeerAccessUnsuppoted"),
	    InvalidPtx => write!(f,"CUDAError::InvalidPtx"),
	    InvalidGraphicsContext => write!(f,"CUDAError::InvalidGraphicsContext"),
	    NvlinkUncorrectable => write!(f,"CUDAError::NvlinkUncorrectable"),
	    JitCompilerNotFound => write!(f,"CUDAError::JitCompilerNotFound"),
	    UnsupportedPtxVersion => write!(f,"CUDAError::UnsupportedPtxVersion"),
	    JitCompilationDisabled => write!(f,"CUDAError::JitCompilationDisabled"),
	    UnsupportedExecAffinity => write!(f,"CUDAError::UnsupportedExecAffinity"),
	    UnsupportedDevSideSync => write!(f,"CUDAError::UnsupportedDevSideSync"),
	    InvalidSource => write!(f,"CUDAError::InvalidSource"),
	    FileNotFound => write!(f,"CUDAError::FileNotFound"),
	    SharedObjectSymbolNotFound => write!(f,"CUDAError::SharedObjectSymbolNotFound"),
	    SharedObjectInitFailed => write!(f,"CUDAError::SharedObjectInitFailed"),
	    OperatingSystem => write!(f,"CUDAError::OperatingSystem"),
	    InvalidHandle => write!(f,"CUDAError::InvalidHandle"),
	    IllegalState => write!(f,"CUDAError::IllegalState"),
	    NotFound => write!(f,"CUDAError::NotFound"),
	    NotReady => write!(f,"CUDAError::NotReady"),
	    IllegalAddress => write!(f,"CUDAError::IllegalAddress"),
	    LaunchOutOfResources => write!(f,"CUDAError::LaunchOutOfResources"),
	    LaunchTimeout => write!(f,"CUDAError::LaunchTimeout"),
	    LaunchIncompatibleTexturing => write!(f,"CUDAError::IncompatibleTexturing"),
	    PeerAccessAlreadyEnabled => write!(f,"CUDAError::PeerAccessAlreadyEnabled"),
	    PeerAccessNotEnabled => write!(f,"CUDAError::PeerAccessNotEnabled"),
	    PrimaryContextActive => write!(f,"CUDAError::PrimaryContextActive"),
	    ContextIsDestroyed => write!(f,"CUDAError::ContextIsDestroyed"),
	    Assert => write!(f,"CUDAError::Assert"),
	    TooManyPeers => write!(f,"CUDAError::TooManyPeers"),
	    HostMemoryAlreadyRegistered => write!(f,"CUDAError::HostMemoryAlreadyRegistered"),
	    HostMemoryNotRegistered => write!(f,"CUDAError::HostMemoryNotRegistered"),
	    HardwareStackError => write!(f,"CUDAError::HardwareStackError"),
	    IllegalInstruction => write!(f,"CUDAError::IllegalInstruction"),
	    MisalignedAddress => write!(f,"CUDAError::MisalignedAddress"),
	    InvalidAddressSpace => write!(f,"CUDAError::InvalidAddressSpace"),
	    InvalidPC => write!(f,"CUDAError::InvalidPC"),
	    LaunchFailed => write!(f,"CUDAError::LaunchFailed"),
	    CooperativeLaunchTooLarge => write!(f,"CUDAError::LaunchTooLarge"),
	    NotPermitted => write!(f,"CUDAError::NotPermitted"),
	    NotSupported => write!(f,"CUDAError::NotSupported"),
	    SystemNotReady => write!(f,"CUDAError::SystemNotReady"),
	    SystemDriverMismatch => write!(f,"CUDAError::DriverMismatch"),
	    CompatNotSupportedOnDevice => write!(f,"CUDAError::CompatNotSupportedOnDevice"),
	    MPSConnectionFailed => write!(f,"CUDAError::MPSConnectionFailed"),
	    MPSRPCFailure => write!(f,"CUDAError::MPSRPCFailure"),
	    MPSServerNotReady => write!(f,"CUDAError::MPSServerNotReady"),
	    MPSMaxClientsReached => write!(f,"CUDAError::MPSMaxClientsReached"),
	    MPSClientTerminated => write!(f,"CUDAError::MPSClientTerminated"),
	    CDPNotSupported => write!(f,"CUDAError::CDPNotSupported"),
	    CDPVersionMismatch => write!(f,"CUDAError::CDPVersionMismatch"),
	    StreamCaptureUnsupported => write!(f,"CUDAError::StreamCaptureUnsupported"),
	    StreamCaptureInvalidated => write!(f,"CUDAError::StreamCaptureInvalidated"),
	    StreamCaptureMerge => write!(f,"CUDAError::StreamCaptureMerge"),
	    StreamCaptureUnmatched => write!(f,"CUDAError::StreamCaptureUnmatched"),
	    StreamCaptureUnjoined => write!(f,"CUDAError::StreamCaptureUnjoined"),
	    StreamCaptureIsolation => write!(f,"CUDAError::StreamCaptureIsolation"),
	    StreamCaptureImplicit => write!(f,"CUDAError::StreamCaptureImplicit"),
	    CaptureEvent => write!(f,"CUDAError::CaptureEvent"),
	    StreamCaptureWrongThread => write!(f,"CUDAError::StreamCaptureWrongThread"),
	    Timeout => write!(f,"CUDAError::Timeout"),
	    GraphExecUpdateFailure => write!(f,"CUDAError::GraphExecUpdateFailure"),
	    ExternalDevice => write!(f,"CUDAError::ExternalDevice"),
	    InvalidClusterSize => write!(f,"CUDAError::ExternalDevice"),
	    Unknown => write!(f,"CUDAError::Unknown")
	}
    }
}

impl From<cudaError> for CUDAError {
    fn from(error:cudaError) -> Self {
	match error {
	    cudaError_enum_CUDA_SUCCESS => CUDAError::Success,
	    cudaError_enum_CUDA_ERROR_INVALID_VALUE => CUDAError::InvalidValue,
	    cudaError_enum_CUDA_ERROR_OUT_OF_MEMORY => CUDAError::OutOfMemory,
	    cudaError_enum_CUDA_ERROR_NOT_INITIALIZED => CUDAError::NotInitalized,
	    cudaError_enum_CUDA_ERROR_DEINITIALIZED => CUDAError::DeInitialized,
	    cudaError_enum_CUDA_ERROR_PROFILER_DISABLED => CUDAError::ProfilerDisabled,
	    cudaError_enum_CUDA_ERROR_PROFILER_NOT_INITIALIZED => CUDAError::ProfilerNotInitialized,
	    cudaError_enum_CUDA_ERROR_PROFILER_ALREADY_STARTED => CUDAError::ProfilerAlreadyStarted,
	    cudaError_enum_CUDA_ERROR_PROFILER_ALREADY_STOPPED => CUDAError::ProfilerAlreadyStopped,
	    cudaError_enum_CUDA_ERROR_STUB_LIBRARY => CUDAError::StubLibrary,
	    cudaError_enum_CUDA_ERROR_DEVICE_UNAVAILABLE => CUDAError::DeviceUnavailable,
	    cudaError_enum_CUDA_ERROR_NO_DEVICE => CUDAError::NoDevice,
	    cudaError_enum_CUDA_ERROR_INVALID_DEVICE => CUDAError::InvalidDevice,
	    cudaError_enum_CUDA_ERROR_DEVICE_NOT_LICENSED => CUDAError::DeviceNotLicensed,
	    cudaError_enum_CUDA_ERROR_INVALID_IMAGE => CUDAError::InvalidImage,
	    cudaError_enum_CUDA_ERROR_INVALID_CONTEXT => CUDAError::InvalidContext,
	    cudaError_enum_CUDA_ERROR_CONTEXT_ALREADY_CURRENT => CUDAError::ContextAlreadyCurrent,
	    cudaError_enum_CUDA_ERROR_MAP_FAILED => CUDAError::MapFailed,
	    cudaError_enum_CUDA_ERROR_UNMAP_FAILED => CUDAError::UnmapFailed,
	    cudaError_enum_CUDA_ERROR_ARRAY_IS_MAPPED => CUDAError::ArrayIsMapped,
	    cudaError_enum_CUDA_ERROR_ALREADY_MAPPED => CUDAError::AlreadyMapped,
	    cudaError_enum_CUDA_ERROR_NO_BINARY_FOR_GPU => CUDAError::NoBinaryForGPU,
	    cudaError_enum_CUDA_ERROR_ALREADY_ACQUIRED => CUDAError::AlreadyAcquired,
	    cudaError_enum_CUDA_ERROR_NOT_MAPPED => CUDAError::NotMapped,
	    cudaError_enum_CUDA_ERROR_NOT_MAPPED_AS_ARRAY => CUDAError::NotMappedAsArray,
	    cudaError_enum_CUDA_ERROR_NOT_MAPPED_AS_POINTER => CUDAError::NotMappedAsPointer,
	    cudaError_enum_CUDA_ERROR_ECC_UNCORRECTABLE => CUDAError::EccUncorrectable,
	    cudaError_enum_CUDA_ERROR_UNSUPPORTED_LIMIT => CUDAError::UnsupportedLimit,
	    cudaError_enum_CUDA_ERROR_CONTEXT_ALREADY_IN_USE => CUDAError::ContextAlreadyInUse,
	    cudaError_enum_CUDA_ERROR_PEER_ACCESS_UNSUPPORTED => CUDAError::PeerAccessUnsuppoted,
	    cudaError_enum_CUDA_ERROR_INVALID_PTX => CUDAError::InvalidPtx,
	    cudaError_enum_CUDA_ERROR_INVALID_GRAPHICS_CONTEXT => CUDAError::InvalidGraphicsContext,
	    cudaError_enum_CUDA_ERROR_NVLINK_UNCORRECTABLE => CUDAError::NvlinkUncorrectable,
	    cudaError_enum_CUDA_ERROR_JIT_COMPILER_NOT_FOUND => CUDAError::JitCompilerNotFound,
	    cudaError_enum_CUDA_ERROR_UNSUPPORTED_PTX_VERSION => CUDAError::UnsupportedPtxVersion,
	    cudaError_enum_CUDA_ERROR_JIT_COMPILATION_DISABLED => CUDAError::JitCompilationDisabled,
	    cudaError_enum_CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY => CUDAError::UnsupportedExecAffinity,
	    cudaError_enum_CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC => CUDAError::UnsupportedDevSideSync,
	    cudaError_enum_CUDA_ERROR_INVALID_SOURCE => CUDAError::InvalidSource,
	    cudaError_enum_CUDA_ERROR_FILE_NOT_FOUND => CUDAError::FileNotFound,
	    cudaError_enum_CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND => CUDAError::SharedObjectSymbolNotFound,
	    cudaError_enum_CUDA_ERROR_SHARED_OBJECT_INIT_FAILED => CUDAError::SharedObjectInitFailed,
	    cudaError_enum_CUDA_ERROR_OPERATING_SYSTEM => CUDAError::OperatingSystem,
	    cudaError_enum_CUDA_ERROR_INVALID_HANDLE => CUDAError::InvalidHandle,
	    cudaError_enum_CUDA_ERROR_ILLEGAL_STATE => CUDAError::IllegalState,
	    cudaError_enum_CUDA_ERROR_NOT_FOUND => CUDAError::NotFound,
	    cudaError_enum_CUDA_ERROR_NOT_READY => CUDAError::NotReady,
	    cudaError_enum_CUDA_ERROR_ILLEGAL_ADDRESS => CUDAError::IllegalAddress,
	    cudaError_enum_CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES => CUDAError::LaunchOutOfResources,
	    cudaError_enum_CUDA_ERROR_LAUNCH_TIMEOUT => CUDAError::LaunchTimeout,
	    cudaError_enum_CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING => CUDAError::LaunchIncompatibleTexturing,
	    cudaError_enum_CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED => CUDAError::PeerAccessAlreadyEnabled,
	    cudaError_enum_CUDA_ERROR_PEER_ACCESS_NOT_ENABLED => CUDAError::PeerAccessNotEnabled,
	    cudaError_enum_CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE => CUDAError::PrimaryContextActive,
	    cudaError_enum_CUDA_ERROR_CONTEXT_IS_DESTROYED => CUDAError::ContextIsDestroyed,
	    cudaError_enum_CUDA_ERROR_ASSERT => CUDAError::Assert,
	    cudaError_enum_CUDA_ERROR_TOO_MANY_PEERS => CUDAError::TooManyPeers,
	    cudaError_enum_CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED => CUDAError::HostMemoryAlreadyRegistered,
	    cudaError_enum_CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED => CUDAError::HostMemoryNotRegistered,
	    cudaError_enum_CUDA_ERROR_HARDWARE_STACK_ERROR => CUDAError::HardwareStackError,
	    cudaError_enum_CUDA_ERROR_ILLEGAL_INSTRUCTION => CUDAError::IllegalInstruction,
	    cudaError_enum_CUDA_ERROR_MISALIGNED_ADDRESS => CUDAError::MisalignedAddress,
	    cudaError_enum_CUDA_ERROR_INVALID_ADDRESS_SPACE => CUDAError::InvalidAddressSpace,
	    cudaError_enum_CUDA_ERROR_INVALID_PC => CUDAError::InvalidPC,
	    cudaError_enum_CUDA_ERROR_LAUNCH_FAILED => CUDAError::LaunchFailed,
	    cudaError_enum_CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE => CUDAError::CooperativeLaunchTooLarge,
	    cudaError_enum_CUDA_ERROR_NOT_PERMITTED => CUDAError::NotPermitted,
	    cudaError_enum_CUDA_ERROR_NOT_SUPPORTED => CUDAError::NotSupported,
	    cudaError_enum_CUDA_ERROR_SYSTEM_NOT_READY => CUDAError::SystemNotReady,
	    cudaError_enum_CUDA_ERROR_SYSTEM_DRIVER_MISMATCH => CUDAError::SystemDriverMismatch,
	    cudaError_enum_CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE => CUDAError::CompatNotSupportedOnDevice,
	    cudaError_enum_CUDA_ERROR_MPS_CONNECTION_FAILED => CUDAError::MPSConnectionFailed,
	    cudaError_enum_CUDA_ERROR_MPS_RPC_FAILURE => CUDAError::MPSRPCFailure,
	    cudaError_enum_CUDA_ERROR_MPS_SERVER_NOT_READY => CUDAError::MPSServerNotReady,
	    cudaError_enum_CUDA_ERROR_MPS_MAX_CLIENTS_REACHED => CUDAError::MPSMaxClientsReached,
	    cudaError_enum_CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED => CUDAError::MPSMaxClientsReached,
	    cudaError_enum_CUDA_ERROR_MPS_CLIENT_TERMINATED => CUDAError::MPSClientTerminated,
	    cudaError_enum_CUDA_ERROR_CDP_NOT_SUPPORTED => CUDAError::CDPNotSupported,
	    cudaError_enum_CUDA_ERROR_CDP_VERSION_MISMATCH => CUDAError::CDPVersionMismatch,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED => CUDAError::StreamCaptureUnsupported,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_INVALIDATED => CUDAError::StreamCaptureInvalidated,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_MERGE => CUDAError::StreamCaptureMerge,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_UNMATCHED => CUDAError::StreamCaptureUnmatched,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_UNJOINED => CUDAError::StreamCaptureUnjoined,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_ISOLATION => CUDAError::StreamCaptureIsolation,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_IMPLICIT => CUDAError::StreamCaptureImplicit,
	    cudaError_enum_CUDA_ERROR_CAPTURED_EVENT => CUDAError::CaptureEvent,
	    cudaError_enum_CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD => CUDAError::StreamCaptureWrongThread,
	    cudaError_enum_CUDA_ERROR_TIMEOUT => CUDAError::Timeout,
	    cudaError_enum_CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE => CUDAError::GraphExecUpdateFailure,
	    cudaError_enum_CUDA_ERROR_EXTERNAL_DEVICE => CUDAError::ExternalDevice,
	    cudaError_enum_CUDA_ERROR_INVALID_CLUSTER_SIZE => CUDAError::InvalidClusterSize,
	    cudaError_enum_CUDA_ERROR_UNKNOWN => CUDAError::Unknown,
	    _ => panic!("unknow cuda error {}", error)
	}
    }
}

pub mod cuda;

