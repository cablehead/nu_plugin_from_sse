# [`nu`](https://www.nushell.sh) [streaming plugin](https://github.com/nushell/nushell/pull/11911): `nu_plugin_from_sse`

This plugin was forked from the
[nu_plugin_stream_example](https://github.com/nushell/nushell/tree/main/crates/nu_plugin_stream_example).

I'm looking to get a feel for how [nu
plugins](https://www.nushell.sh/contributor-book/plugins.html#plugins) work,
and in particular, what's possible with [@devyn](https://github.com/devyn)'s
new [streaming plugin
PR](https://github.com/nushell/nushell/tree/main/crates/nu_plugin_stream_example)
that just landed in [nushell](https://www.nushell.sh) main.

Feedback is appreciated :green_heart:

It provides one command:

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
    update data { from json}
```
