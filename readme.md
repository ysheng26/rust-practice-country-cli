> https://restcountries.com/v3.1/all?fields=name,region,capital,currencies,population,languages,area


Country CLI — Requirements

Setup

	•	Create a new project with cargo new country-cli
	•	Dependencies you’ll need: clap (with derive feature), reqwest (with json and blocking features), serde (with derive feature), serde_json

Core Functionality

Fetch all countries from https://restcountries.com/v3.1/all and deserialize into your own Country struct.

Filter by:

	•	--region <region> — e.g. Europe, Asia, Americas (optional, if omitted show all)

Sort by (use a SortBy enum with ValueEnum):

	•	--sort population (default)
	•	--sort area
	•	--sort name

Limit results:

	•	--top <n> — show only the top N results (default 10)

Output

Print each result to the terminal. Implement Display on Country so printing is just println!("{}", country). Format should look roughly like:

1. Germany (Europe) — Pop: 83,000,000 | Area: 357,114 km²
2. France (Europe)  — Pop: 67,000,000 | Area: 551,695 km²


Country Struct

The API returns a lot of fields — you only need to deserialize what you use. At minimum:

	•	name (the API nests this as name.common)
	•	population
	•	area
	•	region

Serde lets you map nested fields with #[serde(...)] attributes — figuring that out for name.common is part of the exercise.

Iterator Requirements

You must use iterator chains (no manual for loops) for:

	•	Filtering by region
	•	Sorting
	•	Taking top N
	•	Formatting output with an index number

Stretch Goals (if you finish fast)

	•	--language <lang> filter — e.g. all countries that speak Spanish. The languages field in the API is a nested map, so this is a good filter_map / flat_map exercise
	•	--min-population <n> flag
	•	Sort ascending/descending with --asc flag

Come back when you’re stuck or done — either is a good reason to talk.
