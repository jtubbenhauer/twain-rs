use ::libloading::{Library, Symbol};
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct TwainDsm {
    lib: Rc<Library>,
}

impl TwainDsm {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let twain_dsm_path = PathBuf::from("TWAINDSM.dll");
        let lib = Rc::new(unsafe { Library::new(twain_dsm_path)? });

        let _: Symbol<DSMENTRYPROC> = unsafe { lib.get(b"DSM_Entry")? };

        Ok(TwainDsm { lib })
    }

    pub unsafe fn dsm_entry(
        &self,
        origin: pTW_IDENTITY,
        dest: pTW_IDENTITY,
        dg: TW_UINT32,
        dat: TW_UINT16,
        msg: TW_UINT16,
        data: TW_MEMREF,
    ) -> TW_UINT16 {
        let dsm_entry: Symbol<DSMENTRYPROC> = self.lib.get(b"DSM_Entry").unwrap();

        match *dsm_entry {
            Some(func) => func(origin, dest, dg, dat, msg, data),
            None => panic!("DSM_Entry function not found"),
        }
    }
}
