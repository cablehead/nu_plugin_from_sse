# Streaming Plugin Example: `nu_plugin_from_sse`

This plugin was forked off: link to stream crate plugin example nushell

describe what this plugin does, looking to get a feel from how nu plugin's
work, and in particular: what's possible with @d's new streaming plugin PR

## `from sse`

This command transforms to structured records, how do people document records in nushell?

```
event: creatureAlert
id: 1
data: {"id":"uni123","type":"Unicorn","lat":45.4235,"lon":-75.6979,"urgency":"high","desc":"Injured near Crystal River."}

```

> ```nushell
> bp | from sse
> ````

| Syntax | Description |
| --- | ----------- |
| Header | Title |
| Paragraph | Text |

| id| name | data |
| --- | --- | --- |
|1|creatureAlert|{"id":"uni123","type":"Unicorn","lat":45.4235,"lon":-75.6979,"urgency":"high","desc":"Injured near Crystal River."} |

