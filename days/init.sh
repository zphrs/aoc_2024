SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
if ! command -v "aoc"
then
echo "Error: aoc-cli could not be found. Install the aoc-cli by running:\n\$ cargo install aoc-cli"
exit 1
fi
if ! command -v "cargo-generate"
then
echo "Error: aoc-cli could not be found. Install cargo-generate by running:\n\$ cargo install cargo-generate"
exit 1
fi
read -n 128 -p "Paste your Advent of Code session cookie here: " -s varcookie
echo "$varcookie" > "$SCRIPT_DIR/.adventofcode.session"
chmod +x "$SCRIPT_DIR/day"
chmod +x "$SCRIPT_DIR/template/init.sh"