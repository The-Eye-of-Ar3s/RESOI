# Rusty Esolang Interpreter

An interpreter written in rust to support a bunch of esolangs

## Code Conventions

- Variable names should not be abbreviated and be as descriptive as realistically possible;
- snake_case should be used;
- Commands should be ended with a ";" even if not necessary
- leave space for readability
- comment everything

## Language Status

|Language|Interpreter|
|:-:|:-:|
|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|:white_check_mark:|
|[AmBored--](https://esolangs.org/wiki/AmBored--)|:construction:|

## Exit codes

|Exit Code|Language|Message|Subcategory|
|:-:|:-:|:-:|:-:|
|0|ALL|Success||
|1|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|RANGE ERROR||
|2|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|MISMATCHED LOOP|Loop end before loop start|
|3|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|MISMATCHED LOOP|Unclosed loop starts|
|4|SDL2|ERROR INITIALIZING SDL2|ERROR CREATING CONTEXT|
|5|SDL2|ERROR INITIALIZING SDL2|ERROR CREATING VIDEO SUBSYSTEM|
|6|SDL2|ERROR INITIALIZING SDL2|ERROR CREATING WINDOW|
|7|SDL2|ERROR INITIALIZING SDL2|ERROR CREATING CANVAS|
|8|SDL2|ERROR INITIALIZING SDL2|ERROR CREATING EVENT PUMP|
|9|SDL2|SDL2 RUNTIME ERROR|ERROR DRAWING PIXEL|
|10|[AmBored--](https://esolangs.org/wiki/AmBored--)|MISMATCHED LOOP|Loop end before loop start|
|11|[AmBored--](https://esolangs.org/wiki/AmBored--)|MISMATCHED LOOP|Unclosed loop starts|
|-1|ALL|NOT IMPLEMENTED ERROR|

## Todo

### Brainfuck

- Optimize instruction_vector before interpretation

### AmBored--

- Optimize instruction_vector before interpretation
