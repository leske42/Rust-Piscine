/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/11 19:13:46 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/11 20:46:46 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::env::args;
use std::io::stdin;
use std::io::Read;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main()
{
    let args: Vec<String> = args().collect();
    if args.len() == 1 { return ; }
    let mut cmd = Command::new(&args[1]);
    cmd.args(&args[2..]);
    let mut input = String::new();
    let _discard_option = stdin().read_to_string(&mut input);
    cmd.args(input.split_whitespace());
    cmd.exec();
}