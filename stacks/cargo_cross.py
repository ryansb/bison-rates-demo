import os
import subprocess
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Optional

from aws_cdk import (
    RemovalPolicy,
    Stack,
    aws_iam as iam,
    aws_lambda as func,
    aws_logs as logs,
)
from constructs import Construct


def _mb_size(path: os.PathLike):
    """Given a file path, convert to humanized string "X.YZMB"

    Uses 2^20 bytes == 1 MB"""
    return f"{path.stat().st_size / 2**20:.2f}MB"


def rust_compile_to_zip(rust_dir: Path, function: str):
    # nosemgrep: python.lang.security.audit.dangerous-subprocess-use.dangerous-subprocess-use
    if rc := subprocess.check_call(
        "cargo lambda build --arm64 --release --output-format zip".split(),
        cwd=rust_dir,
    ):
        print(f"error: cargo build failed with exit code {rc}", file=sys.stderr)
        raise Exception("Could not build asset")

    zip_path = rust_dir / "target" / "lambda" / function / "bootstrap.zip"

    epoch_force = int(datetime(year=2020, month=2, day=20).timestamp())
    # force mtime/ctime to avoid deploying unchanged artifact
    os.utime(zip_path, (epoch_force, epoch_force))

    print(f"rust asset: zipped_size={_mb_size(zip_path)}", file=sys.stderr)

    return zip_path


class Function(Construct):
    log_group: Optional[logs.LogGroup]
    role: iam.Role
    function: func.Function

    def __init__(
        self,
        scope: Construct,
        construct_id: str,
        code: func.Code,
        handler: str,
        managed_policies: list[iam.IManagedPolicy],
        environment: dict[str, Any],
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
            runtime=func.Runtime.PYTHON_3_9,
            code=code,
            environment=environment,
            role=self.role,
        )

        self.log_group = logs.LogGroup(
            self,
            "Logs",
            log_group_name=f"/aws/lambda/{self.function.function_name}",
            removal_policy=RemovalPolicy.DESTROY,
            retention=logs.RetentionDays.THREE_DAYS,
        )
