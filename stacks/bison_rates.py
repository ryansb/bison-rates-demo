from aws_cdk import CfnOutput, RemovalPolicy, Stack
from aws_cdk import aws_apigateway as apigw
from aws_cdk import aws_dynamodb as dynamo
from aws_cdk import aws_iam as iam
from constructs import Construct

from .cargo_cross import Function, rust_compile_to_zip


class BisonRates(Stack):
    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        table = Table(
            self,
            "DB",
            partition_key=dynamo.Attribute(name="pk", type=dynamo.AttributeType.STRING),
            sort_key=dynamo.Attribute(name="sk", type=dynamo.AttributeType.STRING),
        )
        table.add_global_secondary_index(
            index_name="gsi1",
            partition_key=dynamo.Attribute(name="gsi1pk", type=dynamo.AttributeType.STRING),
            sort_key=dynamo.Attribute(name="gsi1sk", type=dynamo.AttributeType.STRING),
        )

        no = Function(
            self,
            "NotFound",
            code=rust_compile_to_zip(function="not-found"),
            environment={},
            managed_policies=[],
        )

        herds = Function(
            self,
            "List",
            code=rust_compile_to_zip(function="list-bison"),
            environment={"TABLE": table.table_name},
            managed_policies=[table.ro_policy],
        )

        api = apigw.LambdaRestApi(self, "ManualApi", handler=no.function)
        api.root.add_resource("herds").add_method("GET", apigw.LambdaIntegration(handler=herds.function))

        stubbed = Function(
            self,
            "Stubbed",
            code=rust_compile_to_zip(function="stubbed-api"),
            environment={"TABLE": table.table_name},
            managed_policies=[table.ro_policy],
        )
        apigw.LambdaRestApi(self, "StubbedApi", handler=stubbed.function)

        generated = Function(
            self,
            "Generated",
            code=rust_compile_to_zip(function="generated-api"),
            environment={"TABLE": table.table_name},
            managed_policies=[table.rw_policy],
        )
        apigw.LambdaRestApi(self, "GeneratedApi", handler=generated.function)


class Table(dynamo.Table):
    """A DynamoDB table with basic safety features enabled"""

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(
            scope,
            construct_id,
            billing_mode=dynamo.BillingMode.PAY_PER_REQUEST,
            point_in_time_recovery=True,
            removal_policy=RemovalPolicy.DESTROY,
            time_to_live_attribute="ttl",
            **kwargs,
        )

        CfnOutput(self, "Name", value=self.table_name)
        CfnOutput(
            self,
            "Url",
            value=(
                f"https://{Stack.of(self).region}.console.aws.amazon.com/dynamodb/home"
                f"?region={Stack.of(self).region}#tables:selected={self.table_name}"
            ),
        )

        self.rw_policy = iam.ManagedPolicy(
            self,
            "DynamoReadWrite",
            document=iam.PolicyDocument(
                statements=[
                    iam.PolicyStatement(
                        actions=[
                            "dynamodb:BatchGetItem",
                            "dynamodb:BatchWriteItem",
                            "dynamodb:ConditionCheckItem",
                            "dynamodb:DeleteItem",
                            "dynamodb:DescribeTable",
                            "dynamodb:GetItem",
                            "dynamodb:PutItem",
                            "dynamodb:Query",
                            "dynamodb:Scan",
                            "dynamodb:UpdateItem",
                        ],
                        resources=[self.table_arn, f"{self.table_arn}/index/*"],
                    ),
                    iam.PolicyStatement(
                        effect=iam.Effect.DENY,
                        actions=[
                            "dynamodb:*",
                        ],
                        resources=[self.table_arn, f"{self.table_arn}/index/*"],
                        conditions={"Bool": {"dynamodb:FullTableScan": "true"}},
                    ),
                ]
            ),
        )

        self.ro_policy = iam.ManagedPolicy(
            self,
            "DynamoReadOnly",
            document=iam.PolicyDocument(
                statements=[
                    iam.PolicyStatement(
                        actions=[
                            "dynamodb:BatchGetItem",
                            "dynamodb:ConditionCheckItem",
                            "dynamodb:DescribeTable",
                            "dynamodb:GetItem",
                            "dynamodb:Query",
                            "dynamodb:Scan",
                        ],
                        resources=[self.table_arn, f"{self.table_arn}/index/*"],
                    ),
                    iam.PolicyStatement(
                        effect=iam.Effect.DENY,
                        actions=[
                            "dynamodb:*",
                        ],
                        resources=[self.table_arn, f"{self.table_arn}/index/*"],
                        conditions={"Bool": {"dynamodb:FullTableScan": "true"}},
                    ),
                ]
            ),
        )
