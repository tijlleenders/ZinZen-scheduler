use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main()-> Result<(),std::io::Error>{
    let path = Path::new("./tests/jsons");
    match visit_dirs(path, &process_file){
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(filename: &Path) {
    //create a second input.json file (input2.json)
    //that has been edited in some way
    //and store it in same directory as input.json
    let mut output_path = filename.to_owned();
    output_path.set_file_name("input2");
    output_path.set_extension("json");
    let file = File::create(&output_path).unwrap();
    let mut file = OpenOptions::new().append(true).open(output_path).unwrap();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                if l.contains("\"duration\":") {
                    let words = l.split(':').collect::<Vec<&str>>();
                    let trimmed = words[1].trim();
                    let mut duration = &trimmed[..trimmed.len() - 1];
                    let new_line = format!("{}: \"{}\",", words[0], duration);
                    writeln!(file, "{}", new_line).unwrap();
                } else {
                    writeln!(file, "{}", l).unwrap();
                }
            }
        }
    }
}

/*fn process_file(filename: &Path) {
    println!("processing {:?}", filename);
    //capture the outputs into a vec of outputs
    let file = File::open(filename).expect("Error reading file");
    let reader = BufReader::new(file);
    let outputs = serde_json::from_reader(reader).unwrap();
    //crate a finaloutput object with scheduled as the outputs and impossible as empty vec
    let final_ouput = FinalOutput {
        scheduled: outputs,
        impossible: Vec::new(),
    };
    //write the deserialized final_output to output2.json
    let json = serde_json::to_string_pretty(&final_ouput).unwrap();
    let mut output_path = filename.to_owned();
    output_path.set_file_name("output2");
    output_path.set_extension("json");
    fs::write(output_path.to_str().unwrap(), json).unwrap();
}*/

fn visit_dirs(dir: &Path, cb: &dyn Fn(&Path)) -> io::Result<()> {
    //visit each location in 'dir'
    //if the location is a file named 'output.json' run the callback function with that file as input
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                if entry.path().file_name().unwrap() == "input.json" {
                    cb(&entry.path());
                }
            }
        }
    }
    Ok(())
}
