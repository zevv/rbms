
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv {
        Ok,
        ErrNotReady,
        ErrInval,
        //ErrIo,
        //ErrImpl,
        //ErrNoent,
        //ErrNodev,
        //ErrTimeout,
        //ErrBusy,
        //ErrNospc,
        //ErrProto,
        //ErrNomem,
        //ErrAlign,
        //ErrCrc,
        //ErrPerm,
}

impl fmt::Display for Rv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


