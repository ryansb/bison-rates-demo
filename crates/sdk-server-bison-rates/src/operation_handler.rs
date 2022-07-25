// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[async_trait::async_trait]
impl<B, Fun, Fut>
    crate::server_operation_handler_trait::Handler<B, (), crate::input::CreateBisonInput> for Fun
where
    Fun: FnOnce(crate::input::CreateBisonInput) -> Fut + Clone + Send + 'static,
    Fut: std::future::Future<
            Output = Result<crate::output::CreateBisonOutput, crate::error::CreateBisonError>,
        > + Send,
    B: aws_smithy_http_server::body::HttpBody + Send + 'static,
    B::Data: Send,
    aws_smithy_http_server::rejection::RequestRejection:
        From<<B as aws_smithy_http_server::body::HttpBody>::Error>,
{
    type Sealed = crate::server_operation_handler_trait::sealed::Hidden;
    async fn call(
        self,
        req: http::Request<B>,
    ) -> http::Response<aws_smithy_http_server::body::BoxBody> {
        let mut req = axum_core::extract::RequestParts::new(req);
        use axum_core::extract::FromRequest;
        use axum_core::response::IntoResponse;
        let input_wrapper = match crate::operation::CreateBisonOperationInputWrapper::from_request(
            &mut req,
        )
        .await
        {
            Ok(v) => v,
            Err(runtime_error) => {
                return runtime_error
                    .into_response()
                    .map(aws_smithy_http_server::body::boxed);
            }
        };
        let input_inner = input_wrapper.into();
        let output_inner = self(input_inner).await;
        let output_wrapper: crate::operation::CreateBisonOperationOutputWrapper =
            output_inner.into();
        let mut response = output_wrapper.into_response();
        response.extensions_mut().insert(
            aws_smithy_http_server::extension::OperationExtension::new(
                "gov.ny.buffalo",
                "CreateBison",
            ),
        );
        response.map(aws_smithy_http_server::body::boxed)
    }
}
#[async_trait::async_trait]
impl<B, Fun, Fut>
    crate::server_operation_handler_trait::Handler<B, (), crate::input::ListBisonInput> for Fun
where
    Fun: FnOnce(crate::input::ListBisonInput) -> Fut + Clone + Send + 'static,
    Fut: std::future::Future<
            Output = Result<crate::output::ListBisonOutput, crate::error::ListBisonError>,
        > + Send,
    B: aws_smithy_http_server::body::HttpBody + Send + 'static,
    B::Data: Send,
    aws_smithy_http_server::rejection::RequestRejection:
        From<<B as aws_smithy_http_server::body::HttpBody>::Error>,
{
    type Sealed = crate::server_operation_handler_trait::sealed::Hidden;
    async fn call(
        self,
        req: http::Request<B>,
    ) -> http::Response<aws_smithy_http_server::body::BoxBody> {
        let mut req = axum_core::extract::RequestParts::new(req);
        use axum_core::extract::FromRequest;
        use axum_core::response::IntoResponse;
        let input_wrapper =
            match crate::operation::ListBisonOperationInputWrapper::from_request(&mut req).await {
                Ok(v) => v,
                Err(runtime_error) => {
                    return runtime_error
                        .into_response()
                        .map(aws_smithy_http_server::body::boxed);
                }
            };
        let input_inner = input_wrapper.into();
        let output_inner = self(input_inner).await;
        let output_wrapper: crate::operation::ListBisonOperationOutputWrapper = output_inner.into();
        let mut response = output_wrapper.into_response();
        response.extensions_mut().insert(
            aws_smithy_http_server::extension::OperationExtension::new(
                "gov.ny.buffalo",
                "ListBison",
            ),
        );
        response.map(aws_smithy_http_server::body::boxed)
    }
}
#[async_trait::async_trait]
impl<B, Fun, Fut, S>
    crate::server_operation_handler_trait::Handler<
        B,
        aws_smithy_http_server::Extension<S>,
        crate::input::CreateBisonInput,
    > for Fun
where
    S: Send + Clone + Sync + 'static,
    Fun: FnOnce(crate::input::CreateBisonInput, aws_smithy_http_server::Extension<S>) -> Fut
        + Clone
        + Send
        + 'static,
    Fut: std::future::Future<
            Output = Result<crate::output::CreateBisonOutput, crate::error::CreateBisonError>,
        > + Send,
    B: aws_smithy_http_server::body::HttpBody + Send + 'static,
    B::Data: Send,
    aws_smithy_http_server::rejection::RequestRejection:
        From<<B as aws_smithy_http_server::body::HttpBody>::Error>,
{
    type Sealed = crate::server_operation_handler_trait::sealed::Hidden;
    async fn call(
        self,
        req: http::Request<B>,
    ) -> http::Response<aws_smithy_http_server::body::BoxBody> {
        let mut req = axum_core::extract::RequestParts::new(req);
        use axum_core::extract::FromRequest;
        use axum_core::response::IntoResponse;
        let input_wrapper = match crate::operation::CreateBisonOperationInputWrapper::from_request(
            &mut req,
        )
        .await
        {
            Ok(v) => v,
            Err(runtime_error) => {
                return runtime_error
                    .into_response()
                    .map(aws_smithy_http_server::body::boxed);
            }
        };
        let state = match aws_smithy_http_server::extension::extract_extension(&mut req).await {
            Ok(v) => v,
            Err(extension_not_found_rejection) => {
                let extension = aws_smithy_http_server::extension::RuntimeErrorExtension::new(
                    extension_not_found_rejection.to_string(),
                );
                let runtime_error = aws_smithy_http_server::runtime_error::RuntimeError {
                    protocol: aws_smithy_http_server::protocols::Protocol::RestJson1,
                    kind: extension_not_found_rejection.into(),
                };
                let mut response = runtime_error.into_response();
                response.extensions_mut().insert(extension);
                return response.map(aws_smithy_http_server::body::boxed);
            }
        };
        let input_inner = input_wrapper.into();
        let output_inner = self(input_inner, state).await;
        let output_wrapper: crate::operation::CreateBisonOperationOutputWrapper =
            output_inner.into();
        let mut response = output_wrapper.into_response();
        response.extensions_mut().insert(
            aws_smithy_http_server::extension::OperationExtension::new(
                "gov.ny.buffalo",
                "CreateBison",
            ),
        );
        response.map(aws_smithy_http_server::body::boxed)
    }
}
#[async_trait::async_trait]
impl<B, Fun, Fut, S>
    crate::server_operation_handler_trait::Handler<
        B,
        aws_smithy_http_server::Extension<S>,
        crate::input::ListBisonInput,
    > for Fun
where
    S: Send + Clone + Sync + 'static,
    Fun: FnOnce(crate::input::ListBisonInput, aws_smithy_http_server::Extension<S>) -> Fut
        + Clone
        + Send
        + 'static,
    Fut: std::future::Future<
            Output = Result<crate::output::ListBisonOutput, crate::error::ListBisonError>,
        > + Send,
    B: aws_smithy_http_server::body::HttpBody + Send + 'static,
    B::Data: Send,
    aws_smithy_http_server::rejection::RequestRejection:
        From<<B as aws_smithy_http_server::body::HttpBody>::Error>,
{
    type Sealed = crate::server_operation_handler_trait::sealed::Hidden;
    async fn call(
        self,
        req: http::Request<B>,
    ) -> http::Response<aws_smithy_http_server::body::BoxBody> {
        let mut req = axum_core::extract::RequestParts::new(req);
        use axum_core::extract::FromRequest;
        use axum_core::response::IntoResponse;
        let input_wrapper =
            match crate::operation::ListBisonOperationInputWrapper::from_request(&mut req).await {
                Ok(v) => v,
                Err(runtime_error) => {
                    return runtime_error
                        .into_response()
                        .map(aws_smithy_http_server::body::boxed);
                }
            };
        let state = match aws_smithy_http_server::extension::extract_extension(&mut req).await {
            Ok(v) => v,
            Err(extension_not_found_rejection) => {
                let extension = aws_smithy_http_server::extension::RuntimeErrorExtension::new(
                    extension_not_found_rejection.to_string(),
                );
                let runtime_error = aws_smithy_http_server::runtime_error::RuntimeError {
                    protocol: aws_smithy_http_server::protocols::Protocol::RestJson1,
                    kind: extension_not_found_rejection.into(),
                };
                let mut response = runtime_error.into_response();
                response.extensions_mut().insert(extension);
                return response.map(aws_smithy_http_server::body::boxed);
            }
        };
        let input_inner = input_wrapper.into();
        let output_inner = self(input_inner, state).await;
        let output_wrapper: crate::operation::ListBisonOperationOutputWrapper = output_inner.into();
        let mut response = output_wrapper.into_response();
        response.extensions_mut().insert(
            aws_smithy_http_server::extension::OperationExtension::new(
                "gov.ny.buffalo",
                "ListBison",
            ),
        );
        response.map(aws_smithy_http_server::body::boxed)
    }
}