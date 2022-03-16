# Texting Robots: Common Crawl Test

To ensure [Texting Robots](https://github.com/Smerity/texting_robots) handles real world usage and doesn't panic in unknown situations this demo takes a directory full of `robots.txt` responses in WARC format (i.e. `*.warc.gz`) and then tests against each valid response.

In testing against Common Crawl's [January 2022 crawl archive](https://commoncrawl.org/2022/02/january-2022-crawl-archive-now-available/) the library processed over 54.8 million `robots.txt` responses sourced from the 140 gigabytes of compressed WARC files. The run took under 2 hours on an Intel i7-1065G7 CPU laptop.

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

Beware: The January 2022 crawl uses 140 gigabytes of space from 72,000 files.

To run the sanity check:

```bash
cargo run --release ~/Corpora/commoncrawl/robots/
```
