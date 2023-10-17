<div id="header" align="center">
    <img src="/packages/demo/public/logo.svg" width="80%" />
    <h1>The Thrax Programming Language</h1>
</div>

## What is it?

Thrax is a little toy programming language I threw together in my free time near the end of 2022.
My primary motivation was to learn more of the fine-point details of how dynamically-typed interpreted languages worked (JavaScript, Python and Ruby).
In the future, I intend of doing a full write-up of the process and what it taught me on the way.

I was also working on a [project](https://hdiharmony.web.app/) at [work](https://archytasinc.com/) that involved low-level parsing and interpretation of JavaScript code.
While we were using SWC for the actual parsing, writing my own parser gave me a much better understanding of how everything fit together.

## [Live Sandbox](https://thrax.elijahpotter.dev)

You don't have to download or install anything to start using Thrax.
Play around with the language and experiment with a few demo scripts at the [interactive sandbox](https://thrax.elijahpotter.dev).

## Should You Use It?

While it should be trivial to add Thrax to your Rust project, I do not recommend doing so for anything important.
At the moment, I do not have the time to maintain or otherwise continue working on Thrax.

## Local Development

Want to explore Thrax on your own machine?

As long as you have Node.js, Rust, and Yarn, you can run and build everything using:

```bash
./build.sh --release # or --debug
```

Or, to run all unit tests:

```bash
./test.sh
```

## What's in a Name?

In Ancient Greece, "Thrax" usually meant "from Thrace."
In this case, however, the name is a reference to one of the earliest grammarians, [Dionysis Thrax](https://en.wikipedia.org/wiki/Dionysius_Thrax).
Just like how Dionysis Thrax had no formal education on grammar, and thus had to discover everything himself, I found myself discovering different aspects of recursive descent parsers and dynamic interpreters on my own.
Thus the name, "Thrax".
