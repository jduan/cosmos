[![official JetBrains project](https://jb.gg/badges/official.svg)](https://confluence.jetbrains.com/display/ALL/JetBrains+on+GitHub)
[![GitHub license](https://img.shields.io/badge/license-Apache%20License%202.0-blue.svg?style=flat)](https://www.apache.org/licenses/LICENSE-2.0)

# Introduction to Coroutines and Channels Hands-On Lab

This repository is the code corresponding to the
[Introduction to Coroutines and Channels](https://play.kotlinlang.org/hands-on/Introduction%20to%20Coroutines%20and%20Channels/01_Introduction)
Hands-On Lab. 

## Problem Statement

This is a GUI application. Given a github org name, it will do the following:
* Fetch all the repos under the org.
* For each repo, it will find all the contributors, and the number of contributions.

The goal is to be able to do this as fast as possible, without blocking the UI thread.

## Solutions

### Blocking

In this version, the HTTP calls to github are made from the main UI thread, and they are made sequentially. This solution is bad because it not only blocks the main thread but also is slow (due to sequential nature).

### Background thread

This is similar to the solution above. The difference is that the HTTP calls are made in a separate thread so the UI thread is not blocked. The HTTP calls in the separate thread are still made sequentially.

### Callbacks

This solution uses "async callbacks" provided by the HTTP library (retrofit2). HTTP calls are non-blocking. When a response arrives, the callback gets called. An atomic integer is used to check if all the calls have returned. When the last call is returned, the "updateResults" callback is called to update the UI. In other words, you don't see progress on the UI until the last moment.

### Suspend functions

This looks surprisingly similar to the "blocking" version. It's very readable and expresses exactly what needs to be done, unlike the callback version above. The HTTP library, retrofit2, supports "suspend" functions. The HTTP calls run in the UI thread but they don't block the thread. When the HTTP call is happening, they get suspended and the UI thread can be still responsive. When HTTP calls finish, they get resumed on the UI thread to make progress. Underneath, suspend functions are implemented via "continuation passing style".

### Concurrent suspend functions

This is the concurrent version of the solution above. Instead of iterating through the repos one at a time, we fire concurrent coroutines for each repo. We call "awaitAll" on the list of coroutines to wait for all of them to finish.

### Show Progress

This is similar to the "suspend functions" version. The improvement is the callback that updates the UI gets called every time a repo is done processing.

### Sow Progress via Channels

This is the concurrent version of the solution above. A channel is created for communication between producers (getting contributors of repos) and consumers (updating the UI with contributors). The list of contributors are sent over the channel. It's concurrent because all the coroutines run concurrently.
