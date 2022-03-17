# Texting Robots: Common Crawl Test

To ensure [Texting Robots](https://github.com/Smerity/texting_robots) handles real world usage and doesn't panic in unknown situations this demo takes a directory full of `robots.txt` responses in WARC format (i.e. `*.warc.gz`) and then tests against each valid response.

In testing against Common Crawl's [January 2022 crawl archive](https://commoncrawl.org/2022/02/january-2022-crawl-archive-now-available/) the library processed over 54.9 million `robots.txt` responses sourced from the 140 gigabytes of compressed WARC files. The run took under 2 hours on an Intel i7-1065G7 CPU laptop.

Due to the parallel nature of the problem and using Rayon the running time should improve linearly with additional compute.

*Note:* This is only intended for real world / panic safety testing. For testing against the specification see the test suite in the Texting Robots crate.

## Usage

To obtain the data get the `robotstxt.paths.gz` file from Common Crawl and run in a robots directory:

```bash
wget https://commoncrawl.s3.amazonaws.com/crawl-data/CC-MAIN-2022-05/robotstxt.paths.gz
mkdir robots
cd robots
zcat ../robotstxt.paths.gz | xargs --max-procs 64 -I '{}' wget --no-verbose --continue https://commoncrawl.s3.amazonaws.com/'{}'
```

Beware: The January 2022 crawl uses 140 gigabytes of space stored in 72,000 files.

To run the sanity check:

```bash
cargo run --release robots/ > bad_robots.txt
```

Any `robots.txt` files that fail processing will be written to `stdout` with their source URL and the underlying error.
Afterwards we can sort and deduplicate any found bad robots:

```bash
cat bad_robots.txt | sort | uniq > bad_robots_sorted.txt
```

This file has been provided, noting that only 151 unique `robots.txt` are unable to be processed.
As can be seen upon inspecting the file this is understandable given some of the "rules" include Base 64 encoded images, obvious buggy repetitions, and even an adversarial example of a megabyte of repeated "A".
