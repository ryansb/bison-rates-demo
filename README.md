
# Bison Rates

This is a demo project for the Buffalo Rust meetup.

See `presentation/` for slides.

To use `pipenv` environment locally, run:

```
$ pipenv install -d
```

At this point you can now synthesize the CloudFormation template for this code.

```
$ npx cdk synth
```

To add additional dependencies, for example other CDK libraries, run `pipenv install [package name]` command.

## Useful commands

 * `npx cdk ls`          list all stacks in the app
 * `npx cdk synth`       emits the synthesized CloudFormation template
 * `npx cdk deploy`      deploy this stack to your default AWS account/region
 * `npx cdk diff`        compare deployed stack with current state
 * `npx cdk docs`        open CDK documentation
