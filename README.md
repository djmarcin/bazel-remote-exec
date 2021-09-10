# bazel-remote-exec
Demo of remote execution over multiple platforms

This demonstrates how to use bazel remote execution on very simple rules.
Of particular note is how it cross-compiles for the Apple M1 platform (macos-arm64)
from the macos-x86_64 platform and can successfully run tests on the M1
platform without building there.

(bazel-buildfarm)[https://github.com/bazelbuild/bazel-buildfarm] configs are included
in the buildfarm directory. Note that the server is set up to reference a separate
CAS server, such as (bazel-remote)[https://github.com/buchgr/bazel-remote] over grpc.

For example (after setting up buildfarm) run the following command. The `-s` flag displays
the execution platform selected for each action. You can see that the `:get_platform` genrule
executed on the `macos-x86_64` platform while the test executed on the `macos-arm64` platform.
```
$ bazel clean && bazel test -s --noremote_accept_cached --config=remote_macos_arm64 //
:get_platform_test
INFO: Starting clean.
INFO: Invocation ID: 01088ec4-bac7-4c25-8a85-502118a41f91
INFO: Analyzed target //:get_platform_test (26 packages loaded, 370 targets configured).
INFO: Found 1 test target...
SUBCOMMAND: # //:get_platform [action 'Executing genrule //:get_platform', configuration: e92345fa7240a6b9a38e2d16d527940c65a151013e0175c50f0f60522d2c6c55, execution platform: //platforms:macos-x86_64]
(cd /private/var/tmp/_bazel_djmarcin/a56bb6d3e1e326e0a1477913987aac53/execroot/bazel-remote-exec && \
  exec env - \
    PATH=/bin:/usr/bin:/usr/local/bin \
  /bin/bash -c 'source external/bazel_tools/tools/genrule/genrule-setup.sh; uname -a > bazel-out/darwin-fastbuild/bin/platform.txt')
SUBCOMMAND: # //:get_platform_test [action 'Testing //:get_platform_test', configuration: e92345fa7240a6b9a38e2d16d527940c65a151013e0175c50f0f60522d2c6c55, execution platform: //platforms:macos-arm64]
(cd /private/var/tmp/_bazel_djmarcin/a56bb6d3e1e326e0a1477913987aac53/execroot/bazel-remote-exec && \
  exec env - \
    EXPERIMENTAL_SPLIT_XML_GENERATION=1 \
    JAVA_RUNFILES=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    PATH=/bin:/usr/bin:/usr/local/bin \
    PYTHON_RUNFILES=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    RUNFILES_DIR=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    RUN_UNDER_RUNFILES=1 \
    TEST_BINARY=./get_platform_test \
    TEST_INFRASTRUCTURE_FAILURE_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.infrastructure_failure \
    TEST_LOGSPLITTER_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.raw_splitlogs/test.splitlogs \
    TEST_PREMATURE_EXIT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.exited_prematurely \
    TEST_SIZE=medium \
    TEST_SRCDIR=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    TEST_TARGET=//:get_platform_test \
    TEST_TIMEOUT=300 \
    TEST_TMPDIR=_tmp/7f5e41dad1cff3006d6c3d0a6d6485cd \
    TEST_UNDECLARED_OUTPUTS_ANNOTATIONS=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest/ANNOTATIONS \
    TEST_UNDECLARED_OUTPUTS_ANNOTATIONS_DIR=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest \
    TEST_UNDECLARED_OUTPUTS_DIR=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs \
    TEST_UNDECLARED_OUTPUTS_MANIFEST=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest/MANIFEST \
    TEST_UNDECLARED_OUTPUTS_ZIP=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs/outputs.zip \
    TEST_UNUSED_RUNFILES_LOG_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.unused_runfiles_log \
    TEST_WARNINGS_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.warnings \
    TEST_WORKSPACE=bazel-remote-exec \
    TZ=UTC \
    XML_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.xml \
  external/bazel_tools/tools/test/test-setup.sh ./get_platform_test)
SUBCOMMAND: # //:get_platform_test [action 'Testing //:get_platform_test', configuration: e92345fa7240a6b9a38e2d16d527940c65a151013e0175c50f0f60522d2c6c55, execution platform: //platforms:macos-arm64]
(cd /private/var/tmp/_bazel_djmarcin/a56bb6d3e1e326e0a1477913987aac53/execroot/bazel-remote-exec && \
  exec env - \
    EXPERIMENTAL_SPLIT_XML_GENERATION=1 \
    JAVA_RUNFILES=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    PATH=/bin:/usr/bin:/usr/local/bin \
    PYTHON_RUNFILES=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    RUNFILES_DIR=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    RUN_UNDER_RUNFILES=1 \
    TEST_BINARY=./get_platform_test \
    TEST_INFRASTRUCTURE_FAILURE_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.infrastructure_failure \
    TEST_LOGSPLITTER_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.raw_splitlogs/test.splitlogs \
    TEST_NAME=//:get_platform_test \
    TEST_PREMATURE_EXIT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.exited_prematurely \
    TEST_SHARD_INDEX=0 \
    TEST_SIZE=medium \
    TEST_SRCDIR=bazel-out/darwin-fastbuild/bin/get_platform_test.runfiles \
    TEST_TARGET=//:get_platform_test \
    TEST_TIMEOUT=300 \
    TEST_TMPDIR=_tmp/7f5e41dad1cff3006d6c3d0a6d6485cd \
    TEST_TOTAL_SHARDS=0 \
    TEST_UNDECLARED_OUTPUTS_ANNOTATIONS=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest/ANNOTATIONS \
    TEST_UNDECLARED_OUTPUTS_ANNOTATIONS_DIR=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest \
    TEST_UNDECLARED_OUTPUTS_DIR=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs \
    TEST_UNDECLARED_OUTPUTS_MANIFEST=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs_manifest/MANIFEST \
    TEST_UNDECLARED_OUTPUTS_ZIP=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.outputs/outputs.zip \
    TEST_UNUSED_RUNFILES_LOG_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.unused_runfiles_log \
    TEST_WARNINGS_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.warnings \
    TEST_WORKSPACE=bazel-remote-exec \
    TZ=UTC \
    XML_OUTPUT_FILE=bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.xml \
  external/bazel_tools/tools/test/generate-xml.sh bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.log bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.xml 0 1)
FAIL: //:get_platform_test (see /private/var/tmp/_bazel_djmarcin/a56bb6d3e1e326e0a1477913987aac53/execroot/bazel-remote-exec/bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.log)
INFO: From Testing //:get_platform_test:
==================== Test output for //:get_platform_test:
Platform is: Darwin mac-mini.local 20.5.0 Darwin Kernel Version 20.5.0: Sat May  8 05:10:31 PDT 2021; root:xnu-7195.121.3~9/RELEASE_ARM64_T8101 arm64
================================================================================
Target //:get_platform_test up-to-date:
  bazel-bin/get_platform_test
INFO: Elapsed time: 2.848s, Critical Path: 1.15s
INFO: 6 processes: 3 internal, 3 remote.
INFO: Build completed, 1 test FAILED, 6 total actions
//:get_platform_test                                                     FAILED in 0.7s
  /private/var/tmp/_bazel_djmarcin/a56bb6d3e1e326e0a1477913987aac53/execroot/bazel-remote-exec/bazel-out/darwin-fastbuild/testlogs/get_platform_test/test.log

INFO: Build completed, 1 test FAILED, 6 total actions
```