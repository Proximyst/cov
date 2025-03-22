# Security

You can submit security reports to <mariell at mardroemmar dot dev>.

Security fixes will be issued to the latest version of `cov` only. It is your responsibility to keep your version of `cov` up-to-date.
We enable you to do so by providing a well-documented build process, and pre-built binaries and Docker images for those who do not wish to build the project themselves.

## Tenets

The following tenets guide the security of `cov`:

1. **Secure by default**: the default configuration and deployment of `cov` should be secure enough for most large organisations.
2. **Reproducible**: the code and build process of `cov` should always be possible to reproduce.
   There will be no binaries checked in without both a generation script and a workflow to verify it is always valid.
   The code should be enough to create all necessary binaries, if any, and build the entire software without any use of pre-built binaries from us.
3. **Open source**: `cov` is open source. We will not hide any of our code, dependencies, infrastructure, or processes.
   Any organisation can audit `cov` and its dependencies, and can fork or modify `cov` however they see fit.
