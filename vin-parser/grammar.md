# Grammar

Pretty simple for now. Just a sequence of keyboard inputs

```
PROGRAM ::= STATEMENT*

STATEMENT ::= (KEYBOARD_INPUT) NEWLINE

KEYBOARD_INPUT ::= KEYBOARD_COMMAND

KEYBOARD_COMMAND ::= KEYPRESS_COMMAND

KEYPRESS_COMMAND ::= "PRESS" KEY

KEY ::= \<keyboard key\>
```
