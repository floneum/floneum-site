# Announcing Floneum

Floneum is an approachable graph editor for local AI workflows. It lets you guide large language models to build a workflow that works consistently for your specific use case.

__Current tools have either a high barrier to entry or are difficult to control__

The chat gpt interface is easy to get into, but you quickly run into problems when you want a structured workflow. For example, you ask chat gpt to search through files to find any `.txt` files and then upload any files that look related travel. Instead you need to manually interact with external tools .

On the other side of the spectrum, [langchain](https://github.com/hwchase17/langchain) allows you to create your own workflows, but requires a few things of it's users:

1. It requires you to install developer tools like python and CUDA to get started.
2. You need to build your workflow in python code which is not feasible for non-developers.
3. Plugins are not sandboxed. In langchain, other users code is written in python. Any libraries you use have access to the file system. If you want to use a library for some part of your workflow, you need to trust that the library does not contain malware.

FLoneum is a single executable that runs models locally. No further installation steps are necessary.

Floneum provides a simple, graph based editor that allows you to build workflows. You shouldn't need to know how to program to use Floneum.

If the built-in plugins don't fit your application, you can extend Floneum with plugins that are entirely sandboxed from your environment. Because all plugins are run through a [WASM](https://webassembly.org/) compiler, they can only access resources within their sandbox.

## What's Next?

- API based model integrations (Chat Gpt, etc.)
- Improved Plugin System (plugins as values, control flow, etc.)
- Package manager
- Other model types (image generation, classification, etc.)
