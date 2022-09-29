use crate::{
    cores::{auth::role::Role, error::Error as ServiceError},
    services::auth::init::AuthService,
};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    cell::RefCell,
    future::{ready, Ready},
    rc::Rc,
};

pub struct AuthMiddleware {
    pub valid_role: Role,
    pub auth_service: AuthService,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RouteMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RouteMiddleware {
            service: Rc::new(RefCell::new(service)),
            valid_role: self.valid_role,
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct RouteMiddleware<S> {
    service: Rc<RefCell<S>>,
    valid_role: Role,
    auth_service: AuthService,
}

impl<S, B> Service<ServiceRequest> for RouteMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = get_auth_header(&req);
        let auth_service = self.auth_service.clone();
        let svc = self.service.clone();

        Box::pin(async move {
            let token = auth_header.ok_or_else(|| {
                tracing::error!("Auth Token not provided");
                ServiceError::Unauthorized("Auth Token not provided".to_string())
            })?;
            let user = auth_service.token_validation(&token).await.map_err(|e| {
                tracing::error!("{}", e.to_message_display());
                e
            });

            req.extensions_mut().insert(user?);

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}

fn get_auth_header(req: &ServiceRequest) -> Option<String> {
    let result = req.headers().get(header::AUTHORIZATION)?.to_str().ok();
    result.map(|v| v.to_string())
}
