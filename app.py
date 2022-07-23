import aws_cdk as cdk
import subprocess

# prune old cdk files
# nosemgrep: python.lang.security.audit.dangerous-subprocess-use.dangerous-subprocess-use
subprocess.check_call("find cdk.out -mindepth 1 -mmin +15 -delete".split(" "))

from stacks import BuffaloRsStack

app = cdk.App()
BuffaloRsStack(app, "BuffaloRsStack")
app.synth()
