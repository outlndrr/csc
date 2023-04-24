# csc

Cloudflare status checker cli tool;

## Quick start
```bash
    cargo run -- -n <CityName>
```

## Example
```bash
    cargo run -- -n London
```
Output:
```bash
Some(
    Component {
        id: "shcqh0p22750",
        name: "London, United Kingdom - (LHR)",
        status: Operational,
        created_at: "2014-10-27T20:36:04.069Z",
        updated_at: "2023-04-14T03:32:29.961Z",
        position: 25,
        description: None,
        showcase: false,
        start_date: None,
        group_id: Some(
            "zqxhg7y54vy8",
        ),
        page_id: "yh6f0r4529hb",
        group: false,
        only_show_if_degraded: false,
    },
)
```