# [SOLVED] RUST PROJECT FAILS ON MAKE USING CARGO

Posted on May 22, 2021 by Sam
I am working on a project and it is using Rust as backend language. I tried to build it using make watch which under the hood calls cargo run. It failed with error as shown below.

```
RUST_BACKTRACE=1 RUST_LOG=debug cargo watch -x 'run'
[Running 'cargo run']
    Updating git repository `https://github.com/samundra/api-rs`
error: failed to get `api-common` as a dependency of package `api-rs v0.1.0 (/Users/samundra/personal/api-rs)`

Caused by:
  failed to load source for dependency `api-common`

Caused by:
  Unable to update https://github.com/samundra/api-rs?branch=develop#234b4d22

...
...

Caused by:
  failed to authenticate when downloading repository

  * attempted to find username/password via git's `credential.helper` support, but failed

  if the git CLI succeeds then `net.git-fetch-with-cli` may help here
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli

Caused by:
  failed to acquire username/password from local configuration
[Finished running. Exit status: 101]
```
Here api-common is private github repository that has been specified as dependency of api-rs package. During cargo run git tries to clone project api-common but git fails to do so. Because api-common is hosted as private repo and git doesn’t have access to it.

On macos, usually osxkeychain stores credentials and git can retrieve keys from there. But as you see we are using https protocol schema because using ssh do not work with cargo see https://github.com/rust-lang/cargo/issues/5227

Here is how I got it working.

```
export CARGO_NET_GIT_FETCH_WITH_CLI=true
```

# Create credential and store in local

— generate personal access token

— visit https://github.com/settings/tokens

— click on Personal Access Token

see documentation here https://git-scm.com/book/en/v2/Git-Tools-Credential-Storageenter as follow

a. use command git credential-store --file ~/git.store store

b. enter as follow

```
protocol=https
host=github.com
username={your github username}
password={personal access token}
[blank line]
```

c. Enter blank line after password

d. it has been saved to git config

e. To test it type `git credential-store –file ~/git.store get

```host=github.com
username={your github username}
[blank line]
```

f. It should show your stored credentials. It means git can find it.

Now, try to build again with make watch and it worked fine.