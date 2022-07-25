import os
import subprocess
import sys
from functools import lru_cache
from pathlib import Path
from typing import Any, Optional

from aws_cdk import RemovalPolicy, Stack
from aws_cdk import aws_iam as iam
from aws_cdk import aws_lambda as func
from aws_cdk import aws_logs as logs
from constructs import Construct


def _mb_size(path: os.PathLike):
    """Given a file path, convert to humanized string "X.YZMB"

    Uses 2^20 bytes == 1 MB"""
    return f"{path.stat().st_size / 2**20:.2f}MB"


@lru_cache
def rust_compile(rust_dir: Path) -> Path:
    """Given a function name (per cargo-lambda) and rust project, build a Lambda asset"""
    # nosemgrep: python.lang.security.audit.dangerous-subprocess-use.dangerous-subprocess-use

    profile = ["--release"]
    if os.getenv("BISON_ENV", "").lower() in ("p", "prod"):
        profile = ["--profile", "prod"]
    elif os.getenv("BISON_ENV", "").lower() in ("d", "dev"):
        profile = ["--profile", "dev"]

    command = "cargo lambda build --arm64 --output-format zip".split() + profile
    print(" ".join(command), file=sys.stderr),
    if rc := subprocess.check_call(command, cwd=rust_dir):
        print(f"error: cargo build failed with exit code {rc}", file=sys.stderr)
        raise Exception("Could not build asset")
    return rust_dir / "target" / "lambda"


def rust_compile_to_zip(function: str, rust_dir: Path = Path(__file__).parent.parent) -> func.AssetCode:
    """Given a function name (per cargo-lambda) and rust project, build a Lambda asset"""

    zip_path = rust_compile(rust_dir) / function / "bootstrap.zip"

    print(f"rust asset={function}/bootstrap.zip size={_mb_size(zip_path)}", file=sys.stderr)

    return func.AssetCode.from_asset(str(zip_path))


class Function(Construct):
    log_group: Optional[logs.LogGroup]
    role: iam.Role
    function: func.Function

    def __init__(
        self,
        scope: Construct,
        construct_id: str,
        code: func.Code,
        managed_policies: list[iam.IManagedPolicy],
        environment: dict[str, Any],
        handler: str = "Runtime.UnusedName",
        **kwargs,
    ) -> None:
        super().__init__(scope, construct_id, **kwargs)
        stack = Stack.of(self)
        self.role = iam.Role(
            self,
            "Role",
            assumed_by=iam.ServicePrincipal("lambda.amazonaws.com"),
            inline_policies={
                "Logs": iam.PolicyDocument(
                    statements=[
                        iam.PolicyStatement(
                            actions=["logs:CreateLogGroup"],
                            resources=["*"],
                            effect=iam.Effect.DENY,
                        ),
                        iam.PolicyStatement(
                            actions=["logs:CreateLogStream", "logs:PutLogEvents"],
                            resources=[
                                f"arn:{stack.partition}:logs:{stack.region}:{stack.account}:log-group:/aws/lambda/{stack.stack_name}-{construct_id}*",
                            ],
                        ),
                    ]
                )
            },
            managed_policies=managed_policies,
        )

        self.function = func.Function(
            self,
            "Func",
            handler=handler,
            runtime=func.Runtime.PROVIDED_AL2,
            architecture=func.Architecture.ARM_64,
            code=code,
            environment={"RUST_BACKTRACE": "1"} | environment,
            role=self.role,
        )

        self.log_group = logs.LogGroup(
            self,
            "Logs",
            log_group_name=f"/aws/lambda/{self.function.function_name}",
            removal_policy=RemovalPolicy.DESTROY,
            retention=logs.RetentionDays.THREE_DAYS,
        )
