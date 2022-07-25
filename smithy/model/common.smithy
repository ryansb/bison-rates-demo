namespace gov.ny.common

/// Human readable exception message.
string ExceptionMessage

/// Classification of the exception type.
string ExceptionCode

// ---------- Common Exceptions ----------
// Many of those common exceptions come from API Gateway.
// Taken from https://docs.aws.amazon.com/apigateway/latest/developerguide/supported-gateway-response-types.html

/// The server response for authorization failure.
@error("client")
@httpError(403)
structure AccessDeniedException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server cannot process the request due to an apparent client error.
@error("client")
@httpError(400)
structure BadRequestException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server was acting as a gateway or proxy and did not receive a timely response from the upstream server.
@error("server")
@httpError(504)
structure GatewayTimeoutException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server response when an unexpected error occurred while processing request.
@error("server")
@httpError(500)
structure InternalFailureException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

// Can be thrown by Smithy framework itself
/// The server response when the requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request.
@error("client")
@httpError(406)
structure NotAcceptableException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server either does not recognize the request method, or it lacks the ability to fulfil the request.
@error("server")
@httpError(501)
structure NotImplementedException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server response for the request too large error.
@error("client")
@httpError(413)
structure RequestTooLargeException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server could not process the request because of conflict in the current state of the resource.
@error("client")
@retryable
@httpError(409)
structure ResourceConflictException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server response when the specified resource cannot be found after an API request passes authentication and authorization.
@error("client")
@httpError(404)
structure ResourceNotFoundException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

// Can be thrown by Smithy framework itself
/// The server response when attempting to serialize the request.
@error("client")
@httpError(400)
structure SerializationException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

/// The server cannot handle the request due to technical reasons.
@error("server")
@retryable
@httpError(503)
structure ServiceUnavailableException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

// Can be thrown by Smithy framework itself
/// The server response when the operation specified could not be found.
@error("client")
@httpError(404)
structure UnknownOperationException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}

// Can be thrown by Smithy framework itself
/// The server response when a payload is of an unsupported media type.
@error("client")
@httpError(415)
structure UnsupportedMediaTypeException {
  @required
  message: ExceptionMessage,
  code: ExceptionCode
}
