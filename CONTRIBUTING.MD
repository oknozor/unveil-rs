# Contributing

Welcome, if you are here it is probably that you want to contribute. 
Here are some advice to get you started! 

**Don't hesitate to ask question** on the issue tracker or on unveil [matrix channel](https://riot.im/app/#/room/#unveil-rs:matrix.org).
There is no dumb question and we will be happy to answer!

## Getting started

First of all, if you don't know what to do, you probably want to pick an issue. 
You might be interested in the 
[EASY](https://github.com/oknozor/unveil-rs/issues?q=is%3Aissue+is%3Aopen+label%3AEasy) ones to get started.

If you find a bug or have an idea for a new feature, don't hesitate to fill an issue, suggestions are welcome. 

Unveil has two main part : 
1. The Rust part which generate static assets for the presentation.
The rust logic involve a little bit of parsing and pre-processing, handle the live reload server and manipulate
the file system. Issues related to these topics have the [Rust tag](https://github.com/oknozor/unveil-rs/issues?q=is%3Aissue+is%3Aopen+label%3ARust).

2. The Javascript and CSS part, which are the default generated assets for the static presentation site. Those files are
located in the [src/assets](https://github.com/oknozor/unveil-rs/tree/master/src/assets) directory. If you are going to 
participate on this topic you must be aware that the index.html file is generated in by the Rust code using the 
[horrorshow crate](https://docs.rs/horrorshow/). Issues related to this have either the
 [JS](https://github.com/oknozor/unveil-rs/issues?q=is%3Aissue+is%3Aopen+label%3AJS+)
or the [CSS](https://github.com/oknozor/unveil-rs/issues?q=is%3Aissue+is%3Aopen+label%3ACSS) or both.

## Cloning the project

To start making changes, clone or fork the project : 

```shell script
git clone https://github.com/oknozor/unveil-rs.git
``` 

You can then run `cargo build` inside the `unveil-rs` directory and start coding ! 

## Conventional commit

Unveil use the [conventional commit spec](https://www.conventionalcommits.org/en/v1.0.0/). Please take time to read the 
document before submitting your changes. 

## Code quality 

Unveil use [rustfmt](https://github.com/rust-lang/rustfmt) to format the code and
[clippy](https://github.com/rust-lang/rust-clippy) to detect mistakes.
Please install them and run those command before submitting a pull request 
(this is not needed if you are only making changes to Javascript or CSS files). 

#### rustfmt

1. install :
    ```shell script
    rustup component add rustfmt
    ```

2. run :

    run this command inside unveil project directory:

    ```shell script
    cargo fmt --all
    ```

#### install clippy 

1. install :
    ```shell script
    rustup component add clippy
    ```

2. run :
    run this command inside unveil project directory

    ```shell script
    cargo clippy
    ```
   
## Contributing to the Javascript/CSS part

The most straightforward way to start editing the front-end is to install your version of unveil : 

```shell script
cargo install --path . 
```

You can then use unveil to init a presentation project : 

````shell script
unveil init test_presentation
cd test_presentation
unveil build
````

#### Editing existing assets

Once you unveil project is build you can edit existing assets in the `public` directory.
Be careful when making changes, you don't want to use live-reload : **running `unveil serve` on your project will 
override some assets on re-build**. 

#### Adding new assets

If you want to add a javascript library or additional css stylesheet to the project you need include them in the 
generated assets on rust side. This happens in [assets/mod.rs](src/assets/mod.rs) and [unveil.rs](src/unveil.rs). 
If you need help on how to do this, don't hesitate to ask.


## Submit a pull request

You are done making changes ? Congrats ! You can now submit a pull request on github.
