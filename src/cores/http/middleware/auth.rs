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
    sync::Arc,
};
use tracing::Instrument;

use crate::{cores::auth::Role, services::auth};

pub type Authority = Arc<Authorizer>;

pub fn new(auth_service: auth::Service) -> Authority {
    let authorizer = Authorizer {
        admin: Arc::new(AuthMiddleware {
            valid_role: Role::Admin,
            auth_service: auth_service.clone(),
        }),
        business_client: Arc::new(AuthMiddleware {
            valid_role: Role::BusinessClient,
            auth_service: auth_service.clone(),
        }),
    };
    Arc::new(authorizer)
}

pub struct Authorizer {
    admin: Arc<AuthMiddleware>,
    business_client: Arc<AuthMiddleware>,
}

impl Authorizer {
    pub fn admin(&self) -> Arc<AuthMiddleware> {
        self.admin.clone()
    }
    pub fn business_client(&self) -> Arc<AuthMiddleware> {
        self.business_client.clone()
    }
}

pub struct AuthMiddleware {
    valid_role: Role,
    auth_service: auth::Service,
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
    auth_service: auth::Service,
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
        let valid_role = self.valid_role;
        let auth_service = self.auth_service.clone();
        let svc = self.service.clone();

        Box::pin(async move {
            let user = auth_service.authorize(auth_header, valid_role).await?;
            let user_id = user.id.to_string();
            req.extensions_mut().insert(user);

            let res = svc
                .call(req)
                .instrument(tracing::info_span!("ctx", %user_id))
                .await?;
            Ok(res)
        })
    }
}

fn get_auth_header(req: &ServiceRequest) -> Option<String> {
    let result = req.headers().get(header::AUTHORIZATION)?.to_str().ok();
    result.map(|v| v.to_owned())
}
