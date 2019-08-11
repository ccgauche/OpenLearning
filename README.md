# OpenLearning
## Introduction
Open learning is a string pattern matcher than can learn to find regularities into strings.
To use this you will need a database to "normal strings" like:
### normal.txt
```
Apple Tree
Banana Split
Pen Pineapple Apple Pen
Cury Rice
French fries
```
And a noise dataset that contains the thing you want to search:
### dataset.txt
```
Apple pie
Banana cake
Rice cake
Pineapple pie
Strawberries tart
```
And you will get a train dataset:
```
open_learning
```
#### model.txt
```
["apple pie", " cake"]
```
And now we can test with an example:
```
$ open_learning "cheese cake"
Intialized in 0.031245ms
Search done in 0.022149ms
Correspond True
```
As you can see it correspond with the model.
```
$ open_learning "Apple juice"
Intialized in 0.031245ms
Search done in 0.022149ms
Correspond False
```
It work's fine.

## Integration into programs

[OpenLearningAPI](https://docs.rs/open_learning_api/) - A rust library to use the trained model

## Why rust ?

I have chosen rust for four reasons: performance, safety, ecosystem, concurrency
