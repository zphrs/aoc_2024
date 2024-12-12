let d = variable::get("project-name").to_chars().get(-1);

file::write("init.sh", 
`aoc download -d ${d} --session-file ../.adventofcode.session
`
+ 
`SCRIPT_DIR=`+"$"+`( cd -- "$( dirname -- "`+"$"+`{BASH_SOURCE[0]}" )" &> /dev/null && pwd )
rm "`+"$"+`{SCRIPT_DIR}/init.sh"`
);