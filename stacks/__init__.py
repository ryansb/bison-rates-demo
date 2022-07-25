from aws_cdk import Stack
from aws_cdk import aws_apigateway as apigw
from constructs import Construct

from .cargo_cross import Function, rust_compile_to_zip


class HelloBuffalo(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        apigw.LambdaRestApi(
            self,
            "Api",
            handler=Function(
                self,
                "Hello",
                code=rust_compile_to_zip(
                    function="hello-http",
                ),
                environment={},
                managed_policies=[],
            ).function,
        )
