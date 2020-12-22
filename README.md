# Manger

## A performant, low-level, lightweight and intuitive parsing library

## Why use Manger

Manger is really easy to use, easy to understand and really performant.
It is inspired by combinatoric parsers.
Manger has a optimatized standard library including parsing for integers,
floating-point and UTF-8.

## Getting Starting

Getting started with Manger is really easy.
Let's start with the core concept.

### Core Concept

To parse a `struct X`, we always need a function which kinda looks like
`&str -> X`. Manger uses this concept as well.
But instead of consuming the entire `&str`, we consume a part of the string.
So we get a function, which looks similar to `&str -> (X, &str)`.
This way we can easily combine different parsers.
We can chain two parsers of `struct X` and `struct Y` together
to get a function like `&str -> ((X, Y), &str)`.

### Actually coding

The core trait used by _Manger_ is `Consumable`.
We need to always import this trait to use the parsing capability.

After importing `Consumable` we can parse integers and floating points alike.

``` rust
use manger::Consumable;

// We can parse an integer
// consume_from is the function to parse.
let (integer, unconsumed) = u32::consume_from("42 is the answer.").unwrap();
assert_eq!(integer, 42);
assert_eq!(unconsumed, " is the answer.");

// We can also parse a floating point number
let (floating_point, unconsumed) = f32::consume_from("42.0 0.42").unwrap();
assert_eq!(floating_point, 42.0f32);
assert_eq!(unconsumed, " 0.42");
```

We can also use the standard library to parse characters.
Here we parse an amount of money using the dollar sign from the standard library.

``` rust
use manger::Consumable;
use manger::chars;

// Consume the '$' character.
let (_, unconsumed) = <chars::Dollar>::consume_from("$12.50, please!").unwrap();
assert_eq!(unconsumed, "12.50, please!");

// Consume the '12.50' floating point number.
let (floating_point, unconsumed) = f32::consume_from(unconsumed).unwrap();
assert_eq!(floating_point, 12.5f32);
assert_eq!(unconsumed, ", please!");
```

Ofcourse, we also create implement parsing for our own structs.
Here we create a consume parser for the string 'l33t'.

``` rust
use manger::Consumable;
use manger::chars;
use manger::chars::CharConsumeError;

struct L33T;

impl Consumable for L33T {
    type ConsumeError = CharConsumeError;

    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
        let (_, unconsumed) = <chars::alphabet::lower::L>::consume_from(s)?;
        let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
        let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
        let (_, unconsumed) = <chars::alphabet::lower::T>::consume_from(s)?;

        Ok((L33T, unconsumed))
    }
}
```

And now we can use it:

``` rust
# use manger::Consumable;
# use manger::chars;
# use manger::chars::CharConsumeError;
#
# struct L33T;
#
# impl Consumable for L33T {
#    type ConsumeError = CharConsumeError;
#
#    fn consume_from(s: &str) -> Result<(Self, &str), Self::ConsumeError> {
#       let (_, unconsumed) = <chars::alphabet::lower::L>::consume_from(s)?;
#       let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
#       let (_, unconsumed) = <chars::nums::Three>::consume_from(unconsumed)?;
#       let (_, unconsumed) = <chars::alphabet::lower::T>::consume_from(unconsumed)?;
#
#       Ok((L33T, unconsumed))
#    }
# }
#
let (_, unconsumed) = <L33T>::consume_from("l33t is a hackername!").unwrap();
assert_eq!(unconsumed, " is a hackername!");
```

#### Removing the boilerplate

But this is a lot of boiler-plate code for just consuming a string.
We can do better!

``` rust
use manger::Consumable;
use manger::manger_str;

manger_str!( "l33t" => L33T );
let (_, unconsumed) = <L33T>::consume_from("l33t is a hackername!").unwrap();
assert_eq!(unconsumed, " is a hackername!");
```
