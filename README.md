# Hermes CLI Invoice App

Hey there, fellow code enthusiasts and invoice-wranglers! Welcome to Hermes, my little pet project for managing invoices and generating PDFs, because who doesn't love a good command-line interface for handling money stuff, right?

## What's This All About?

Hermes is a CLI application written in Rust (because I'm a glutton for punishment and I love me some memory safety). It's designed to make invoice management a breeze, or at least as breezy as dealing with money can be. Whether you're a freelancer juggling clients or just someone who likes to overcomplicate their billing process, Hermes has got your back!

## Features

- Manage clients (because remembering who owes you money is important)
- Create and manage invoices (the bread and butter of getting paid)
- Generate PDF invoices (because PDFs make everything look official)
- Customizable settings (for when you want to feel in control of something in your life)

## Installation

First things first, make sure you've got Rust and Cargo installed. If you don't, head over to [rustup.rs](https://rustup.rs) and get that sorted.

Once you're all set with Rust, clone this repository:

```bash
git clone https://github.com/JordanRobo/Hermes-CLI-Invoice-App-
cd Hermes-CLI-Invoice-App-
```

Now, let's get Hermes installed:

```bash
cargo install --path .
```

And voila! You're ready to invoice like a pro (or at least like someone who knows how to use a CLI).

## Usage

Hermes is invoked using the `hermes` command. Here are the main commands to get you started:

- `hermes client`: Manage your clients
- `hermes invoice`: Create and manage invoices
- `hermes print`: Generate those fancy PDF invoices
- `hermes settings`: Customize Hermes to your liking

For more detailed information on each command, use the `--help` flag:

```bash
hermes --help
hermes client --help
hermes invoice --help
# You get the idea...
```

## Under the Hood

For those of you who like to peek behind the curtain, here's what's powering Hermes:

- [Comfy Table](https://github.com/nukesor/comfy-table): Because even CLI apps deserve pretty output
- [Clap](https://github.com/clap-rs/clap): For parsing command-line arguments without losing our sanity
- [Diesel](https://github.com/diesel-rs/diesel): Handling our database needs with the power of Rust
- [Inquire](https://github.com/mikaelmello/inquire): Making our CLI interactions smoother than a well-oiled machine

## Contributing

Found a bug? Have an idea for a feature? Want to tell me my code is terrible? Feel free to open an issue or submit a pull request! Just remember, with great pull requests comes great responsibility.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Because nothing says "I'm a serious developer" like choosing a license, right?

## Final Thoughts

Remember, the journey of a thousand invoices begins with a single CLI command. Happy invoicing, and may your clients always pay on time!

---

Built with ❤️, ☕, and an unhealthy obsession with Rust by Jordan Robinson.
