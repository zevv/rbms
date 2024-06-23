
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rv {
        Ok,          /* No error */
        ErrIo,       /* I/O error */
        ErrImpl,     /* Not implemented */
        ErrNotReady, /* Not ready */
        //ErrNoent,    /* Not found */
        //ErrNodev,    /* No such device */
        //ErrTimeout,  /* Timeout */
        //ErrBusy,     /* Busy */
        //ErrInval,    /* Invalid argument */
        //ErrNospc,    /* No space left */
        //ErrProto,    /* Protocol error */
        //ErrNomem,    /* Out of memory */
        //ErrAlign,    /* Invalid alignment */
        //ErrCrc,      /* Checksum error */
        //ErrPerm,     /* Permission denied */
}

impl fmt::Display for Rv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


