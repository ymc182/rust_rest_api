use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, ResponseError,
};
use futures_util::future::LocalBoxFuture;

use crate::error::api_error::ApiError;
///a simple logger that logs the request path

pub struct ApiKeyAuth {
    expected_key: String,
}

impl ApiKeyAuth {
    pub fn new(expected_key: String) -> Self {
        Self { expected_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ApiKeyAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ApiKeyAuthMiddleware {
            service,
            expected_key: self.expected_key.clone(),
        }))
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: S,
    expected_key: String,
}

impl<S, B> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
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
        let headers = req.headers();
        let x_api_key = headers
            .get("x-api-key")
            .map(|v| v.to_str().unwrap_or("none"));

        let is_authorized = x_api_key == Some(self.expected_key.as_str());

        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;

            if !is_authorized {
                println!("Unauthorized");
                return Err(Error::from(ApiError::Unauthorized));
            }

            Ok(res)
        })
    }
}
