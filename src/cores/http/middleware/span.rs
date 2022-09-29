use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    time::SystemTime,
};
use tracing::Instrument;

pub fn new() -> Middleware {
    Middleware {}
}

pub struct Middleware;

impl<S, B> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ServiceMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServiceMiddleware { service }))
    }
}

pub struct ServiceMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let req_attributes = format!(
            "{} {} {} {}",
            req.peer_addr()
                .map_or(String::new(), |val| val.ip().to_string()),
            req.method(),
            req.path(),
            req.query_string(),
        );
        let fut = self.service.call(req);

        Box::pin(
            async move {
                let start_time = SystemTime::now();
                tracing::info!("HTTP Request Started: {}", req_attributes);

                let res = fut.await?;

                let end_time = SystemTime::now();
                let response_time = end_time
                    .duration_since(start_time)
                    .unwrap_or_default()
                    .as_millis();
                tracing::info!("HTTP Request finished: {}ms", response_time);
                Ok(res)
            }
            .instrument(tracing::info_span!(
                "context",
                request_id = ?uuid::Uuid::new_v4()
            )),
        )
    }
}
