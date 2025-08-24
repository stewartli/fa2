use std::path::PathBuf;

struct JobFile{
    name: String, 
    create_at: u64, 
    modify_at: u64, 
}

pub fn run(){
    let path_root = PathBuf::from(std::env::var("USER_FA_DIR").unwrap());
    let path_job = path_root.join("faproj/job");
    // get metadata
    let mut out: Vec<JobFile> = vec![];
    if let Ok(entry) = std::fs::read_dir(path_job){
        for ent in entry.flatten(){
            if let Ok(meta) = ent.metadata(){
                let path_ent = ent.path();
                let path_ent = path_ent.strip_prefix("/home/stli/").unwrap();
                let path_create = meta.created().unwrap()
                    .elapsed().unwrap()
                    .as_secs()
                    .checked_div(60 * 60 * 24)
                    .unwrap();
                let path_modify = meta.modified().unwrap()
                    .elapsed().unwrap()
                    .as_secs()
                    .checked_div(60 * 60 * 24)
                    .unwrap();
                let res = JobFile{
                    name: path_ent.to_str().unwrap().to_string(), 
                    create_at: path_create, 
                    modify_at: path_modify, 
                };
                out.push(res);
            }
        }
    }
    // print out 
    out.into_iter().for_each(|x| {
         let res_str = format!("\x1b[93m{:<10}\x1b[0m{:>10}{:>10}\n", x.name, x.create_at, x.modify_at);
        println!("{}", res_str);
    });
}
