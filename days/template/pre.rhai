let d = parse_int(variable::get("project-name").sub_string(3));
let d = d.to_string();

file::write("init.sh", 
`aoc download -d ${d} --session-file ../.adventofcode.session
SCRIPT_DIR=`+"$"+`( cd -- "$( dirname -- "`+"$"+`{BASH_SOURCE[0]}" )" &> /dev/null && pwd )
chmod +x `+"$"+`{SCRIPT_DIR}/check-p1
rm "`+"$"+`{SCRIPT_DIR}/init.sh"
`
);

variable::set("day", d)