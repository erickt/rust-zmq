This is the C examples from the zeromq guide (http://zguide.zeromq.org/),
written in Rust.

The coding guidelines for porting examples is:

- Faithfully implement the semantics from the guide, and also follow
  their code structure as far as reasonable. This allows for knowledge
  transfer between languages.

- Use ideomatic Rust, and use the most fitting abstraction offered by
  the bindings. We want these to also highlight the differences in API
  usage and the higher-level abstractions provided, if applicable.

Besides giving potential rust-zmq users an impression of the bindings,
these examples are also intended as a "proving ground" for API
additions and changes. Ideally, an API change should be reflected in
changes to the examples that improve code quality (by whatever
metric).
