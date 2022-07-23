from aws_cdk import (
    # Duration,
    Stack,
    aws_lambda as compute,
)
from pathlib import Path
from constructs import Construct
from .cargo_cross import Function, rust_compile_to_zip


class BuffaloRsStack(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        artifact = rust_compile_to_zip(
            Path(__file__).parent.parent / "bison-rates", function="hello-http"
        )

        Function(
            self,
            "Hello",
            code=compute.AssetCode.from_asset(str(artifact.absolute())),
            environment={},
            managed_policies=[],
            handler="UNUSED",
        )
