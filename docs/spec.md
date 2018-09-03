Specification
================================================================================

Source Code Representation
--------------------------------------------------------------------------------

Source code is defined as Unicode UTF-8 input. This must be broken into functional units of text known as _tokens_. These are largely analogous with the words in a text-document, but there are more specific rules involved for runs of text such as Strings and Comments.

It should be noted early on that Retronym is case-sensitive and white-space insensitive, the details of which will be covered a little later.

The term _white-space_ refers to ASCII space, horizontal-tab, carriage-return and/or line-feed (collectively know as 'new-line' or 'end-of-line').

A _word-boundary_ refers to the breaking-point between words and/or padding _white-space_, this includes the beginning / end-of-input and some specific character sequences detailed below.

There are five tokenisation rules:

* **Standard Rule** :  _Tokens_ begin and end between _word-boundaries_. I.e. _white-space_ and the special "punctuation" named below

* **Punctuation Rule** : The _token_ is a specialised _word-boundary_. That is, the _token_ acts as a _word-boundary_ between _tokens_ and is also a _token_ itself. See [Operators](#operators) for the list

* **End-of-Line Rule** : The _token_ begins after a _word-boundary_ and continues to the next end-of-line/input, including any _white-space_ along the way. E.g. _Comments_

* **Wrapping Rule** : The _token_ begins and ends at specified markers, including any _white-space_ along the way, but cannot not exceed the end-of-line/input

* **Block Rule** : The _token_ begins and ends at specified markers and includes all white-space in-between, even new-lines

No _token_ can exist within another _token_.

These rules apply to the different lexical elements defined below:

### Atoms ###

An _Atom_ is the simplest lexical element, it consists of a single word.
_Atoms_ are the "keywords" of the language. They do not represent any value, but they can be compared with each other for equality (see Macros). _Atoms_ are used for all the CPU primitives: _Instructions_ (`adc`, `jmp` etc.) and _Registers_ (`A`, `HL` etc.).

* An _Atom_ cannot begin with a number
* An _Atom_ consists of the letters "a"-"z", "A"-"Z" & "0"-"9" (excepting the first character, as above)
* An _Atom_ cannot contain an underscore

The following regular-expression describes an _Atom_:

```regex
[a-zA-Z][a-zA-Z0-9]*
```

### String Literals ###

Strings represent a contiguous series of UTF-8 byte values. They use the _wrapping tokenisation rule_ and are delimitated by speech marks.

    "This is a string!"

The following regular expression describes a string literal.

```regex
"[^"]+"
```

Note that an empty string (`""`) is invalid.

There are **no** string escapement sequences. For example, the "`\n`", commonly used to represent a new-line in many programming languages.

This is because: _1._ string escapement sequences have always been a bad idea, introducing bugs and reducing readability, especially in nested-contexts, and _2._ not all retro systems use ASCII or even have the concept of a new-line character code (such as with consoles).

Because strings are just bytes you can freely intermix them with numerical data, allowing you to compose non-printable character codes, such as the new-line character and to include speech-marks:

    "the ghost said ", 34, " boo!", 34, 10

See the [section on text mapping](#text_mapping) for remapping string sequences to alternative values.

### Comments ###

Comments serve as documentation for readers of your source code. They follow the _end-of-line tokenisation rule_. Implementations may discard them once tokenised.

Comments begin with a semi-colon character.

    ;this is a comment

### Documentation ###

A _documentation line-comment_ begins with two back-tick characters and follows the _end-of-line tokenisation rule_. Unlike a _comment_, _documentation line-comments_ are intended to be preserved for automatically generating external documentation.

    ``this is a documentation line-comment

A _documentation block-comment_ follows the _block tokenisation rule_ and begins with three back-tick characters, continuing until the next three back-tick characters -- including new-lines.

    ```
    this is a documentation block-comment
    ```

This is intended for longer, multi-line, free-form external documentation.

### Assembly Mnemonics ###

Assembly language mnemonics follow the _standard tokenisation rule_. This section does not detail the allowed parameters for the numerous assembly instructions, just the keywords reserved for tokenisation.

**NOTE:** The "CPU(s)" column refers to the following:

* _"Z80"_ - Zilog [Z80] _only_ (not on [Nintendo Game Boy][gb80])
* _"Z80+"_ - Zilog [Z80], including Sharp LR35902 ([Nintendo Game Boy][gb80])
* _"Z80?"_ - Undocumented [Z80] Instruction. Some Z80s only
* _"GB80"_ - Sharp LR35902 ([Nintendo Game Boy][gb80]) _only_
* _"6502"_ - MOS Technologies / Commodore, 6502 / 6507 / 6510

[z80]: https://en.wikipedia.org/wiki/Zilog_Z80
[gb80]: https://en.wikipedia.org/wiki/Game_Boy

| keyword   | CPU(s)            | Description
|-----------|-------------------|----------------------------------------------|
| `adc`     | Z80+, 6502        | Add to Accumulator with Carry
| `add`     | Z80+              | Add to Accumulator
| `and`     | Z80+, 6502        | Logical AND with Accumulator
| `bit`     | Z80+, 6502        | Test Bit
| `call`    | Z80+              | Call Routine
| `ccf`     | Z80+              | Complement (invert) Carry Flag
| `cp`      | Z80+              | Compare with Accumulator
| `cpd`     | Z80+              | Compare and Decrement
| `cpdr`    | Z80+              | Compare, Decrement and Repeat
| `cpi`     | Z80+              | Compare and Increment
| `cpir`    | Z80+              | Compare, Increment and Repeat
| `cpl`     | Z80+              | Complement (invert) Accumulator
| `daa`     | Z80+              | Decimal Adjust Accumulator
| `dec`     | Z80+              | Decrement
| `di`      | Z80+              | Disable Interrupts
| `djnz`    | Z80               | Decrement, Jump if Not Zero
| `ei`      | Z80+              | Enable Interrupts
| `ex`      | Z80               | Exchange DE and HL Registers
| `exx`     | Z80               | Exchange AF / DE / HL with AF' / DE' / HL'
| `halt`    | Z80+              | Halt (Wait for Interrupt)
| `im`      | Z80+              | Interrupt Mode
| `in`      | Z80               | Input from Port
| `inc`     | Z80+              | Increment
| `ind`     | Z80               | Input from Port and Decrement
| `indr`    | Z80               | Input from Port, Decrement and Repeat
| `ini`     | Z80               | Input from Port and Increment
| `inir`    | Z80               | Input from Port, Increment and Repeat
| `jp`      | Z80+              | Jump (absolute)
| `jr`      | Z80+              | Jump Relative
| `ld`      | Z80+              | Load
| `ldd`     | Z80+              | Load and Decrement
| `lddr`    | Z80+              | Load, Decrement and Repeat
| `ldi`     | Z80+              | Load and Increment
| `ldir`    | Z80+              | Load, Increment and Repeat
| `neg`     | Z80+              | Negate Accumulator
| `nop`     | Z80+              | No Operation
| `or`      | Z80+              | Logical OR with Accumulator
| `otdr`    | Z80               | Output to Port, Decrement and Repeat
| `otir`    | Z80               | Output to Port, Increment and Repeat
| `out`     | Z80               | Output to Port
| `outd`    | Z80               | Output to Port and Decrement
| `outi`    | Z80               | Output to Port and Increment
| `pop`     | Z80+              | Pop from Stack
| `push`    | Z80+              | Push to Stack
| `res`     | Z80+              | Reset Bit
| `ret`     | Z80+              | Return from Routine
| `reti`    | Z80+              | Return from Interrupt
| `retn`    | Z80+              | Return from Non-Maskable Interrupt
| `rl`      | Z80+              | Rotate Operand Left
| `rla`     | Z80+              | Rotate Accumulator Left
| `rlc`     | Z80+              | Rotate Operand Left with Carry
| `rlca`    | Z80+              | Rotate Accumulator Left with Carry
| `rld`     | Z80+              | Rotate Left Decimal
| `rr`      | Z80+              | Rotate Operand Right
| `rra`     | Z80+              | Rotate Accumulator Right
| `rrc`     | Z80+              | Rotate Operand Right with Carry
| `rrca`    | Z80+              | Rotate Accumulator Right with Carry
| `rrd`     | Z80+              | Rotate Right Decimal
| `rst`     | Z80+              | Reset (call page zero)
| `sbc`     | Z80+              | Subtract from Accumulator with Carry
| `scf`     | Z80+              | Set Carry Flag
| `set`     | Z80+              | Set Bit
| `sla`     | Z80+              | Shift Accumulator Left
| `sll`     | Z80?              | Shift Operand Left (undocumented)
| `sra`     | Z80+              | Shift Accumulator Right
| `srl`     | Z80+              | Shift Operand Right
| `stop`    | GB80              | Stop CPU
| `sub`     | Z80+              | Subtract from Accumulator
| `xor`     | Z80+              | Logical Exclusive-OR with Accumulator

### Registers ###

CPU registers for the target machine follow the _standard tokenisation rule_.
The following keywords are reserved as registers:

**NOTE:** The "CPU(s)" column refers to the following:

* _"Z80"_ - Zilog [Z80] _only_ (not on [Nintendo Game Boy][gb80])
* _"Z80+"_ - Zilog [Z80], including Sharp LR35902 ([Nintendo Game Boy][gb80])
* _"Z80?"_ - Undocumented [Z80] Instruction. Some Z80s only

[z80]: https://en.wikipedia.org/wiki/Zilog_Z80
[gb80]: https://en.wikipedia.org/wiki/Game_Boy

| Keyword   | CPU(s)            | Description
|-----------|-------------------|----------------------------------------------|
| `A`       | Z80+, 6502        | Accumulator
| `B`       | Z80+              | B Register
| `C`       | Z80+              | C Register
| `D`       | Z80+              | D Register
| `E`       | Z80+              | E Register
| `H`       | Z80+              | H Register
| `L`       | Z80+              | L Register
| `I`       | Z80+              | Interrupt Register (unrelated to `IX` / `IY`)
| `R`       | Z80+              | Refresh Register
| `AF`      | Z80+              | Register Pair AF
| `BC`      | Z80+              | Register Pair BC
| `DE`      | Z80+              | Register Pair DE
| `HL`      | Z80+              | Register Pair HL
| `IX`      | Z80               | Index-X Register (16-bit)
| `IY`      | Z80               | Index-Y Register (16-bit)
| `IXH`     | Z80?              | Index-X Register (upper half)
| `IXL`     | Z80?              | Index-X Register (lower half)
| `IYH`     | Z80?              | Index-Y Register (upper half)
| `IYL`     | Z80?              | Index-Y Register (lower half)
| `SP`      | Z80+              | Stack Pointer
| `X`       | 6502              | Index-X Register (8-bit)
| `Y`       | 6502              | Index-Y Register (8-bit)

All _registers_ support _hints_, a form of documentation to describe what a _register_ is being used for. A _register hint_ is appended to a _register_ keyword with a single back-tick character and a user-chosen arbitrary descriptor consisting of any combination of letters A-Z (uppercase), a-z (lowercase), "`_`" (underscore) and numerals 0-9 with the one exception that it may not begin with a numeral.

    HL`width        ;documents that register HL currently holds a width value
    HL `width       ;ERROR: separation between register and hint disallowed

The _hint_ is purely descriptive and does not affect assembly. A different _hint_ can be used for every _register_ use to document the changing role of the _register_ in your assembly code.

_Shadow registers_ are secondary _registers_ in a CPU not normally accessible by their own name, instead an assembly instruction swaps the contents of an existing _register_ with its shadow counterpart. It is useful to keep track of this swapping happening and a different form of _register hint_ is provided to do this.

    AF'flags        ;the shadow version of AF is in play
    AF'             ;the hint descriptor is optional for shadow hints

    AF'`flags       ;this is not valid
    AD`'flags       ;neither is this

    ex AF  AF'      ;Z80 code to swap to AF shadow register
    ex AF' AF       ;Z80 code to swap back to original contents

The use of _shadow hints_ are valid only on _registers_ that support them, namely:

| Keyword   | CPU(s)            | Description
|-----------|-------------------|----------------------------------------------|
| `A'`      | Z80               | Accumulator
| `B'`      | Z80               | B Register
| `C'`      | Z80               | C Register
| `D'`      | Z80               | D Register
| `E'`      | Z80               | E Register
| `H'`      | Z80               | H Register
| `L'`      | Z80               | L Register
| `AF'`     | Z80               | Register Pair AF
| `BC'`     | Z80               | Register Pair BC
| `DE'`     | Z80               | Register Pair DE
| `HL'`     | Z80               | Register Pair HL

\*_Shadow registers_ are not available on the Nintendo Game Boy.

The following regular expressions describe valid _registers_ and _shadow registers_ including optional _hint_:

_Non-shadow Registers:_

```regex
(AF?|BC?|C|DE?|E|HL?|L|I([XY][HL]?)?|R|SP)(`[A-Za-z_][A-Za-z_0-9]*)?
```

_Shadow Registers:_

```regex
(AF?|BC?|C|DE?|E|HL?|L)'([A-Za-z_][A-Za-z_0-9]*)?
```

### Condition Codes ###

Some assembly instructions take a _condition code_ as a parameter that is compared with the CPU status flags. _Conditions codes_ follow the _standard tokenisation rule_ and are always lower-case (to avoid confusion with _registers_).

The following keywords are reserved as _condition codes_:

**NOTE:** The "CPU(s)" column refers to the following:

* _"Z80"_ - Zilog [Z80], including Sharp LR35902 ([Nintendo Game Boy][gb80])
* _"Z80*"_ - Zilog [Z80] _only_ (not on [Nintendo Game Boy][gb80])

[z80]: https://en.wikipedia.org/wiki/Zilog_Z80
[gb80]: https://en.wikipedia.org/wiki/Game_Boy

| Keyword   | CPU(s)            | Description
|-----------|-------------------|----------------------------------------------|
| `c`       | Z80               | Carry
| `nc`      | Z80               | No Carry
| `z`       | Z80               | Zero
| `nz`      | Z80               | Not Zero
| `p`       | Z80*              | Sign Positive
| `m`       | Z80*              | Sign Negative
| `pe`      | Z80*              | Parity Even
| `po`      | Z80*              | Parity Odd

_Condition codes_ may also use _hints_:

    ret nc`gun_loaded   ;return early if gun is already loaded

The following regular expression describes a valid _condition code_ with optional _hint_:

```regex
p[eo]?|m|n?[cz](`[A-Za-z_][A-Za-z_0-9]*)?
```

### Operators ###

The following operators use the _punctuation tokenisation rule_ and therefore can be separated from other _tokens_ with or without _white-space_.

    +       Add
    -       Subtract / Negate
    *       Multiply
    /       Divide
    ^       Power
    (       Begin list
    ,       List-item separator (optional)
    )       End list

The _repetition operator_ is a lower-case "x" and follows the _standard tokenisation rule_, requiring white-space either side.

    x       Repetition operator

### Integer Literals ###

Decimal integers follow the _standard tokenisation rule_ and are be written in the common format shared by most programming languages these days. The positive-sign (`+`) is not allowed.

    100         ;positive decimal integers
    -10         ;negative decimal integers

**NOTE:**
    The subtract / negate operator ("`-`") follows the _punctuation tokenisation rule_ and therefore does not require white-space for separation. This implies that an integer literal such as "`-10`" should be taken as two _tokens_, consisting of a _negate operator_ followed by a _**positive** integer_ of 10. Expression parsing will apply one to the other to produce the desired negative integer.

### Floating-point Literals ###

Floating point numbers follow the IEEE-754 standard notation:

    1.23        ;positive (and negative) decimal floats
    1.23e45     ;floating point scientific notation
    0.12        ;leading zero required for subnormal numbers

### Hexadecimal Literals ###

Hexadecimal literals follow the _standard tokenisation rule_ and use the `$` sigil, _not_ `0x` as is common today. This is because `$` was the standard hexadecimal sigil in the 8-bit era and most such assembly code uses that format.

Following the sigil, 1 to 8 hexadecimal digits can follow; e.g. `$AB, $AB12, $ABCD1234`. Hexadecimal digits may be upper-case or lower-case.

The following regular expression describes a valid _hexadecimal literal_:

```regex
\$[0-9A-Fa-f]{1,8}
```

### Binary Literals ###

Binary literals follow the _standard tokenisation rule_ and use a `%` sigil. Following the sigil, 1 to 32 binary digits follow.

    %1010101011111111

The following regular expression describes a valid binary literal_:

```regex
%[01]{1,32}
```

### Symbols ###

_Symbols_ are user-defined names for referencing data. All symbols begin with a sigil (a type-mark) followed by an _identifier_, the user-defined name.

#### Identifiers ####

All _identifiers_, regardless of sigil, are limited to one or more characters A-Z (upper and lower-case), underscore and numerals 0-9 with the exception that it may not begin with a numeral.

A _composite identifier_ is the combination of multiple _identifiers_ separated by a period, for example: `label.subroutine`. These are used to reference symbol hierarchies.

The following regular expression describes a valid _composite identifier_ (sans sigil):

```regex
[A-Za-z_][A-Za-z_0-9]*(\.[A-Za-z_][A-Za-z_0-9]*)*
```

#### Labels ####

_Labels_ use a colon sigil followed by a _composite identifier_. \
See the [section on procedures](#procedures) for details on the purpose of _labels_.

#### User-defined Types ####

_User-defined types_ use a percent-sign sigil followed by a _composite identifier_. \
See the [section on types](#types) for details on the purpose of _user-defined types_.

#### Constants ####

_Constants_ use an exclamation mark sigil followed by a _composite identifier_.

### Keywords ###

Finally, other keywords are reserved:

The built-in types are `bool`, `nybl`, `byte`, `word` & `long` and use the _standard tokenisation rule_.

---

Any _token_ that does not match any of the above _lexical elements_ is a syntax error and should stop further processing.

<!--

Types
--------------------------------------------------------------------------------

Retronym assembles machine code and data for 8-bit systems. It is up to the assembly program how to interpret any particular byte or sequence of bytes. Retronym provides a set of basic data-types for the purpose of ensuring that your data is packed and stored in the manner your assembly code expects it.

### Built-in Types ###

The built-in types are `bool`, `nybl`, `byte`, `word` & `long`.

The `bool` type represents a single bit of memory. These can be packed together and each bool will always occupy the next available bit in memory.

The `nybl` type represents four bits of memory (a "nybble"), always aligned to a nybble-boundary (that is, half-bytes). For example the _Record Type_ "`bool, bool, nybl`" would actually occupy memory as 2 bits, followed by 2 unused bits and then the nybble. A nybble will always be aligned to the nearest half-byte boundary and unused bits will be inserted as necessary.

If you want to use a nybble without the alignment, use 4 combined bits instead, e.g. "`bool, bool x 4`"

The `byte` type represents a single byte (8-bits) of memory, aligned to the nearest byte boundary.

The `word` type represents two bytes of memory in little-endian order, aligned to the nearest _byte_ boundary. Likewise, the `long` type represents four bytes of memory (in little-endian order), aligned to the nearest _byte_ boundary.

### User-Defined Types ###

Known as Structures or Structs in many other languages, _types_ in Retronym are the packing of one or more _built-in types_ and/or other _user-defined types_ into a single unit of memory.

In this example we define a new _user-defined type_ `%myType` consisting of 7 bytes of memory composed from one `byte`, one two-byte `word` and one four-byte `long`.

```retronym
%myType     byte, word, long
```

The memory size and layout of _types_ are only required to be exact in the final binary output; during parsing, assembling and linking, _types_ may be represented in memory in whichever way is convenient to the implementation.



-->


Procedures
--------------------------------------------------------------------------------

Section pending.

Text Mapping
--------------------------------------------------------------------------------

Section pending.


Appendix A: Atoms
================================================================================

* _"Z80"_ - Zilog [Z80] _only_ (not on [Nintendo Game Boy][gb80])
* _"Z80+"_ - Zilog [Z80], including Sharp LR35902 ([Nintendo Game Boy][gb80])
* _"Z80?"_ - Undocumented [Z80] instruction. Some Z80s only
* _"GB80"_ - Sharp LR35902 ([Nintendo Game Boy][gb80]) _only_
* _"6502"_ - MOS Technologies / Commodore, 6502 / 6507 / 6510
* _"6502?"_ - Undocumented 6502 instruction

[z80]: https://en.wikipedia.org/wiki/Zilog_Z80
[gb80]: https://en.wikipedia.org/wiki/Game_Boy

| keyword   |      |            | Description
|-----------|------|------------|----------------------------------------------|
| `adc`     | Z80+ | 6502       | Add to Accumulator with Carry
| `add`     | Z80+ |            | Add to Accumulator
| `and`     | Z80+ | 6502       | Logical AND with Accumulator
| `bit`     | Z80+ | 6502       | Test Bit
| `call`    | Z80+ |            | Call Routine
| `ccf`     | Z80+ |            | Complement (invert) Carry Flag
| `cp`      | Z80+ |            | Compare with Accumulator
| `cpd`     | Z80+ |            | Compare and Decrement
| `cpdr`    | Z80+ |            | Compare, Decrement and Repeat
| `cpi`     | Z80+ |            | Compare and Increment
| `cpir`    | Z80+ |            | Compare, Increment and Repeat
| `cpl`     | Z80+ |            | Complement (invert) Accumulator
| `daa`     | Z80+ |            | Decimal Adjust Accumulator
| `dec`     | Z80+ |            | Decrement
| `di`      | Z80+ |            | Disable Interrupts
| `djnz`    | Z80  |            | Decrement, Jump if Not Zero
| `ei`      | Z80+ |            | Enable Interrupts
| `ex`      | Z80  |            | Exchange DE and HL Registers
| `exx`     | Z80  |            | Exchange AF / DE / HL with AF' / DE' / HL'
| `halt`    | Z80+ |            | Halt (Wait for Interrupt)
| `im`      | Z80+ |            | Interrupt Mode
| `in`      | Z80  |            | Input from Port
| `inc`     | Z80+ |            | Increment
| `ind`     | Z80  |            | Input from Port and Decrement
| `indr`    | Z80  |            | Input from Port, Decrement and Repeat
| `ini`     | Z80  |            | Input from Port and Increment
| `inir`    | Z80  |            | Input from Port, Increment and Repeat
| `jp`      | Z80+ |            | Jump (absolute)
| `jr`      | Z80+ |            | Jump Relative
| `ld`      | Z80+ |            | Load
| `ldd`     | Z80+ |            | Load and Decrement
| `lddr`    | Z80+ |            | Load, Decrement and Repeat
| `ldi`     | Z80+ |            | Load and Increment
| `ldir`    | Z80+ |            | Load, Increment and Repeat
| `neg`     | Z80+ |            | Negate Accumulator
| `nop`     | Z80+ |            | No Operation
| `or`      | Z80+ |            | Logical OR with Accumulator
| `otdr`    | Z80  |            | Output to Port, Decrement and Repeat
| `otir`    | Z80  |            | Output to Port, Increment and Repeat
| `out`     | Z80  |            | Output to Port
| `outd`    | Z80  |            | Output to Port and Decrement
| `outi`    | Z80  |            | Output to Port and Increment
| `pop`     | Z80+ |            | Pop from Stack
| `push`    | Z80+ |            | Push to Stack
| `res`     | Z80+ |            | Reset Bit
| `ret`     | Z80+ |            | Return from Routine
| `reti`    | Z80+ |            | Return from Interrupt
| `retn`    | Z80+ |            | Return from Non-Maskable Interrupt
| `rl`      | Z80+ |            | Rotate Operand Left
| `rla`     | Z80+ |            | Rotate Accumulator Left
| `rlc`     | Z80+ |            | Rotate Operand Left with Carry
| `rlca`    | Z80+ |            | Rotate Accumulator Left with Carry
| `rld`     | Z80+ |            | Rotate Left Decimal
| `rr`      | Z80+ |            | Rotate Operand Right
| `rra`     | Z80+ |            | Rotate Accumulator Right
| `rrc`     | Z80+ |            | Rotate Operand Right with Carry
| `rrca`    | Z80+ |            | Rotate Accumulator Right with Carry
| `rrd`     | Z80+ |            | Rotate Right Decimal
| `rst`     | Z80+ |            | Reset (call page zero)
| `sbc`     | Z80+ |            | Subtract from Accumulator with Carry
| `scf`     | Z80+ |            | Set Carry Flag
| `set`     | Z80+ |            | Set Bit
| `sla`     | Z80+ |            | Shift Accumulator Left
| `sll`     | Z80? |            | Shift Operand Left (undocumented)
| `sra`     | Z80+ |            | Shift Accumulator Right
| `srl`     | Z80+ |            | Shift Operand Right
| `stop`    | GB80 |            | Stop CPU
| `sub`     | Z80+ |            | Subtract from Accumulator
| `xor`     | Z80+ |            | Logical Exclusive-OR with Accumulator