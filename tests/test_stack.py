import aws_cdk as core
import aws_cdk.assertions as assertions

from stacks import HelloBuffalo


# example tests. To run these tests, uncomment this file along with the example
# resource in buffalo_rs/buffalo_rs_stack.py
def test_sqs_queue_created():
    app = core.App()
    stack = HelloBuffalo(app, "buffalo-rs")
    template = assertions.Template.from_stack(stack)

    template.has_resource_properties(
        "AWS::Logs::LogGroup",
        {"RetentionInDays": 3},
    )
