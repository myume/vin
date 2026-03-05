# vin

virtual input scripting language

For the grammar of the language see the [Grammar](/vin-parser/grammar.md) file.

## Example

The following example should enter `echo HELLO WORLD`

```vin
SEND echo
PRESS Space
HOLD LeftShift
SEND hello world
PRESS ENTER
```

See the [examples](examples) folder for more examples.

## Permissions

The interpreter needs access to the `/dev/uinput` file. You can acheive this in
multiple ways. The easiest is to run the interpreter with `sudo`. Alternatively,
you can create a new group and give permissions like that.

## Compatability

Currently, vin only works on linux since it depends on `/dev/uinput`.
