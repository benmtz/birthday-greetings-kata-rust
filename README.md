

# Rust implementation of [the birthday greeting kata](http://matteo.vaccari.name/blog/archives/154)

## Running application

In order to run the application you need a smtp server running, it's possible to start one easily thanks to [https://hub.docker.com/r/mailhog/mailhog/](https://hub.docker.com/r/mailhog/mailhog/), just ```docker-compose up``` will start one instance on localhost:1025.

Then run 

```
cargo run --bin birthday_greeting_cli -- 
    -i employee.csv \
    --smtp-server localhost \
    --smtp-port 1025 \
    --smtp-username "ben" \
    --smtp-password "********" \
    --from "Ben <noreply@ben.org>"
```

If you want to run it without an smtp server it's possible to run it in dry-run mode, like so:

```
cargo run --bin birthday_greeting_cli -- 
    -i employee.csv \
    --smtp-server localhost \
    --smtp-port 1025 \
    --smtp-username "ben" \
    --smtp-password "********" \
    --from "Ben <noreply@ben.org>" \
    --dry-run

# Will output :
# [1952-04-17] Happy birthday Enola Quitzon
# [1986-04-17] Happy birthday Genesis Macejkovic
# [1961-04-17] Happy birthday Mayra Beier
# [1977-04-17] Happy birthday Ellie McLaughlin
# [1968-04-17] Happy birthday Jamey Bednar
# [1962-04-17] Happy birthday Hudson Huel
# [1952-04-17] Happy birthday Elliot Swift
# [1966-04-17] Happy birthday Fay Torphy
# [1977-04-17] Happy birthday Terrence Cormier
# [1970-04-17] Happy birthday Lance Miller
# [1977-04-17] Happy birthday Roberto Keebler
# [1959-04-17] Happy birthday Luella Walker
# [1967-04-17] Happy birthday Mathilde Stroman
# [1965-04-17] Happy birthday Kristopher Borer
```
