A Very Gentle Introduction to Retronym
================================================================================

What is Retronym
--------------------------------------------------------------------------------

So you want to write games and programs for 8-bit retro computer systems and consoles? Retronym is the development system for you!

Retronym is an assembler -- a program that takes written CPU instruction code (assembly) and _assembles_ it into a working binary for the target retro system.

Assemblers have existed since the 1950s, they were invented so that programmers no longer had to flip switches for each individual bit in memory.

Almost all console games written during the 1980s were written in assembly. The "C" programming language only became common in the 90s when 16/32-bit hardware had taken over from the 8-bit microcomputer generation.

Basic Types
--------------------------------------------------------------------------------

In assembly, all code and data are ultimately just bytes. At the CPU-level, there's no real difference between numbers, text or instructions -- you can read assembled code as numbers -- some early games actually used their own code as a pseudo-random number stream!

Because Retronym is an assembler and you will be the one writing the assembly code, it is critical that the size of program data matches the instructions intended to read it -- if you write an instruction to read two bytes, but the data is only one byte, your program will quickly go haywire.

_Types_ are used to describe the storage requirements of data. A numerical value such as "42" doesn't tell us if it should occupy one, two or four bytes of memory. Whilst numbers below 256 will fit into one byte, the number alone doesn't tell us if it's expected to increase to a much bigger number (such as a score).

There are five basic built-in _types_ in Retronym that describe data sizes for retro systems:

* A `bool` occupies one bit of memory
* A `nybl` ("nybble") occupies four bits of memory (half a byte)
* A `byte` occupies 8 bits of memory
* A `word` occupies 16 bits (2 bytes) of memory
* A `long` occupies 32 bits (4 bytes) of memory

Other than some early mainframe computers, all systems -- retro and modern -- load and store data in a minimum of 8-bits at a time, which introduces some special rules regarding the use of the `bool` & `nybl` types that we will get into a little later.

Records
--------------------------------------------------------------------------------

Data is often stored in structured, table-like forms. You might have, for example, a table of data that describes the enemies present in a level. The 'columns' of this table would define the properties of each enemy such as X-Position, Y-Position, Kind, Weapon and Health etc. The 'rows' would define each specific enemy, providing a value for each of those columns.

The example below demonstrates a simple data-table:

```retronym
:someData
        byte, word, long
        1, 2, 3
        10, 20, 30
```

The first line is a _label_. It is a named value (a.k.a "symbol") in the final binary, much like an exported function in other programming languages, or a data-table (as in this example).

The second line defines a _record type_. This describes the data-size of each 'column' in a data-table; e.g. here we have three columns.

The third and fourth lines are data; now that the _record type_ has been defined, any data that follows will be assembled according to the _record type_.

In our example, the data `1, 2, 3` would be assembled, not into 3 bytes (`$01, $02, $03`) but instead, into 7 bytes: `$01` (a `byte`), `$02 $00` (a `word`) and `$03 $00 $00 $00` (a `long`); note that these bytes are in Little-Endian order

> **PRO TIP:** "Little Endian" means that in a multi-byte number the byte(s) that contain the lower-power portion of the number come first, e.g. for the hexadecimal number `$FF00` (=65'280 decimal) the bytes are in the order: `$00, $FF`

The number of bytes that a _record type_ occupies -- the 'width' of a _record_ -- is called the _stride_.

When a _record_'s _stride_ has been filled, it is "satisfied". You cannot partially fill a _record_ and leave it "unsatisfied"!

```retronym
:someData
        byte, word, long
        1, 2, 3
        10, 20                  ;<-- ERROR: unsatisfied record!
```

User-Defined Types
--------------------------------------------------------------------------------

A _user-defined type_ allows us to give a name to a commonly used pattern of data-sizes. Instead of having to write "`byte, word, long`" on every data-table where this applies (not least that this is terrible if you need to change this later), we can define a name to mean what we say:

```retronym
%thing          byte, word, long

:someData
        %thing
        1, 2, 3
        10, 20, 30
```

A _user-defined type_ begins with a percent sigil followed by an _identifier_: a name containing `A-Z`, `a-z`, `0-9` and `_`, though cannot begin with a numeral.

Retronym distinguishes between the defining of symbols and their recall based on the indent; in the first line the _user-defined type_ appears without indent, the third line also defines a _label_ ("`:someData`") and the fourth line, which sets up the _record-type_, **uses** the new _user-defined type_ because it begins indented ('under') the _label_.

The following would be incorrect:

```retronym
        ;ERROR! Not a definition:
        %thing  byte, word, long

:someData
%thing              ;ERROR! Cannot define `%thing% here
        1, 2, 3
        10, 20, 30
```

The _record-type_ is not limited to a single _user-defined type_. You can combine _built-in types_ and _user-defined types_ freely:

```retronym
:someData
        byte, %thing, %thing, word
```

_User-defined types_ can contain other _user-defined types_:

```retronym
%vector         word, word
%enemy          %vector, %vector
```




<!--

Record-strides can also be mixed within a procedure / table. In real-world usage you will probably need to produce data-tables that begin with a list of pointers to index the actual data that follows, you may even have to intermix assembly code and data or even make tables of assembly code!

```retronym
%boxA       byte byte
%boxB       word

:boxes
    ;a record using the custom types:
    %boxA %boxB

    1, 2, 34

    ;two records using only the first custom type:
    %boxA

    1, 2, 3, 4

    ;two records using only the second custom type:
    %boxB

    12, 34
```

-->

Type Repetition
--------------------------------------------------------------------------------

These examples are nice and all, but quite trite as soon as you get to something meatier; what happens when your _records_ are very wide?

The _repetition operator_ (" `x` ") expands a single _type_ into a list, repeating it however many times we want. There must be whitespace either side of the "`x`" and it must be lower-case (to distinguish it from the machine register "`X`").

```retronym
;define a type with a stride of 10 bytes followed by a word
%thing      byte x 10, word

:longThing
        %thing
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12345
```

Data Repetition
--------------------------------------------------------------------------------

The _Repetition Operator_ ("`x`") also helps us when we have data that repeats itself. If our data consisted of a hundred zeroes followed by fifty of one number and then twenty-five of another number, we could write that as:

```retronym
;repeats each number the given amount of times
0 x 100, 1 x 50, 2 x 25
```

This shouldn't be confused with the mathematical multiply operator "`*`", e.g.:

```retronym
;calculate 6 * 7 and repeat the result 42 times
6 * 7 x 42
```

Data is often more complex than that and you may need to repeat a _sequence_ of numbers rather than a single value over and over. Using grouping parentheses you can specify a list of values to repeat:

```retronym
;outputs the numbers 1 to 5, repeated 10 times
(1, 2, 3, 4, 5) x 10
;note that this would output 1 to 4, then the number 5 ten times
1, 2, 3, 4, 5 x 10
```

We still have to write that 1 to 5 as individual numbers though, and what if that were 1 to 100 or more!? This is where _ranges_ come in.

Ranges
--------------------------------------------------------------------------------

A _range_ can automatically generate a list from a starting number through to an ending number (inclusive):

```retronym
1 ~ 5       ;outputs 1, 2, 3, 4, 5
5 ~ 1       ;if end is lower than start, counts backwards
```

But that's limited to numbers strictly in order. What if your data requires a larger step between numbers?

Range Mapping
--------------------------------------------------------------------------------

If we think of a _range_ as a stack which pops off each value automatically in succession, then we can provide a calculation to apply to each value using the _function operator_.

The `?` operator can be thought of as a substitute for 'the current value' coming from the range. Here we show how you can create a data list of even and odd numbers:

```retronym
1 ~ 5 ? * 2             ;outputs 2, 4, 6, 8, 10 (evens)
1 ~ 5 ? * 2 - 1         ;outputs 1, 3, 5, 7, 9  (odds)
```

Each number from the _range_ is fed into the expression to the right, and the result of the calculation is returned as the value to output instead.

The ability to take a range of numbers and do a calculation on each should not be under-estimated! It's an incredibly powerful tool for creating look-up tables for retro systems.

Doing large multiplications on most retro systems is very slow, so a lookup table can speed things up greatly. Let's build a lookup table for multiplying a byte by 32:

```retronym
:multiplyBy32table
    word
    0 ~ 255 ? * 32
```

That's it! You've just produced 256 sixteen-bit numbers (512 bytes) mapping the input byte (0-255) to its value when multiplied by 32.

<!-- Your assembly code just needs to take the input byte, multiply it by two to look up words, and add that value to the base-address of the lookup table. -->

In some kinds of expression you will need to refer to the input value more than once; the Fibonacci sequence, for example, is a series of numbers where the next number in the series comes from the previous two added together. Producing this sequence is used as an [example in most programming languages](http://rosettacode.org/wiki/Fibonacci_sequence):

```retronym
0, 1, 2 ~ 8 ? - 1 + ? - 2
```

This outputs the first ten numbers in the Fibonacci sequence: `0, 1, 1, 2, 3, 5, 8, 13, 21, 34`. The first two numbers are always `0` and `1` and then we take the numbers in the range `2` through `8` and output _'the current number minus 1, plus: the current number minus 2'_.

List Mapping
--------------------------------------------------------------------------------

Lists can also be mapped. Use grouping parentheses to explicitly state the beginning and end of the list (or you'll end up just mapping the last value on its own):

```retronym
() ?
```

<!--

Strings:
--------------------------------------------------------------------------------

Strings of text are, like everything else in assembly, just bytes. To save you having to provide your strings as a list of numerical ASCII codes Retronym provides String-literals like any other language:

```retronym
"Hello, world!"
```

As far as Retronym is concerned, this is just 13 bytes in a row and could just as validly be written as:

```retronym
72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33
```

Because strings are just bytes in Retronym you can freely intermix them with numerical data, allowing you to compose non-printable ASCII codes, such as the new-line character:

```retronym
"Hello", 12, "world!"
```

Retronym **does not** provide string escapement sequences, for example "`\n`" used to represent a new-line character in many programming languages.

This is because: _1._ string escapement sequences have always been a bad idea (leading to untold bugs and security issues) and _2._ not all retro systems use ASCII or even have the concept of a new-line character code (such as consoles).

```retronym
:announcement
    %byte

    "Hello, World!"
    ;
```

There is something a little-bit wrong here that I hope you've noticed in an "aha!" / light-bulb moment.

The stride is only one-byte wide, meaning each letter of the text becomes its own Record!

As far as the example program is concerned it makes no difference, but when we begin wanting to manipulate and share data across a large code-base we don't want to be dealing with individual letters like that.

We could just increase the stride to the right number of bytes, like this:

```retronym
:announcement
    byte~13

    "Hello, World!"
```

Variadic Types
--------------------------------------------------------------------------------

-->