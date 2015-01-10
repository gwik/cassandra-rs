#![allow(non_camel_case_types)]
#![allow(dead_code)]

use cass_batch::CassBatch;
use cass_future::CassFuture;
use cass_string::CassString;
use cass_statement::CassStatement;
use cass_schema::CassSchema;

pub enum Struct_CassSession_ { }
pub type CassSession = Struct_CassSession_;

extern "C" {
    pub fn cass_session_close(session: *mut CassSession) -> *mut CassFuture;
    pub fn cass_session_prepare(session: *mut CassSession, query: CassString) -> *mut CassFuture;
    pub fn cass_session_execute(session: *mut CassSession, statement: *const CassStatement) -> *mut CassFuture;
    pub fn cass_session_execute_batch(session: *mut CassSession, batch: *const CassBatch) -> *mut CassFuture;
    pub fn cass_session_get_schema(session: *mut CassSession) -> *const CassSchema;
}