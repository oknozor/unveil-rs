#  Unveil Rs [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.com/oknozor/unveil-rs.svg?branch=master
[travis]: https://travis-ci.com/oknozor/unveil-rs
[Latest Version]: https://img.shields.io/crates/v/unveil-rs.svg
[crates.io]: https://www.crates.io/crates/unveil-rs

Unveil Rs is a tool to create presentations from markdown files.
It is inspired by [reveal.js](https://github.com/hakimel/reveal.js) 
, [mdbook](https://github.com/rust-lang/mdBook) and [zola](https://www.getzola.org/).

## What does it look like ?

See the [live demo](https://oknozor.github.io/unveil-rs/).

## Installation

1. From crates.io 

At the moment unveil is only available on [crates.io](https://crates.io). 

To get started you will need to install rust and then type the following command in a terminal :

```shell script
cargo install unveil-rs --version=0.1.2-alpha
```

Note : the `--version` flag is required while unveil version is still in alpha. 

2. From git

If you want the latest you can run :
```shell script
cargo install --git https://github.com/oknozor/unveil-rs.git unveil-rs
``` 

## Usage

### Initialize 

To initialize an empty project run :

```shell script
unveil init mypresentation
```

This will create the following directory structure :

```shell script
├── slides
│   └── landing.md
└── unveil.toml
```

### Build and run

To build your project run : 
```shell script
cd mypresentation && unveil build
```

This command generate the following files : 
```shell script
├── public
│   ├── fontawesome
│   │   ├── css
│   │   │   └── fontawesome.css
│   │   └── webfonts
│   │       ├── (...)
│   ├── clipboard.js
│   ├── highlight.css
│   ├── highlight.js
│   ├── index.html
│   ├── livereload.js
│   ├── unveil.css
│   └── unveil.js
│   └── user_css.css
├── slides
│   └── landing.md
└── unveil.toml
```

Actually the build command is optional, you can directly run `unveil serve` inside your
project root directory. This will build the static site and start serving it on `localhost:7878`.

From this point you can start editing your markdown slides. The site will reload as you edit it. 

### Add new slides

To add a slide run `unveil new myslide` inside your project root directory. it will create a new markdown file 
`myslide.md` in the `slides/` directory and add a slide entry in the `unveil.toml` config file. 

```toml
name = "mypresentation"
language = "EN"
gitignore = true
slides = ["landing.md", "myslide.md"]
```

### Add style to your slides

Inspired by [zola's frontmatter](https://www.getzola.org/documentation/content/page/#front-matter) unveil slides can be 
styled with a style matter block. The Sass style matter is a style attached to the current slide embedded in a file at the beginning of 
the file enclosed by triple pluses (+++). If your slide does not have additional styling, the opening and closing +++ are optional.

Example : 
```markdown 
+++
background-color: black;
color: white;  

h1 {
    color: red;
}
+++
# I am red 

I am white and my background is black
```

### Custom CSS properties 

Unveil use some custom CSS properties to help you design your slides.

For example you can add transition on slides using the `--on-enter-animation` CSS property on your slide's stylematter :

```markdown
+++
--on-enter-animation: zoom-in
+++
# I will zoom in on enter
```

#### Available custom CSS properties

| name                  |   allowed values                    | status       |  
| :---                  | :---                                |  :---        |
|`--on-enter-animation` | `fade-in`, `zoom-in`                | implemented  |

If you want to see more custom properties and transitions in unveil, please let us know ! 

### Hljs

Unveil use hljs to generate pretty code snippet. Rust code can be played thanks to [the rust playground project](https://play.integer32.com/help).


### Commands

| name   | description                              |   args                              | 
| :---   | :-----------                             | :---                                |
|init    | new project                              |  `PROJECT_NAME` default = `unveil`  |
|build   | build the project                        |                                     |
|clean   | wipe the public  directory               |                                     |
|serve   | serve the project, build it if needed    |                                     |
|add     | create a new slide                       | `SLIDE_NAME` required               |

## Contributions

Unveil is at a very early stage of it's development and any help is welcome. If you want to participate read the 
[contributing guidelines](CONTRIBUTING.MD) and pick an [issue](https://github.com/oknozor/unveil-rs/issues)! 

## Getting Help 

Need help ? You can contact us via our [matrix channel](https://riot.im/app/#/room/#unveil-rs:matrix.org) or ask a question
on the [issue tracker](https://github.com/oknozor/unveil-rs/issues).




