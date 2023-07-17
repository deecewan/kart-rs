# Analyzer

This is the crate that does the actual analysis of the images. It reads a
screenshot, and from there decides if it knows what kind of screen it is. If it
does, the screen is processed and any relevant data is extracted (course name
for an intro screen, player positions/items for a race screen, etc).

## Utils

The program relies on reference images to compare against. If these need to be
generated (i.e. if there are new tracks released), you can use the following:

### generate_intro_references

```
cargo build --release --bin generate_intro_references
../target/release/generate_intro_references --help
```
