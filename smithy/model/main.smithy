$version: "1.0"

namespace gov.ny.buffalo

use aws.protocols#restJson1
use gov.ny.common#AccessDeniedException
use gov.ny.common#BadRequestException
use gov.ny.common#GatewayTimeoutException
use gov.ny.common#InternalFailureException
use gov.ny.common#NotAcceptableException
use gov.ny.common#NotImplementedException
use gov.ny.common#RequestTooLargeException
use gov.ny.common#ResourceConflictException
use gov.ny.common#ResourceNotFoundException
use gov.ny.common#SerializationException
use gov.ny.common#ServiceUnavailableException
use gov.ny.common#UnknownOperationException
use gov.ny.common#UnsupportedMediaTypeException
use smithy.framework#ValidationException

@restJson1
@cors({
  additionalAllowedHeaders: [
    "Content-Type",
  ],
  additionalExposedHeaders: [
    "X-Bison-Version",
    "X-Amzn-ErrorType"
  ]
})
@title("Rank members of the herd")
service BisonRates {
    version: "2022-07-07",
    operations: [CreateBison, ListBison],
    errors: [
        AccessDeniedException,
        BadRequestException,
        GatewayTimeoutException,
        InternalFailureException,
        NotAcceptableException,
        NotImplementedException,
        RequestTooLargeException,
        ResourceConflictException,
        ResourceNotFoundException,
        SerializationException,
        ServiceUnavailableException,
        UnknownOperationException,
        UnsupportedMediaTypeException,
        ValidationException
    ]
}

@length(min: 20, max: 30)
@pattern("^[A-Z0-9]+$")
string BisonId

string ServiceVersion

@length(min: 5, max: 60)
@pattern("^[A-Za-z][A-Za-z0-9 ]*[A-Za-z0-9]?$")
string Name

@range(min: 1, max: 100)
integer HerdRank

list BisonTags {
    member: BisonTag
}

structure BisonTag { key: String, value: String}