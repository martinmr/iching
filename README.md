# I Ching

```
Arguments for the CLI

Usage: iching [OPTIONS] [COMMAND]

Commands:
  analyze  Sub-commands to analyze hexagrams
  help     Print this message or the help of the given subcommand(s)

Options:
  -m, --method <METHOD>
          The method used to generate the reading

          [default: yarrow-stalks]

          Possible values:
          - yarrow-stalks: A method using yarrow stalks. This is the traditional method, which is
                more involved. The probabilities that a yin or yang line will transform are not
                equal. This asymmetry reflects the traditional understanding of the intrinsic
                tendency of yin towards stability and of yang towards transformation
          - coin:          A method using random draws from a coin. This is a simplified method,
                which is easier to perform. The probabilities that a yin or yang line will transform
                are equal

  -r, --randomness <RANDOMNESS>
          Whether to use random.org or a pseudorandom number generator to generate the reading

          [default: random]

          Possible values:
          - random:       Generate truly random numbers using random.org
          - pseudorandom: Generate pseudo-random numbers using the system's random number generator

  -q, --question <QUESTION>
          The optional question to ask the I Ching

          [default: ]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```