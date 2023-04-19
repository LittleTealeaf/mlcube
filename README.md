# MlCube

An attempt to use Deep-Q Learning to solve a rubik's cube.

Yes, this README is partially completed. :)

## What is this project?

This project is a personal research project that I started along with my academic advisor at Quinnipiac University in the Summer of 2022. I had recently switched to being a double major with Data Science, and wanted to dive into and learn more about machine learning.

Previously, I only had a bit of exposure to neural networks and how they work through my Algorithms for Data Science course (The final project can be found here: [LittleTealeaf/DS-210-Final](https://www.github.com/LittleTealeaf/DS-210-Final)). I wanted to dive much deeper into machine learning, and with some guidance with my advisor, I decided to learn and attempt to implement the common reinforcement learning solution of Deep-Q Learning.

Now, I thought I could get it done in a summer. I was wrong, so here we are! I'll try to update this readme at various points to keep it up to date with what I have done, what I've learned, and what I need to do.

## General Structure

This project has multiple facets to it. Over the iterations, I've found the benefit of using rust to host the replay / environment, and using a database to store all of the data (even if, uh, it's not the best choice).

The following sections describe each used directory / sub-project of this project.

### `sql/`

This directory contains all of the setup scripts used for SQL configuration. I am using a Microsoft SQL Server to store both the training data, as well as (regrettably) the network weights and biases.

You might be asking, _why?!_. Well, first of all I got tired of using `.json` files to store everything. I didn't know what I would do until I decided to take an Advanced Database Course in the Spring 2023 Semester. In the course, we needed to pick a large project to implement in a database, and I decided that it was the perfect excuse to get back to working on this project. Thus, I ported all of the data from json to a Microsoft SQL Server Database.

### `dashboard/`

Similar to the `sql/` directory, I needed to create an "application" that consumes and displays the data into reports (or a dashboard as I've done here). Is it leagues more work than what is asked in the course? Yes. Have I spent hours upon hours on it? Also yes. Is it worth it? _Probably not but I wouldn't be writing this README if we only did things that were worth it._

This part of the project is basically as [Next.js](https://nextjs.org/) website written in Typescript that uses [Prisma](https://www.prisma.io/) to connect to the database. I decided to use this stack because I've had prior experiences with Next.js (however, Next.js 13 threw me a curveball), Prisma was the only ORM that seemed to comply with the project requirements (specifically that string concatenation is not allowed for building SQL queries with user input).

### `rust/`

Let's get this out of the way: **WHY RUST?!**

1. I've been learning rust and it's already my favorite language.
2. In prior iterations, one of the biggest time-sinks for executing the code was both compiling / sampling the replay, as well as simulating the rubik's cube.
3. I don't want to work in CPython
4. Because I can

Basically, python efficiency is less than optimal when it comes to certain tasks. I've decided to implement two major components of Deep-Q Learning within Rust: The Environment, and the Replay Database.

I am using [Py03](https://github.com/PyO3) in order to compile Rust into a Python module. Specifically, I am using [Maturin](https://github.com/PyO3/maturin) (`maturin develop`) to install the module into the current virtual python environment. 


### `python/`


This is the directory where it really gets fun. Kind-of. This basically contians the following important steps to the project:

- Connecting to and interacting with the database
- The python class describing a Network
- The python class describing an Agent
- The best python script you have ever seen that runs it all. 

The scripts in `python/` are primarily going to be run in a dev-environment, described in the `.devcontainer/` directory. However, I suppose it could be used elsewhere.
