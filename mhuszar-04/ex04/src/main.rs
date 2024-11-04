/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/11 20:48:08 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/11 23:28:36 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::env::args;
// use std::io::stdin;
use std::io::stdout;
use std::io::{Read, Write};
// use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio, Child};

fn main()
{
    let args: Vec<String> = args().collect();
    if args.len() == 1 { return ; }
    
    let mut split_vec: Vec<Vec<String>> = Vec::new();
    let mut sub_vec: Vec<String> = Vec::new();
    let mut child_vec: Vec<Child> = Vec::new();
    let mut ctr = 0;
    while ctr < args.len()
    {
        ctr += 1;
        if ctr == args.len() || args[ctr] == "," { split_vec.push(sub_vec.clone()); sub_vec.clear() }
        else { sub_vec.push(args[ctr].clone()); }
    }
    ctr = 0;
    while ctr < split_vec.len()
    {
        // println!("Iter!!");
        if split_vec[ctr].is_empty() { ctr += 1 ; continue ; }
        let mut cmd = Command::new(&split_vec[ctr][0]);
        if split_vec[ctr].len() > 1
        {
            cmd.args(&split_vec[ctr][1..]);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::null());
        }
        // let mut _discard = write!(stdout(), "\n========BEGIN\n\n");
        if let Ok(child) = cmd.spawn()
        {
            child_vec.push(child)
        }
        // _discard = write!(stdout(), "\n========END\n");
        ctr += 1;
        // com_vec.push(cmd);
    }
    let mut stdout = stdout().lock();
    for mut child in child_vec
    {
        // let _ = child.wait();
        let mut output = String::new();
        let mut _discard;
        if let Some(mut stdout) = child.stdout.take() { let _lol = stdout.read_to_string(&mut output); }
        _discard = write!(stdout, "\n\n========BEGIN\n\n\n");
        _discard = write!(stdout, "{}", output);
        _discard = write!(stdout, "\n\n========END\n\n");
        _discard = stdout.flush();
    }
//     for mut child in child_vec
//     {
//         match child.try_wait()
//         {
//             Ok(Some(_)) =>
//             {
//                 let mut _discard = writeln!(stdout(), "==========");
//                 if let Some(mut child_stdout) = child.stdout.take()
//                 {
//                     let mut output = String::new();
//                     let _discard = child_stdout.read_to_string(&mut output);
//                     let _ = write!(stdout(), "{}", output);
//                 }
//                 _discard = writeln!(stdout());
//                 _discard = stdout().flush();
//             },
//             Ok(None) => { continue ; },
//             _ => ()
//         }
//     }
}