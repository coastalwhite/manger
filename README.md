![GitHub last commit](https://img.shields.io/github/last-commit/coastalwhite/manger?style=for-the-badge)
![GitHub branch checks state](https://img.shields.io/github/checks-status/coastalwhite/manger/main?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/coastalwhite/manger?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/coastalwhite/manger?style=for-the-badge)

# Manger

# A performant, low-level, lightweight and intuitive combinatoric parser library.

Manger allows for translation of the intuition developed for _Rust_'s primitive and standard
library types into your intuition for using this library. Most of the behaviour is defined with
the `Consumable` trait, which can be easily implemented using the `consume_struct` and
`consume_enum` macros.

This library is suited for deterministic regular languages. It is optimally used in addition to
a predefined syntax. For example, if you have a predefined
[EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form), it is really easy to
implement the syntax within this crate.

# Getting Started

To get started with implementing `Consumable` on your own traits, I suggest taking a look at
the `consume_struct` or `consume_enum` documentation. Then you can come back here and look
at some common patterns.

## Common patterns

Parsing and thus consuming has a lot of often used patterns. Ofcourse, these are very easily
available here aswell.

### Concatenation

Often we want to express that two patterns follow eachother in a `source` string. For example,
you might want to express that every `Line` is followed by a `';'`. In manger there are two
ways to do this.

#### Macro's

The first way, and the preferred way, is with the `consume_struct` or `consume_enum` macros
where you can present sequential consume instructions. You can see in the following example that
we are first consuming a `'('`, followed by a [`i32`](https://doc.rust-lang.org/std/primitive.i32.html),
followed by a closing `')'`.

```rust
use manger::{ Consumable, consume_struct };

struct EncasedInteger(i32);
consume_struct!(
    EncasedInteger => [
        > '(',
        value: i32,
        > ')';
        (value)
    ]
);
```

#### Tuples

Another way to represent the same concept is with the tuple type syntax. This can be done with
up to 10 types. Here we are again parsing the same `(i32)` structure.

```rust
use manger::chars;

type EncasedInteger = (chars::OpenParenthese, i32, chars::CloseParenthese);
```

### Repetition

Most of the time you want to represent some kind of repetition. There are a lot of different
way to represent repetition. Here there are two easy ways.

#### Vec

The easiest way to do repetition is with the [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
This will consume 0 or
more instances of type `T`. Ofcourse, the type `T` has have has `Consumable` implemented.
Here you can see how what that looks like:

> Since [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html) will consume instances of type `T` until it finds a error, it
can never fail itself. You are therefore safe to unwrap the result.

```rust
use manger::{ Consumable, consume_struct };

struct EncasedInteger(i32);
consume_struct!(
    EncasedInteger => [
        > '[',
        value: i32,
        > ']';
        (value)
    ]
);

let source = "[3][-4][5]";

let (encased_integers, _) = <Vec<EncasedInteger>>::consume_from(source)?;

let sum: i32 = encased_integers
    .iter()
    .map(|EncasedInteger(value)| value)
    .sum();

assert_eq!(sum, 4);
# Ok::<(), manger::ConsumeError>(())
```

#### OneOrMore

The other easy way to do repetition is with `OneOrMore<T>`. This allows for
consuming 1 or more instances of type `T`. And again, type `T` has to have `Consumable`
implemented. Here you can see what that looks like:

```rust
use manger::{ Consumable, consume_struct };
use manger::common::OneOrMore;

struct EncasedInteger(i32);
consume_struct!(
    EncasedInteger => [
        > '[',
        value: i32,
        > ']';
        (value)
    ]
);

let source = "[3][-4][5]";

let (encased_integers, _) = <OneOrMore<EncasedInteger>>::consume_from(source)?;

let product: i32 = encased_integers
    .into_iter()
    .map(|EncasedInteger(value)| value)
    .product();

assert_eq!(product, -60);
# Ok::<(), manger::ConsumeError>(())
```

### Optional value

To express optional values you can use the [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) standard rust
type. This will consume either 0 or 1 of type `T`.

> Since [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html) will consume a instance of type `T` if it finds no error, it
can never fail itself. You are therefore safe to unwrap the result.

```rust
use manger::consume_struct;
use manger::chars;

struct PossiblyEncasedInteger(i32);
consume_struct!(
    PossiblyEncasedInteger => [
        : Option<chars::OpenParenthese>,
        value: i32,
        : Option<chars::CloseParenthese>;
        (value)
    ]
);
```

### Recursion

Another common pattern seen within combinatoric parsers is recursion. Since rust types need to
have a predefined since, we cannot do direct type recursion and we need to do heap allocation
with the [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html) type from the standard library. We can make a prefixed
math expression parser as followed:

```rust
use manger::consume_enum;
use manger::common::{OneOrMore, Whitespace};

enum Expression {
    Times(Box<Expression>, Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Constant(u32),
}

consume_enum!(
    Expression {
        Times => [
            > '*',
            : OneOrMore<Whitespace>,
            left: Box<Expression>,
            : OneOrMore<Whitespace>,
            right: Box<Expression>;
            (left, right)
        ],
        Plus => [
            > '+',
            : OneOrMore<Whitespace>,
            left: Box<Expression>,
            : OneOrMore<Whitespace>,
            right: Box<Expression>;
            (left, right)
        ],
        Constant => [
            value: u32;
            (value)
        ]
    }
);
```

### Whitespace

For whitespace we can use the `manger::common::Whitespace` struct. This will consume any
utf-8 character that is identified as a whitespace character by the [`char::is_whitespace`](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace)
function.

### Either

If two possibilities are present for consuming there are two options to choose from. Both are
valid in certain scenarios.

#### Macro

Using the `consume_enum` you can create an struct which can be consuming in a number of
options and you can see which option was selected. If you need to see which of the different
options was selected, this should be your choice.

#### Either<L, R>

You can also use the [`Either<L, R>`](https://docs.rs/either/1.6.1/either/enum.Either.html) type to represent the either
relationship. This option is preferred if we do not care about which option is selected.

## Roadmap

See the [open issues](https://github.com/coastalwhite/manger/issues) for a list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
