# kuntakoodit

Look up Finnish municipality codes (kuntakoodi) by municipality name. Data comes from the [Statistics Finland municipality classification](https://stat.fi/en/luokitukset/kunta/) and is embedded in the crate.

## Install

```bash
cargo add kuntakoodit
```

## Usage

```rust
// Finnish or Swedish name, case-insensitive
let espoo = kuntakoodit::by_name("esbo").unwrap();
assert_eq!(espoo.code, "049");
assert_eq!(espoo.name_sv, "Esbo");

let akaa = kuntakoodit::by_code("020").unwrap();
assert_eq!(akaa.name_fi, "Akaa");

for municipality in kuntakoodit::all() {
    println!("{} {}", municipality.code, municipality.name_fi);
}
```

## Data

`data/kunnat_fi.json` and `data/kunnat_sv.json` are the Statistics Finland [classification API](https://data.stat.fi/api/classifications/v2/classifications) responses for the currently effective `kunta_1_*` classification, pretty-printed with `jq`. [`data/ATTRIBUTION.md`](data/ATTRIBUTION.md) names the exact classification version.

A [scheduled workflow](.github/workflows/update-data.yml) refreshes them (and the attribution) on the 1st of every month and opens a pull request when they change. To refresh manually:

```bash
task update-data
```

Pull requests opened with the default `GITHUB_TOKEN` do not trigger CI. Add a `DATA_UPDATE_TOKEN` repository secret (fine-grained PAT or GitHub App token with contents and pull request write access) to have CI run on them. The repository must also allow GitHub Actions to create pull requests (Settings → Actions → General).

## Development

```bash
task test
task lint
task format
```

## License

The code is licensed under [MIT](LICENSE).

The bundled municipality data (`data/`) is © Statistics Finland and licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/). See [`data/ATTRIBUTION.md`](data/ATTRIBUTION.md) for the source and classification version.
