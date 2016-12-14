// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net
// Commercial License, version 1.0 or later, or (2) The General Public License
// (GPL), version 3, depending on which licence you accepted on initial access
// to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project
// generally, you agree to be bound by the terms of the MaidSafe Contributor
// Agreement, version 1.0.
// This, along with the Licenses can be found in the root directory of this
// project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network
// Software distributed under the GPL Licence is distributed on an "AS IS"
// BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied.
//
// Please review the Licences for the specific language governing permissions
// and limitations relating to use of the SAFE Network Software.

//! Errors thrown by Authenticator routines

use core::CoreError;
use futures::sync::mpsc::SendError;
use ipc::IpcError;
use maidsafe_utilities::serialisation::SerialisationError;
use nfs::errors::NfsError;
use std::error::Error;
use std::fmt::{self, Debug, Formatter};
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::sync::mpsc::RecvError;

/// Intended for converting Launcher Errors into numeric codes for propagating
/// some error information across FFI boundaries and specially to C.
pub const AUTH_ERROR_START_RANGE: i32 = 0;

/// Authenticator errors
pub enum AuthError {
    /// Unexpected - Probably a Logic error
    Unexpected(String),
    /// Error from safe_core. Boxed to hold a pointer instead of value so that this enum variant is
    /// not insanely bigger than others.
    CoreError(CoreError),
    /// Input/output error
    IoError(IoError),
    /// NFS error
    NfsError(NfsError),
    /// Serialisation error
    SerialisationError(SerialisationError),
    /// IPC error
    IpcError(IpcError),
}

impl Into<IpcError> for AuthError {
    fn into(self) -> IpcError {
        match self {
            AuthError::Unexpected(desc) => IpcError::Unexpected(desc),
            AuthError::IpcError(err) => err,
            err => IpcError::Unexpected(format!("{:?}", err)),
        }
    }
}

impl Into<i32> for AuthError {
    fn into(self) -> i32 {
        match self {
            AuthError::Unexpected(_) => AUTH_ERROR_START_RANGE - 1,
            AuthError::IoError(_) => AUTH_ERROR_START_RANGE - 2,
            AuthError::CoreError(error) => error.into(),
            AuthError::NfsError(error) => error.into(),
            AuthError::SerialisationError(_) => AUTH_ERROR_START_RANGE - 3,
            AuthError::IpcError(_) => AUTH_ERROR_START_RANGE - 4,
        }
    }
}

impl<T: 'static> From<SendError<T>> for AuthError {
    fn from(error: SendError<T>) -> AuthError {
        AuthError::Unexpected(error.description().to_owned())
    }
}

impl From<CoreError> for AuthError {
    fn from(error: CoreError) -> AuthError {
        AuthError::CoreError(error)
    }
}

impl From<IpcError> for AuthError {
    fn from(error: IpcError) -> AuthError {
        AuthError::IpcError(error)
    }
}

impl From<RecvError> for AuthError {
    fn from(error: RecvError) -> AuthError {
        AuthError::Unexpected(error.description().to_owned())
    }
}

impl From<IoError> for AuthError {
    fn from(error: IoError) -> AuthError {
        AuthError::IoError(error)
    }
}

impl<'a> From<&'a str> for AuthError {
    fn from(error: &'a str) -> AuthError {
        AuthError::Unexpected(error.to_owned())
    }
}

impl From<Utf8Error> for AuthError {
    fn from(error: Utf8Error) -> AuthError {
        AuthError::Unexpected(error.description().to_owned())
    }
}

impl From<NfsError> for AuthError {
    fn from(error: NfsError) -> AuthError {
        AuthError::NfsError(error)
    }
}

impl From<SerialisationError> for AuthError {
    fn from(error: SerialisationError) -> AuthError {
        AuthError::SerialisationError(error)
    }
}

impl Debug for AuthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            AuthError::CoreError(ref error) => write!(f, "AuthError::CoreError -> {:?}", error),
            AuthError::IoError(ref error) => write!(f, "AuthError::IoError -> {:?}", error),
            AuthError::Unexpected(ref s) => write!(f, "AuthError::Unexpected{{{:?}}}", s),
            AuthError::NfsError(ref error) => write!(f, "AuthError::NfsError -> {:?}", error),
            AuthError::IpcError(ref error) => write!(f, "AuthError::IpcError -> {:?}", error),
            AuthError::SerialisationError(ref error) => {
                write!(f, "AuthError::SerialisationError -> {:?}", error)
            }
        }
    }
}
