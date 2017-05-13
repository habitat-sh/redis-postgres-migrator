// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error as StdError;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::result::Result as StdResult;
use std::time::Duration;

//use hyper::{self, Url};
//use hyper::status::StatusCode;
//use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
//use hyper::mime::{Mime, TopLevel, SubLevel};
use protocol::{net, sessionsrv};
use rustc_serialize::json;

use config;
use error::{Error, Result};

const USER_AGENT: &'static str = "Habitat-Builder";
const HTTP_TIMEOUT: u64 = 3_000;
// These OAuth scopes are required for a user to be authenticated. If this list is updated, then
// the front-end also needs to be updated in `components/builder-web/app/util.ts`. Both the
// front-end app and back-end app should have identical requirements to make things easier for
// our users and less cumbersome for us to message out.
// https://developer.github.com/v3/oauth/#scopes
const AUTH_SCOPES: &'static [&'static str] = &["user:email", "read:org"];

#[derive(Clone)]
pub struct GitHubClient {
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
}
