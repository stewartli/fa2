use std::path::PathBuf;

struct JobFile{
    name: String, 
    create_at: u64, 
    modify_at: u64, 
}

impl JobFile{
    fn print_out(&self){
        let res_str = format!("\x1b[93m{:<10}\x1b[0m{:>10}{:>10}\n", self.name, self.create_at, self.modify_at);
        println!("{}", res_str);
    }
}

pub fn run(query: String, sort: bool, topn: Option<usize>){
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
    if let Some(n) = topn{
        if sort{
            out.sort_by(|a, b| a.modify_at.cmp(&b.modify_at));
            out.into_iter()
                .filter(|x| x.name.contains(&query))
                .take(n)
                .for_each(|x| x.print_out() );
        }else{
            out.into_iter()
                .filter(|x| x.name.contains(&query))
                .take(n)
                .for_each(|x| x.print_out() );
        }
    }else{
        if sort{
            out.sort_by(|a, b| a.modify_at.cmp(&b.modify_at));
            out.into_iter()
                .filter(|x| x.name.contains(&query))
                .for_each(|x| x.print_out() );
        }else{
            out.into_iter()
                .filter(|x| x.name.contains(&query))
                .for_each(|x| x.print_out() );
        }
    }
}
