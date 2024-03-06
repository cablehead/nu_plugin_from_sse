# [`nu`](https://www.nushell.sh) [streaming plugin](https://www.nushell.sh/blog/2024-03-05-nushell_0_91_0.html#plugin-protocol-overhaul-toc): `nu_plugin_from_sse`

This plugin was forked from the
[nu_plugin_stream_example](https://github.com/nushell/nushell/tree/main/crates/nu_plugin_stream_example).

With Nushell
[0.91.0](https://www.nushell.sh/blog/2024-03-05-nushell_0_91_0.html) [@devyn](https://github.com/devyn) has added support
for [plugins that operate on streams](https://www.nushell.sh/blog/2024-03-05-nushell_0_91_0.html#plugin-protocol-overhaul-toc)

This makes possible plugin's like `nu_plugin_from_sse` which parses a stream of HTTP server sent events.

`nu_plugin_from_sse` provides one command:


## `from sse`

This command transforms HTTP SSE (Server-Sent Events) into structured records with the shape:

```plaintext
{
  id: string,    // Unique identifier for the SSE event
  event: string, // Type of event
  data: string   // Data payload of the event
}
```

### known issue: nu table buffering

If your SSE endpoint dispatches initial events upon connection, then
pauses—awaiting rare updates—you won't see any output until the first new
update after connecting. This behavior is due to nu's [table buffering
mechanism](https://github.com/nushell/nushell/blob/65e5abaa3e48126ff730c9a59e5f6f55777a85bd/crates/nu-command/src/viewers/table.rs#L846-L875),
where a duration timeout is factored in only during active input processing.

An easy workaround for this issue is to pipe to
[`each`](https://www.nushell.sh/commands/docs/each.html).

### example

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

### live example

```nushell
http get https://ndyg.cross.stream/projects/enchanted-animal-rescue/rescue-feed |
    from sse |
    update data { from json }
```
