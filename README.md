# bazel-remote-exec
Demo of remote execution over multiple platforms

(bazel-buildfarm)[https://github.com/bazelbuild/bazel-buildfarm] configs are included
in the buildfarm directory. Note that the server is set up to reference a separate
CAS server, such as (bazel-remote)[https://github.com/buchgr/bazel-remote] over grpc.

This repo is attempting to demonstrate how one might go about being able to build the
same targets for multiple different architectures using remote execution.

One thing that is currently not working is the ability to cross-build actions using
one platform that execute on another. After many attempts, though bazel supports cross
compilation, it's not clear that it actually supports testing the cross-compiled
binaries. In particular, the `--config=remote_macos_arm64` ends up building and running
