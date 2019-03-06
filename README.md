# Dhash

This crate allows a `dhash` signature to be constructed from an image.

Requires the `image` crate

A `dhash` is a differential gradient hash that compares the difference in gradient between adjacent pixels, and provides a 64 bit signature of an image.

A `dhash` can be used to compare against other images for similarity and is resilient to differences in:

* Aspect Ratio
* Image Size
* Brightness and Contrast


Implementation details taken from the [Kind of Like That](http://www.hackerfactor.com/blog/?/archives/529-Kind-of-Like-That.html) blog

## Usage (CLI)

Install this crate:
```bash
cargo install dhash
```

Run `dhash <img1>` to print out a `dhash` of the image at path `img1`

```bash
$ dhash test.jpg
dhash for test.jpg is `13547707017824698364`
```

Run `dhash <img1> <img2>` to print out a `dhash` of both images and the distance between them (a lower number is closer):

```bash
$ dhash test.jpg other.jpg
dhash for test.jpg is `4485936524854165493`
dhash for other.jpg is `3337201687795727957`
distance is: 11
```
