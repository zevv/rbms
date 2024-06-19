
use std::fmt;

pub enum Rv {
        Ok,          /* No error */
        //ErrNoent,    /* Not found */
        ErrIo,       /* I/O error */
        //ErrNodev,    /* No such device */
        ErrImpl,     /* Not implemented */
        //ErrTimeout,  /* Timeout */
        //ErrBusy,     /* Busy */
        //ErrInval,    /* Invalid argument */
        //ErrNotready, /* Not ready */
        //ErrNospc,    /* No space left */
        //ErrProto,    /* Protocol error */
        //ErrNomem,    /* Out of memory */
        //ErrAlign,    /* Invalid alignment */
        //ErrCrc,      /* Checksum error */
        //ErrPerm,     /* Permission denied */
}

impl fmt::Display for Rv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Rv::Ok => write!(f, "Ok"),
           //Rv::ErrNoent => write!(f, "ErrNoent"),
           Rv::ErrIo => write!(f, "ErrIo"),
           Rv::ErrImpl => write!(f, "ErrImpl"),
       }
    }
}


