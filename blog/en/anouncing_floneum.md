# Anoucing Floneum

## What

Floneum is an approachable graph editor for local large language model workflows. It lets you build a workflow that works consistantly for your specific usecase.

## Why

### Current tools have either a high barrier to entry or are difficult to control

The chat gpt interface is easy to get into, but you quickly run into problems when you want structured output. For example, you cannot make it respond with the names of exactly five countries that are in europe. Instead you need to either trust that the model with understand you, or edit the output.

On the other side of the spectrum, [langchain](https://github.com/hwchase17/langchain) allows you to create your own workflows, but requires a few things of it's users:

1. It requires you to install developer tools like python and CUDA to get started.
2. You need to build your workflow in python code which is not feasible for non-developers.
3. Plugins are not sandboxed. In langchain, plugins are arbitary python code. They have arbitrary access to the file system. If you want to run a plugin, you need to trust that the plugin does not contain malware.

## How

### Ease of use

### Extensability

You can extend Floneum with plugins that are sandboxed from there enviorment. This is enabled by [WASM](https://webassembly.org/).

## What's Next?

- API based model intigrations (Chat Gpt, etc.)
- Improved Plugin System (plugins as values, improved logic support, etc.)
- Package manager
- Other model types (image generation, classification, etc.)
