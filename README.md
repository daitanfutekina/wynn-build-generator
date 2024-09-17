# Wynn Build Generator
Easily compare hundreds of item combinations to find the best builds!

Check out the website [here](https://yellowly.github.io/wynn-build-generator/)

## How to use

Insert all items you want to include in the search, build requirements (ie; a minimum effective health requirement), and what stat to optimize for (ie; melee dps)

## Features

- Test out hundreds of item combinations
- Find builds to meet your playstyle requirements
- View a summary of resulting build stats
- Easily view builds in [wynnbuilder](https://wynnbuilder.github.io/builder/)

## Documentation
This site is built around an alpha-beta search algorithmn which can quickly compare many combinations of wynncraft items.
There are also various other optimizations to ensure build generation can be done as quickly as possible

See [src/best_build_search](https://github.com/Yellowly/wynn-build-generator/tree/master/src/best_build_search) for more info on the search algorithmn
See [src/wynn_data](https://github.com/Yellowly/wynn-build-generator/tree/master/src/wynn_data) for more info on item parsing and build generation
Please don't see [src/website](https://github.com/Yellowly/wynn-build-generator/tree/master/src/website), [style.css](https://github.com/Yellowly/wynn-build-generator/blob/master/style.css), or [rs_generator.py](https://github.com/Yellowly/wynn-build-generator/blob/master/rs_generator.py) because all the code there is shit.
