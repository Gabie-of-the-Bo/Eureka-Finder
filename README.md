# Eureka Finder

A pandigital operation searcher powered by Rust and Angular.

# Overview

Eureka Finder is a tool that discovers pandigital operations – mathematical expressions where each digit is used exactly once to form a valid equation. For example, using the digits 1–9 once each, the tool can search for an expression that approximates constants like π or e within a given precision.

The project combines a Rust brute-force solver with a basic Angular frontend, making it easy to use. Does it have any real-world use? No, but it sure was fun to build!

Take a look at it [here](https://gabie-of-the-bo.github.io/Eureka-Finder/).

# How to Use

## 1. Choose Your Target and Precision
Decide which number you want to approximate and how many decimal places you need. Fill the text boxes with that information.
> **Note:** The library supports complex numbers, but the frontend currently works only with real numbers.

## 2. Understand What You Can Approximate
You can approximate **any number** using **any set of digits**. Examples:  
- All digits from 0 to 9  
- Two of each digit from 0 to 5  
- Ten repetitions of the digit 5  

## 3. Enter Digits in the Text Box
Use the following formats to specify your digits:  
- **Single digit:** `1`, `2`, `3` …  
- **Range of digits:** `0-5`, `4-9`, `10-20` …  

## 4. Examples
- Numbers from 0 to 9: `0-9`  
- Two of each from 0 to 5: `0-5, 0-5`  
- Ten 5s: `5, 5, 5, 5, 5, 5, 5, 5, 5, 5`

# Contributing

Contributions are welcome! Open an issue for bugs or feature requests or submit a PR with improvements.