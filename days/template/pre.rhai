let d = variable::get("project-name").to_chars().get(-1);


file::write("init.sh", `aoc download -d ${d}`);