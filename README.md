As of Nushell [0.102.0](https://www.nushell.sh/blog/2025-02-04-nushell_0_102_0.html#generate-with-input-toc) `nu`'s [`generate`](https://www.nushell.sh/commands/docs/generate.html) command now supports stateful aggregation and downstream of streaming input. This means a plugin for things like `from sse` is no longer required. You can achieve the same result with a custom command similar to:

```nushell
export def "from sse" [] {
  lines | generate {|line pending = {data: []}|

    match ($line | split row -n 2 ":" | each { str trim }) {
      [$prefix $content] if $prefix == "id" => {
        return {next: ($pending | upsert id $content)}
      }

      [$prefix $content] if $prefix == "event" => {
        return {next: ($pending | upsert event $content)}
      }

      [$prefix $content] if $prefix == "data" => {
        return {next: ($pending | update data { append $content })}
      }

      [$empty] if $empty == "" => {
        if ($pending == {data: []}) {
          return {next: $pending}
        }
        return {next: {data: []} out: ($pending | update data { str join "\n" })}
      }

      _ => { error make {msg: $"unexpected: ($line)"} }
    }
  }
}
```


# [`nu`](https://www.nushell.sh) [streaming plugin](https://www.nushell.sh/blog/2024-03-05-nushell_0_91_0.html#plugin-protocol-overhaul-toc): `nu_plugin_from_sse`

This plugin was forked from the
[nu_plugin_example](https://github.com/nushell/nushell/blob/main/crates/nu_plugin_example/src/commands/sum.rs).

## Install with Cargo

From within nushell:

    cargo install --locked nu_plugin_from_sse
    plugin add ~/.cargo/bin/nu_plugin_from_sse
    # and then restart nu or use plugin add to activate

## Usage

`nu_plugin_from_sse` provides one command:


### `from sse`

This command transforms HTTP SSE (Server-Sent Events) into structured records with the shape:

```plaintext
{
  id: string,    // Unique identifier for the SSE event
  event: string, // Type of event
  data: string   // Data payload of the event
}
```

#### known issue: nu table buffering [#12129](https://github.com/nushell/nushell/issues/12129)

If your SSE endpoint dispatches initial events upon connection, then
pauses—awaiting rare updates—you won't see any output until the first new
update after connecting. This behavior is due to nu's [table buffering
mechanism](https://github.com/nushell/nushell/blob/65e5abaa3e48126ff730c9a59e5f6f55777a85bd/crates/nu-command/src/viewers/table.rs#L846-L875),
where a duration timeout is factored in only during active input processing.

An easy workaround for this issue is to pipe to
[`each`](https://www.nushell.sh/commands/docs/each.html).

#### example

Copy this text to your clipboard:

```
event: creatureAlert
id: 1
data: {"id":"uni123","type":"Unicorn","lat":45.4235,"lon":-75.6979,"urgency":"high","desc":"Injured near Crystal River."}


```

Use [`bp`](https://github.com/printfn/bp) to pipe it:

```nushell
bp | from sse | update data { from json }
````

![output](./docs/out.png)

#### live example

```nushell
http get https://ndyg.cross.stream/projects/enchanted-animal-rescue/rescue-feed |
    from sse |
    update data { from json }
```
