# Helm Lint(r)

This is a Helm plugin to run a series of tests to verify that a Helm chart is well-formed. It works
like `helm lint`, except that it runs locally, has more output options, and is quite a bit faster.

It's also written in Rust, hence the name.

## Usage

Runs a series of tests to verify that a Helm chart is well-formed.

If the linter encounters things that will cause the chart to fail installation, it will emit
[ERROR] messages. If it encounters issues that break with convention or recommendation, it will
emit [WARNING] messages.

```
$ helm lintr [flags] CHART
```

## Install

```
$ helm plugin install https://github.com/bacongobbler/helm-lintr
```

The above will fetch the latest source code of `helm lintr` and compile it. You will need to have Rust installed
in order to build the plugin.

### Developer (From Source) Install

If you would like to handle the build yourself, instead of fetching a binary, this is how recommend doing it.

First, set up your environment:

- You need to have [Rust](https://www.rust-lang.org/) installed

Clone this repo, and build from source:

```
$ git clone https://bacongobbler/helm-lintr
$ cd helm-lintr
$ make
$ make install
```

That last command will take the plugin and install it into ~/.helm/plugins/helm-lintr.
