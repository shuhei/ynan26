# ynan26

You Need An N26—`ynan26` connects your N26 account to YNAB.

## Warning

Make sure to keep your credentials secret ([especially N26 password](https://github.com/Rots/n26-api#security-warning)). As stated in the MIT license, I'm not responsible for any consequences caused by using this software.

## Usage (for now)

`ynan26` posts new transactions on your N26 account to YNAB. Even if you run it multiple times, it won't create duplicated transactions on YNAB, thanks to `import_id` in YNAB API.

Run the `ynan26` command with the following environment variables:

- `YNAB_PERSONAL_TOKEN`: [A personal access token of YNAB API](https://api.youneedabudget.com/#personal-access-tokens).
- `YNAB_BUDGET_ID`: The budget ID of YNAB that you want to connect N26 to.
- `YNAB_ACCOUNT_ID`: The account ID of YNAB that you want to connect N26 to. This is typically your N26 account on YNAB.
- `N26_USERNAME`: Your N26 username, which is typically your email address.
- `N26_PASSWORD`: Your N26 password. [Be careful!](https://github.com/Rots/n26-api#security-warning)

The debug build reads environment variables from `.env` file.

## Development

Run:

```sh
cargo run
```

Build:

```sh
cargo build
```

Test:

```sh
cargo test
```

## License

MIT
