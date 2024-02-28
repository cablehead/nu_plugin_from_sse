# [`nu`](https://www.nushell.sh) [streaming plugin](https://github.com/nushell/nushell/pull/11911): `nu_plugin_from_sse`

This plugin was forked of the [nu_plugin_stream_example](https://github.com/nushell/nushell/tree/main/crates/nu_plugin_stream_example).

describe what this plugin does, looking to get a feel from how nu plugin's
work, and in particular: what's possible with @d's new streaming plugin PR

## `from sse`

This command transforms to structured records, how do people document records in nushell?

```
event: creatureAlert
id: 1
data: {"id":"uni123","type":"Unicorn","lat":45.4235,"lon":-75.6979,"urgency":"high","desc":"Injured near Crystal River."}

```

```nushell
bp | from sse
````

| id| name | data |
| --- | --- | --- |
|1|creatureAlert|{"id":"uni123","type":"Unicorn","lat":45.4235,"lon":-75.6979,"urgency":"high","desc":"Injured near Crystal River."} |
