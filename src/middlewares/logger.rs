use actix_web::{dev::{Transform, Service, ServiceRequest, ServiceResponse}, Error};
use std::rc::Rc;
use std::future::{Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use log::{info, error};
use crate::utils::get_user_id_from_request;
use chrono::Utc;
use backtrace::Backtrace;
// 请求日志中间件
pub struct RequestLogger;

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequestLoggerService { service: Rc::new(service) }))
    }
}

pub struct RequestLoggerService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let start_time = Instant::now();
        let user_id = get_user_id_from_request(&req.request());
        let method = req.method().to_string();
        let path = req.path().to_string();

        Box::pin(async move {
            // 执行后续服务
            let result = service.call(req).await;

            // 计算请求持续时间
            let duration = start_time.elapsed().as_millis() as u64;

            // 处理结果
            match result {
                Ok(response) => {
                    let status_code = response.status().as_u16();

                    // 记录成功请求
                    info!(
                        "REQUEST_LOG: time={}, method={}, path={}, user_id={:?}, status={}, duration={}ms",
                        Utc::now().to_rfc3339(),
                        method,
                        path,
                        user_id,
                        status_code,
                        duration
                    );

                    Ok(response)
                }
                Err(err) => {
                    // 记录失败请求
                    let error_message = format!("{:?}", err);
                    let backtrace = Backtrace::new();
                    let stack_trace = format!("{:?}", backtrace);

                    error!(
                        "REQUEST_LOG: time={}, method={}, path={}, user_id={:?}, status=500, duration={}ms, error={}, stack_trace={}",
                        Utc::now().to_rfc3339(),
                        method,
                        path,
                        user_id,
                        duration,
                        error_message,
                        stack_trace
                    );

                    Err(err)
                }
            }
        })
    }
}