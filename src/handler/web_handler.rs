use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{Extension, Json, Router, Server};
use axum::routing::{any, get_service, post};
use axum::http::StatusCode;
use log::{debug, info};
use serde::Deserialize;
use serde_json::json;
use systemstat::{Platform, System};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::{AppConfigByWeb, Cipher256, Cipher512, CipherParser, SysInfo, SystemInfo, UserConfig, UserConfigParser, UserRow};

pub struct WebHandler {
    sys: System,
    users: HashMap<String, UserRow>,
    cipher: String,
    authorized: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Deserialize, Debug)]
pub struct WebHandlerAuthorizedParam {
    authorized: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct WebHandlerUserParam {
    username: Option<String>,
    password: Option<String>,
}


impl WebHandler {
    pub async fn sys(
        Extension(handler): Extension<Arc<WebHandler>>
    ) -> Result<Json<SystemInfo>, StatusCode> {
        Ok(Json(SystemInfo::sys(&handler.sys).unwrap()))
    }


    pub async fn sys_with_auth(
        Extension(handler): Extension<Arc<WebHandler>>,
        Json(params): Json<WebHandlerAuthorizedParam>,
    ) -> Result<Json<SystemInfo>, StatusCode> {
        debug!("authorized = {:?}",params);
        if params.authorized.is_none() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // check empty
        let authorized = params.authorized.unwrap();
        if authorized.is_empty() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        // exists ?
        let handler_authorized = handler.authorized.lock().await;
        if !handler_authorized.contains_key(authorized.as_str()) {
            return Err(StatusCode::UNAUTHORIZED);
        }


        Ok(Json(SystemInfo::sys(&handler.sys).unwrap()))
    }


    pub async fn sys_check_auth(
        Extension(handler): Extension<Arc<WebHandler>>,
        Json(params): Json<WebHandlerUserParam>,
    ) -> Result<Json<serde_json::Value>, StatusCode> {
        debug!("user = {:?}",params);
        let username = match &params.username {
            Some(active) => active,
            None => {
                return Err(StatusCode::BAD_REQUEST);
            }
        };
        let password = match &params.password {
            Some(active) => active,
            None => {
                return Err(StatusCode::BAD_REQUEST);
            }
        };


        // hash
        let hash = match &handler.users.get(username) {
            Some(active) => &active.password,
            None => {
                return Err(StatusCode::NOT_FOUND);
            }
        };

        // comparisons
        let password = match handler.cipher.as_str() {
            "sha512" => Cipher512::create(password.as_bytes()).cipher(),
            _ => Cipher256::create(password.as_bytes()).cipher(),
        };
        let password = match password {
            Some(active) => active,
            None => {
                return Err(StatusCode::NOT_ACCEPTABLE);
            }
        };
        if !password.eq(hash) {
            return Err(StatusCode::NOT_ACCEPTABLE);
        }

        // remove other
        let mut handler_authorized = handler.authorized.lock().await;
        handler_authorized.retain(|_k, v| !v.as_str().eq(username.as_str()));


        // write token
        let uuid = Uuid::new_v4();
        let authorized_uuid = uuid.to_string();
        let authorized_username = username.clone();
        handler_authorized.insert(authorized_uuid, authorized_username);
        debug!("{:?}",handler_authorized);


        // response
        let response_username = username.clone();
        let response_uuid = uuid.to_string();
        let response = json!({
            "username": response_username,
            "authorized": response_uuid,
        });
        Ok(Json(response))
    }


    pub async fn init(conf: &AppConfigByWeb) -> Result<(), Box<dyn std::error::Error>> {

        // auth information
        let users = match &conf.auth {
            None => UserConfig::with_capacity(0),
            Some(active) => UserConfigParser::user_config(active.as_str())
        };

        // server monitor
        let address = format!("{}:{}", conf.address, conf.port);
        let socket = address.parse::<SocketAddr>()?;
        info!("web url = http://{}",address);

        // system shared
        let cipher = match &conf.cipher {
            None => String::default(),
            Some(active) => active.clone(),
        };
        let shared = Arc::new(Self {
            sys: System::new(),
            authorized: Arc::new(Mutex::new(HashMap::new())),
            cipher,
            users,
        });


        // router
        let router = if shared.users.is_empty() {
            Router::new().route("/sys", any(Self::sys))
        } else {
            Router::new()
                .route("/sys", post(Self::sys_with_auth))
                .route("/auth", post(Self::sys_check_auth))
        }.layer(Extension(shared));

        // static files
        let router = match &conf.static_dir {
            None => router,
            Some(dir) => router.fallback(get_service(ServeDir::new(dir.as_str())))
        };

        // server
        let _ = tokio::spawn(async move {
            Server::bind(&socket)
                .serve(router.into_make_service())
                .await
        }).await;
        Ok(())
    }
}
