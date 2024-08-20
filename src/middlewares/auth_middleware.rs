use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{constants, models::response::ResponseBody};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    //type Response = ServiceResponse<B>;
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    //type Response = ServiceResponse<B>;
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // Self::Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<EitherBody<B>>, Error>> + 'static>>;
    // Output: Result<ServiceResponse<EitherBody<B>>, Error>> + 'static>
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;

        if !authenticate_pass {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(ResponseBody::new(
                    constants::MESSAGE_INVALID_TOKEN,
                    constants::EMPTY,
                ))
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            fut.await.map(ServiceResponse::map_into_left_body) // Result<ServiceResponse<EitherBody<B>>, Error>> + 'static>
        })
    }

    // Self::Future = Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, Error>> + 'static>>;
    // Output: Result<ServiceResponse<B>, Error>> + 'static>
    // fn call(&self, req: ServiceRequest) -> Self::Future {
    //     println!("Hi from start. You requested: {}", req.path());

    //     let fut = self.service.call(req);

    //     Box::pin(async move {
    //         let res = fut.await?;

    //         println!("Hi from response");
    //         Ok(res) // Result<ServiceResponse<B>, Error>> + 'static>
    //     })
    // }
}
