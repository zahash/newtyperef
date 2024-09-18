<div align="center">

<pre>
███╗   ██╗███████╗██╗    ██╗████████╗██╗   ██╗██████╗ ███████╗██████╗ ███████╗███████╗
████╗  ██║██╔════╝██║    ██║╚══██╔══╝╚██╗ ██╔╝██╔══██╗██╔════╝██╔══██╗██╔════╝██╔════╝
██╔██╗ ██║█████╗  ██║ █╗ ██║   ██║    ╚████╔╝ ██████╔╝█████╗  ██████╔╝█████╗  █████╗  
██║╚██╗██║██╔══╝  ██║███╗██║   ██║     ╚██╔╝  ██╔═══╝ ██╔══╝  ██╔══██╗██╔══╝  ██╔══╝  
██║ ╚████║███████╗╚███╔███╔╝   ██║      ██║   ██║     ███████╗██║  ██║███████╗██║     
╚═╝  ╚═══╝╚══════╝ ╚══╝╚══╝    ╚═╝      ╚═╝   ╚═╝     ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝     
------------------------------------------------------------------------------
Generate custom newtype reference types
</pre>

[![Crates.io](https://img.shields.io/crates/v/newtyperef.svg)](https://crates.io/crates/newtyperef)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

## 🚀 Installation

include it in your `Cargo.toml` under `[dependencies]`

```toml
newtyperef = "*"
```

## 🧑‍💻 Usage examples

### Basic Usage

```rust
use newtyperef::newtyperef;

#[newtyperef]
pub struct Name(pub String);

fn main() {
    let mut name = Name("X Æ A-12".into());

    let name_ref: NameRef<'_> = name.as_ref();
    let name_ref_inner: &String = name_ref.deref();

    let mut name_mut: NameRefMut<'_> = name.as_mut();
    let name_mut_inner: &String = name_mut.deref();
    let name_mut_inner: &mut String = name_mut.deref_mut();
}
```

### Customizing Reference Types

```rust
use newtyperef::newtyperef;

#[newtyperef(ref = str, mut = str)]
pub struct Name(pub String);

fn name_example() {
    let mut name = Name("X Æ A-12".into());

    let name_ref: NameRef<'_> = name.as_ref();
    let name_ref_inner: &str = name_ref.deref();

    let mut name_mut: NameRefMut<'_> = name.as_mut();
    let name_mut_inner: &str = name_mut.deref();
    let name_mut_inner: &mut str = name_mut.deref_mut();
}


#[newtyperef(ref = [String])]
pub struct Emails(pub Vec<String>);

fn emails_example() {
    let mut emails = Emails(vec!["a@a.com".into(), "b@b.com".into()]);

    let emails_ref: EmailsRef<'_> = emails.as_ref();
    let emails_ref_inner: &[String] = emails_ref.deref();

    let mut emails_mut: EmailsRefMut<'_> = emails.as_mut();
    let emails_mut_inner: &Vec<String> = emails_mut.deref();
    let emails_mut_inner: &mut Vec<String> = emails_mut.deref_mut();
}

```

## 🌟 Connect with Us

M. Zahash – zahash.z@gmail.com

Distributed under the MIT license. See `LICENSE` for more information.

[https://github.com/zahash/](https://github.com/zahash/)

## 🤝 Contribute to `newtyperef`!

1. Fork it (<https://github.com/zahash/newtyperef/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## ❤️ Show Some Love!

If you find `newtyperef` helpful and enjoy using it, consider giving it a [⭐ on GitHub!](https://github.com/zahash/newtyperef/stargazers) Your star is a gesture of appreciation and encouragement for the continuous improvement of `newtyperef`.