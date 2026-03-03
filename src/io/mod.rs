use crate::prelude::*;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("An entity was not found, often a file.")]
    NotFound,
    #[error("The operation lacked the necessary privileges to complete.")]
    PermissionDenied,
    #[error("The connection was refused by the remote server.")]
    ConnectionRefused,
    #[error("The connection was reset by the remote server.")]
    ConnectionReset,
    #[error("The remote host is not reachable.")]
    HostUnreachable,
    #[error("The network containing the remote host is not reachable.")]
    NetworkUnreachable,
    #[error("The connection was aborted (terminated) by the remote server.")]
    ConnectionAborted,
    #[error("The network operation failed because it was not connected yet.")]
    NotConnected,
    #[error("A socket address could not be bound because the address is already in use elsewhere.")]
    AddrInUse,
    #[error("A nonexistent interface was requested or the requested address was not local.")]
    AddrNotAvailable,
    #[error("The system’s networking is down.")]
    NetworkDown,
    #[error("The operation failed because a pipe was closed.")]
    BrokenPipe,
    #[error("An entity already exists, often a file.")]
    AlreadyExists,
    #[error(
        "The operation needs to block to complete, but the blocking operation was requested to not occur."
    )]
    WouldBlock,
    #[error("A filesystem object is, unexpectedly, not a directory.")]
    NotADirectory,
    #[error("The filesystem object is, unexpectedly, a directory.")]
    IsADirectory,
    #[error("A non-empty directory was specified where an empty directory was expected.")]
    DirectoryNotEmpty,
    #[error("The filesystem or storage medium is read-only, but a write operation was attempted.")]
    ReadOnlyFilesystem,
    // #[error("")]
    // FilesystemLoop,
    // #[error("Stale network file handle.")]
    // StaleNetworkFileHandle,
    #[error("A parameter was incorrect.")]
    InvalidInput,
    #[error("Data not valid for the operation encountered.")]
    InvalidData,
    #[error("The I/O operation’s timeout expired, causing it to be canceled.")]
    TimedOut,
    // #[error("")]
    // WriteZero,
    #[error(
        "The underlying storage (typically, a filesystem) is full. This does not include out of quota errors."
    )]
    StorageFull,
    #[error("Seek on unseekable file.")]
    NotSeekable,
    #[error("Filesystem quota or some other kind of quota was exceeded.")]
    QuotaExceeded,
    #[error("File larger than allowed or supported.")]
    FileTooLarge,
    #[error("Resource is busy.")]
    ResourceBusy,
    #[error("Executable file is busy.")]
    ExecutableFileBusy,
    // #[error("")]
    // Deadlock,
    #[error("Cross-device or cross-filesystem (hard) link or rename.")]
    CrossesDevices,
    #[error("Too many (hard) links to the same filesystem object.")]
    TooManyLinks,
    #[error("A filename was invalid.")]
    InvalidFilename,
    #[error("Program argument list too long.")]
    ArgumentListTooLong,
    #[error("This operation was interrupted.")]
    Interrupted,
    #[error("This operation is unsupported on this platform.")]
    Unsupported,
    #[error(
        "An error returned when an operation could not be completed because an “end of file” was reached prematurely."
    )]
    UnexpectedEof,
    #[error("An operation could not be completed, because it failed to allocate enough memory.")]
    OutOfMemory,
    // #[error("")]
    // InProgress,
    #[error("A custom error that does not fall under any other I/O error kind.")]
    Other,
}

impl From<sys::nsys::net::socket::Error> for Error {
    fn from(value: sys::nsys::net::socket::Error) -> Self {
        use sys::nsys::net::socket::Error as SocketError;
        match value {
            SocketError::InsufficientResources => Self::OutOfMemory,
            SocketError::TimedOut => Self::TimedOut,
            SocketError::AlreadyConnected => Self::AlreadyExists,
            SocketError::OperationNotSupported => Self::Unsupported,
            SocketError::ConnectionAborted => Self::ConnectionAborted,
            SocketError::WouldBlock => Self::WouldBlock,
            SocketError::ConnectionRefused => Self::ConnectionRefused,
            SocketError::NotConnected => Self::NotConnected,
            SocketError::AlreadyInProgress => Self::ResourceBusy,
            SocketError::InvalidOperation => Self::Unsupported,
            SocketError::MessageTooLarge => Self::InvalidInput,
            SocketError::BrokenPipe => Self::BrokenPipe,
            SocketError::DestinationAddressRequired => Self::NotConnected,
            SocketError::Shutdown => Self::Interrupted,
            SocketError::ProtocolOptionNotSupported => Self::Unsupported,
            SocketError::OutOfBandDataPending => Self::Interrupted,
            SocketError::InsufficientMemory => Self::OutOfMemory,
            SocketError::AddressNotAvailable => Self::AddrNotAvailable,
            SocketError::AddressInUse => Self::AddrInUse,
            SocketError::AddressFamilyNotSupported => Self::AddrNotAvailable,
            SocketError::InProgress => Self::AlreadyExists,
            SocketError::IpLayerError => Self::BrokenPipe,
            SocketError::NotASocket => Self::ConnectionRefused,
            SocketError::IoError => Self::Interrupted,
            SocketError::TooManyReferences => Self::Interrupted,
            SocketError::BadAddress => Self::InvalidData,
            SocketError::NetworkUnreachable => Self::NetworkUnreachable,
            SocketError::ProtocolNotSupported => Self::Unsupported,
            SocketError::ProtocolTypeMismatch => Self::Unsupported,
            SocketError::GenericError => Self::Other,
            SocketError::FailedToOpenResourceManager => Self::ResourceBusy,
            SocketError::LibraryNotInitialized => Self::Interrupted,
            SocketError::Busy => Self::ResourceBusy,
            SocketError::Unknown => Self::Other,
            SocketError::InternalApiError => Self::Other,
            SocketError::InvalidErrorCode => Self::Other,
            SocketError::NoResources => Self::OutOfMemory,
            SocketError::BadFileDescriptor => Self::InvalidInput,
            SocketError::Aborted => Self::ConnectionAborted,
            SocketError::TooManySockets => Self::QuotaExceeded,
            SocketError::IcmpDestinationUnreachable => Self::HostUnreachable,
            SocketError::IcmpSourceQuench => Self::Other,
            SocketError::IcmpRedirect => Self::Other,
            SocketError::IcmpTimeExceeded => Self::Other,
            SocketError::IcmpParameterProblem => Self::Other,
            //
            _ => Self::Other,
        }
    }
}

impl From<core::net::AddrParseError> for Error {
    fn from(value: core::net::AddrParseError) -> Self {
        Self::InvalidInput
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait BufRead: Read {
    fn fill_buf(&mut self) -> Result<&[u8]>;
    fn consume(&mut self, amount: usize);
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
