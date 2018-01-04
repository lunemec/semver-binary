# semver-binary

Semver-binary can be used to manipulate version data from CLI. This is intended mainly
for CI scripts.

## Usage
Basic minor version bump with added `alpha` pre-release and `CI + git commit ` info.
```bash
$ semver-binary "1.1.1" -m --pre "alpha" --meta "CI" --meta "6fe444c"
1.2.0-alpha+CI.6fe444c
```

Suppose the following versions:
```notrust
production    1.2.3
testing       1.3.0-beta
testing CI    $testing+CI.git_ci_id
```

You can use semver to automatically create these versions correctly for you:
production -> testing
```bash
$ PRODUCTION="1.2.3"
$ semver $PRODUCTION -m --pre beta
"1.3.0-beta"
```

testing -> CI autodeploy
```bash
$ TESTING="1.3.0-beta"
$ semver $TESTING --meta CI --meta 6fe444c  # git rev-parse --short HEAD
"1.3.0-beta+CI.6fe444c"
```

These versions can be used for your Dockerfile, docker image tag, application version (for API/binary versioning)
and others.

## Building
```
cargo build --release
```