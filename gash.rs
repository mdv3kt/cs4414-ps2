//
// gash.rs
//
// Starting code for PS2
// Running on Rust 0.9
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//
#[feature(globs)];
extern mod extra;

use std::{io, run, os};
use std::io::buffered::BufferedReader;

use std::io::stdin;

use extra::getopts;
use std::path::*;
use std::libc;





struct Shell {
    pipeflag: bool,
    ioflag: bool,
    flag: bool,
    history: ~[~str],
    cmd_prompt: ~str,
    
}

impl Shell {
    fn new(prompt_str: &str, zerohis: &[~str]) -> Shell {
        Shell {
            pipeflag: false,
            ioflag: false,
            flag: false,
            history: zerohis.to_owned(),
            cmd_prompt: prompt_str.to_owned(),
            
            
        }
    }
    
    fn run(&mut self) {
        
        let mut stdin = BufferedReader::new(stdin());

        let mut in_chan = libc::STDIN_FILENO;
        let mut out_chan = libc::STDOUT_FILENO;


       loop {
            self.pipeflag = false;
            self.flag = false;
            self.ioflag = false;
            print(self.cmd_prompt);
            io::stdio::flush();
            
            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            
            
            //println!("{:s}", cmd_line)   command_line is owned and trimmed of leading/lagging white space
            //println!("{:s}", line)        line is the arguement passed in as is
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");
                   
            //println!("{:s}", program)     program is just the command without arguements
            if cmd_line.ends_with("&")
            {
                self.flag = true;
             
            }

            let cmd_line1 = cmd_line.clone();
            
            let cmd_line2 = cmd_line1.to_owned();

            let cmd_line3 = cmd_line2.slice_to(cmd_line2.len()-2);
            let mut argvAnd: ~[~str] =
            cmd_line3.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
            argvAnd.remove(0);


            let mut argvPipe: ~[&str];

            let mut cmd_array: ~[&str];

            if self.flag == true
            {
                cmd_array = cmd_line3.split_str(" ").collect();
                argvPipe = cmd_line3.split_str_iter('|').filter_map(|x| if x != "" {Some(x.to_owned())}else{None}).to_owned_vec();
            }
            else
            {
                cmd_array = cmd_line1.split_str(" ").collect();
                argvPipe = cmd_line1.split_str_iter('|').filter_map(|x| if x != "" {Some(x.to_owned())}else{None}).to_owned_vec();
            }

            let mut cmd_ac: ~[&str] = cmd_array.clone().to_owned();


            for i in range(0, cmd_ac.len()-1)
            {
                if cmd_ac[i] == ">"
                {
                    self.ioflag  = true;
                    //print(cmd_ac[i+1]);
                    out_chan = self.get_fd(cmd_ac.remove(i+1), "w");
                }
                if cmd_ac[i] == "<"
                {
                    self.ioflag = true;
                    //print(cmd_ac[i+1]);
                    in_chan = self.get_fd(cmd_ac.remove(i+1), "r");
                }

            }

            for i in range(0, cmd_ac.len()-1)
            {
                if cmd_ac[i] == ">"
                {
                    self.ioflag  = true;
                    cmd_ac.remove(i);
                }
                if cmd_ac[i] == "<"
                {
                    self.ioflag = true;
                    cmd_ac.remove(i);
                }


            }

            /*for i in range(0, cmd_ac.len()-1)
            {
                println(format!(" print if you are working: {:s}", cmd_ac[i]));
            }*/




            //println(format!("{:s}", program));


            //println("{:s}",cmd_line3);

            
            match program {
                ""      =>  { continue; },
                "exit"  =>  { return; },
                _       =>  { 
                            if self.cmd_exists(program)
                            {
                            if self.flag == true
                            {
                                self.flag = false;

                                if self.ioflag == true
                                {
                                    self.ioflag = false;

                                    let myproc = run::Process::new(program, argvAnd, run::ProcessOptions
                                    {
                                            env: None,
                                            dir: None,
                                            in_fd: Some(in_chan),
                                            out_fd: Some(out_chan),
                                            err_fd: Some(libc::STDERR_FILENO)

                                    });
                            
                           
                            //let i = myproc.clone().unwrap().get_id();
                            //println(format!("{}", i));
                                self.run();
                                myproc.unwrap().finish();
                                }

                                else
                                {
                                    let myproc = run::Process::new(program, argvAnd, run::ProcessOptions
                                    {
                                            env: None,
                                            dir: None,
                                            in_fd: None,
                                            out_fd: None,
                                            err_fd: None

                                    });
                                    self.run();
                                    myproc.unwrap().finish();
                                }

                            }

                            else
                            {
                            self.run_cmdline(cmd_line2);

                            }

                            }

                            else
                            {
                                println!("{:s}: command not found", program);
                            }    
                              

                                //self.run_cmdline(port.recv());
                                //self.run_cmdline(port1.recv());
                            }
                                

                               
                        }; 
                        
        } 
        
    }
    
 
    fn get_fd(&mut self, filepth: &str, mode: &str)-> libc::c_int
    {
        unsafe
        {
            let filename = filepth.to_c_str().unwrap();
            let filemode = mode.to_c_str().unwrap();
            return libc::fileno(libc::fopen(filename, filemode));
        }
    }
    

    

    fn cd(&mut self, new_path:&Path) // the change directory method
    {
        
        if new_path.exists() && new_path.is_dir()
        {
            os::change_dir(new_path);

        }
        else if !new_path.is_dir()
        {
            println(format!("cd: {}: is not a Directory", new_path.display()));
            
        }
        else
        {
            println(format!("cd: {}: is not a File", new_path.display()));
        }

    }

    fn run_cmdline(&mut self, cmd_line: &str) {
        
        let mut argv: ~[~str] =
            cmd_line.split(' ').filter_map(|x| if x != "" { Some(x.to_owned()) } else { None }).to_owned_vec();
            //argv is owned pointer to a vector of owned strings - each word gets its own place in the vector
        if argv.len() > 0 {
            self.history.push(cmd_line.to_owned());
            let program: ~str = argv.remove(0); //removes and returns the elements, shift vector left
            self.run_cmd(program, argv); 
        }
    }
    
    fn run_cmd(&mut self, program: &str, argv: &[~str]) {
        
                  

        
        let mut args: ~[~str] = argv.to_owned();
        match program
        {
            "cd"        =>  { 
                                if !argv.is_empty()
                                {   
                                    let mut my_str: ~str;
                                    my_str = args.remove(0);
                                    let file = Path::new(my_str);
                                    self.cd(&file);
                                }
                                else
                                {
                                    self.cd(&os::getcwd());
                                }
                            },


           "history"   =>  {
                                println("History:");
                                let mut i = 1;
                                for x in self.history.rev_iter()
                                {
                                    println(format!("{:d}:  {:s}", i, x.to_owned()));
                                    i = i + 1;
                                    
                                }
                           },

            _           =>  

                           {      
                                if self.cmd_exists(program) 
                                {

                                    run::process_status(program, argv); //spawns new process, waits for completion - must take vector string args for program
                                     
                                } 
                                else 
                                {
                                    println!("{:s}: command not found", program);
                                }
                            }
        }
        let cwd = os::getcwd();
        print(format!("{}: ", cwd.display()));
    
    }
    
    fn cmd_exists(&mut self, cmd_path: &str) -> bool {
        let ret = run::process_output("which", [cmd_path.to_owned()]); //
        //print(ret);
        return ret.expect("exit code error.").status.success();
    }
}

fn get_cmdline_from_args() -> Option<~str> {
    /* Begin processing program arguments and initiate the parameters. */
    let args = os::args();
    
    let opts = ~[
        getopts::optopt("c")
    ];
    
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_err_msg()) }
    };
    
    if matches.opt_present("c") {
        let cmd_str = match matches.opt_str("c") {
                                                Some(cmd_str) => {cmd_str.to_owned()}, 
                                                None => {~""}
                                              };
        return Some(cmd_str);
    } else {
        return None;
    }
}

fn main() {
    
    let opt_cmd_line = get_cmdline_from_args();
    let newhis = ~[];
    match opt_cmd_line {
        Some(cmd_line) => Shell::new("", newhis).run_cmdline(cmd_line),
        None           => Shell::new("gash > ", newhis).run()
    }
}
