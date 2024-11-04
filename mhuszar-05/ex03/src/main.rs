/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/12 19:46:18 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/12 23:09:30 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::sync::mpsc::sync_channel;
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::io::BufRead;
use ftkit::ARGS;

fn main() -> std::io::Result<()>
{
    let argc = ARGS.len();
    if argc != 2 { return Ok(()) ; }
    let arg1 = &ARGS[1];
    
    let brainsize;
    // let mut zero = false;
    if let Ok(result) = arg1.parse::<usize>()
    {
        brainsize = result;
    }
    else { return Ok(()) ; }
    // if brainsize == 0 { brainsize = 1; zero = true; }
    // println!("{brainsize}");
    {
        let (sender, receiver) = sync_channel(brainsize/* - 1*/);
        let _thread = spawn(move ||
        {
            while let Ok(word) = receiver.recv()
            {
                if word == "secret STOP msg"
                {
                    break ;
                }
                println!("the philosopher is thinking about {}", word);
                sleep(Duration::from_secs(5));
            }
        });
    
        for line in std::io::stdin().lock().lines()
        {
            // println!("line: {}", line.unwrap());
            if let Ok(word) = line
            {
                if word.is_empty()
                {
                    break ;
                }
                if /*zero || */sender.try_send(word).is_err()
                {
                    println!("the philosopher's head is full");
                }
            }
            else { break ; }
        }
        
        if sender.try_send("secret STOP msg".to_string()).is_err()
        {
            return Ok(()) //i guess thread gets detached here
        }  
        // _ = thread.join();
    }

    Ok(())
}

