import subprocess

from aws_cdk import App

from . import HelloBuffalo
from .bison_rates import BisonRates

if __name__ == "__main__":
    # prune old cdk files
    # nosemgrep: python.lang.security.audit.dangerous-subprocess-use.dangerous-subprocess-use
    subprocess.check_call("find cdk.out -mindepth 1 -mmin +15 -delete".split(" "))

    app = App()
    HelloBuffalo(app, "HiBuffalo")
    BisonRates(app, "BisonRates")
    app.synth()
