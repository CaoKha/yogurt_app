//! This is a prelude for all .._rpc modules to avoid redundant imports.
//! NOTE: This is only for the `rpcs` module and sub-modules.

pub use crate::generate_common_rpc_fns;
pub use crate::backend::rpc::rpc_result::DataRpcResult;
pub use crate::backend::rpc::Result;
pub use crate::backend::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
pub use crate::backend::ctx::Ctx;
pub use crate::backend::model::ModelManager;
pub use paste::paste;
pub use rpc_router::{router_builder, RouterBuilder};
