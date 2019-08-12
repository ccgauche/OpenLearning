# OpenLearning
## Introduction
Open learning is a string pattern matcher than can learn to find regularities into strings.
To use this you will need a dataset of "normal strings" like:
### *normal.txt*
```
Apple Tree
Banana Split
Pen Pineapple Apple Pen
Cury Rice
French fries
```
And a dataset that contains the thing you want to search:
### *dataset.txt*
```
Apple pie
Banana cake
Rice cake
Pineapple pie
Strawberries tart
```
And you will get a train dataset:
```shell
$ open_learning learn
```
### *model.txt*
```
["apple pie", " cake"]
```
And now we can test with an example:
```shell
$ open_learning test "cheese cake"
   Reading Done in 0ms
   Correspond true
```
As you can see it correspond with the model.
```shell
$ open_learning test "Apple juice"
   Reading Done in 0ms
   Correspond false
```
It work's fine.

## Command usage
```shell
$ open_learning learn [-d DATASET-FILE] [-n NORMAL-FILE] [-o MODEL-OUTPUT-FILE]

$ open_learning test [-i INPUT-FILE] <INPUT-STRING>
```

### Examples
```shell
$ open_learning learn -d "dataset.txt" -n "normal.txt" -o "model.txt"

$ open_learning test -i "model.txt" "Apple pen"
```

## Integration into programs

[OpenLearningAPI](https://docs.rs/open_learning_api/) - A rust library to use the trained model

## Why rust ?

I have chosen rust for four reasons: performance, safety, ecosystem, concurrency

## Download executables

Linux executable: [0.1.0beta](https://rs.mcalts.fr/OpenLearning/0.1.0/open_learning)

## Compile from source

Download the OpenLearning folder and run into it:
```shell
$ cargo build --release
```

Rust is needed: [Download rust](https://www.rust-lang.org/tools/install)

## Futures features (in progress)

Console colorized Learner [DONE]
Linear String matching [DONE]
Positional String matching [TO DO]
Patern string matching [TO DO]
Semantic matching [TO DO]

Error Percentage Calculator [TO DO]
Valid Percentage Calculator [TO DO]
More validity and more error option [TO DO]

More examples [TO DO]

Rust Font-End [DONE]
JS Font-End [TO DO]
Python Font-End [TO DO]
Java Font-End [TO DO]

Specify specification [TO DO]


