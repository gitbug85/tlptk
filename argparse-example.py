import argparse

# 1. Initialize the parser
parser = argparse.ArgumentParser(description="A simple math and greeting utility.")

# 2. Add Positional Argument (Required by default)
parser.add_argument("name", type=str, help="The name of the person to greet.")

# 3. Add Optional Arguments / Flags
parser.add_argument("-n", "--number", type=int, default=1, help="Number of times to greet.")
parser.add_argument("--loud", action="store_true", help="Make the greeting uppercase.")

# 4. Parse the arguments
args = parser.parse_args()

# Use the values
greeting = f"Hello, {args.name}!" * args.number
if args.loud:
    greeting = greeting.upper()

print(greeting)
