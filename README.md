# Rusty Esolang Interpreter

An interpreter written in rust to support a bunch of esolangs

## Symbol Meaning

|Symbol|Meaning|
|:-:|:-:|
|:x:|Not Implemented, Not Planned|
|:construction:|Work in Progress|
|:white_check_mark:|Completed|
|:negative_squared_cross_mark:|Outdated|

## Language Status

|Language|Interpreter|
|:-:|:-:|
|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|:white_check_mark:|
|[AmBored--](https://esolangs.org/wiki/AmBored--)|:white_check_mark:|

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

|Language|Status|Description|
|:-:|:-:|:-:|
|[Brainfuck](https://esolangs.org/wiki/Brainfuck)|:construction:|Optimize instruction_vector before interpretation|
|[AmBored--](https://esolangs.org/wiki/AmBored--)|:construction:|Optimize instruction_vector before interpretation|
