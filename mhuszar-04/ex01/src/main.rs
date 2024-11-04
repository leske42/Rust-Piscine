/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/11 13:19:57 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/11 14:23:39 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len();
    let mut input = String::new();
    let _discard_option = std::io::stdin().read_to_string(&mut input);
    let mut _discard2 = write!(std::io::stdout(), "{}", input);
    let mut ctr = 1;
    while ctr < argc
    {
        if let Ok(mut file) = File::create(args[ctr].clone())
        {
            if let Ok(_discard2) = write!(file, "{}", input)
            {
                println!("OK");
            }
            else
            {
                println!("ERROR");
            }
        }
        else
        {
            _discard2 = write!(std::io::stdout(), "Error opening {}", args[ctr]);
        }
        ctr += 1;
    }
}

