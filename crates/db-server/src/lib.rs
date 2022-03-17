// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod server {
    use std::sync::Arc;

    use db::{rpc, KeyValueStore};

    pub type DataType = Arc<KeyValueStore>;

    pub struct StupidServer {
        pub(crate) store: DataType,
    }

    impl StupidServer {
        pub fn new() -> Self {
            Self {
                store: Arc::new(KeyValueStore::empty()),
            }
        }

        pub fn request(&self, req: &rpc::GenericRequest) -> rpc::GenericResponse {
            use rpc::generic_request::Request;
            use rpc::generic_response::Response;
            let inner: Response = match &req.request {
                Some(actual) => match actual {
                    Request::GetRequest(get) => Response::GetResponse(self.get(get)),
                    Request::SetRequest(set) => Response::SetResponse(self.set(set)),
                    Request::DeleteRequest(del) => Response::DeleteResponse(self.delete(del)),
                },
                None => return rpc::GenericResponse { response: None },
            };

            rpc::GenericResponse {
                response: Some(inner),
            }
        }

        pub fn get(&self, req: &rpc::GetRequest) -> rpc::GetResponse {
            let (value, resp_msg, code) = match self.store.get_clone(req.key.as_str()) {
                Ok(row) => (row.value().to_string(), "".to_string(), rpc::StatusCode::Ok),
                Err(err) => ("".to_string(), err.to_string(), rpc::StatusCode::Fail),
            };
            rpc::GetResponse {
                value,
                resp_msg,
                status_code: code.into(),
            }
        }

        pub fn set(&self, req: &rpc::SetRequest) -> rpc::SetResponse {
            let (message, resp_msg, code) = match self
                .store
                .set_or_insert(req.key.as_str(), req.value.as_str())
            {
                Ok(_) => (
                    format!("set/updated {}", req.key),
                    "".to_string(),
                    rpc::StatusCode::Ok,
                ),
                Err(err) => ("".to_string(), err.to_string(), rpc::StatusCode::Fail),
            };

            rpc::SetResponse {
                message,
                resp_msg,
                status_code: code.into(),
            }
        }

        pub fn delete(&self, req: &rpc::DeleteRequest) -> rpc::DeleteResponse {
            let (message, resp_msg, code) = match self.store.delete(req.key.as_str()) {
                Ok(deleted) => (
                    format!("deleted {}", deleted),
                    "".to_string(),
                    rpc::StatusCode::Ok,
                ),
                Err(err) => ("".to_string(), err.to_string(), rpc::StatusCode::Fail),
            };

            rpc::DeleteResponse {
                message,
                resp_msg,
                status_code: code.into(),
            }
        }
    }
}
