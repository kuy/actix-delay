use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, Result,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use futures_timer;
use std::{cell::RefCell, pin::Pin, rc::Rc, task, time::Duration};

pub struct Delay {
    pub dur: u64,
}

impl Delay {
    pub fn new(dur: u64) -> Self {
        Self { dur }
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new(1000)
    }
}

impl<S, B> Transform<S> for Delay
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DelayMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DelayMiddleware {
            service: Rc::new(RefCell::new(service)),
            dur: self.dur,
        })
    }
}

pub struct DelayMiddleware<S> {
    service: Rc<RefCell<S>>,
    dur: u64,
}

impl<S, B> Service for DelayMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let dur = Duration::from_millis(self.dur);
        let mut service = self.service.clone();
        Box::pin(async move {
            futures_timer::Delay::new(dur).await;
            Ok(service.call(req).await?)
        })
    }
}
