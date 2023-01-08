# Rusty Esopreter

An interpreter written in rust to support a bunch of esolangs

## Code Conventions

- Variable names should not be abbreviated and be as descriptive as realistically possible;
- snake_case should be used;
- Commands should be ended with a ";" even if not necessary
- leave space for readability
- comment everything

## Exit codes

|Exit Code|Language|Message|Subcategory|
|:-:|:-:|:-:|:-:|
|0|ALL|Success||
|1|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|RANGE ERROR||
|2|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|MISMATCHED LOOP|Loop end before loop start|
|3|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|MISMATCHED LOOP|Unclosed loop starts|
|-2|ALL|ERROR READING FILE|
|-1|ALL|NOT IMPLEMENTED ERROR|

## Language Status

|Language|Status|
|:-:|:-:|
|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|WIP|