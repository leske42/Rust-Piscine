/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/11 15:26:13 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/11 22:07:52 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fs;
use std::path::Path;
// use std::fs::{Metadata, DirEntry, ReadDir};

fn size_counter(argv1 : String) -> u64
{
    let mut result = 0;
    let path = Path::new(&argv1);
    if let Ok(unwrapped) = path.read_dir()
    {
        for entry in unwrapped.flatten()
        {
            if let Ok(data) = entry.metadata()
            {
                if data.is_dir()
                {
                    if let Some(hehe) = entry.path().to_str()
                    {
                        result += size_counter(hehe.to_string());
                    }
                }
                else if data.is_file()
                {
                    result += data.len();
                }
            }
        }
    }
    else if let Err(e) = path.read_dir()
    {
        if e.to_string().contains("Not a directory")
        {
            if let Ok(meta) = fs::metadata(argv1)
            {
                result = meta.len()
            }
        }
    }
    result
}

fn display_size(size : u64)
{
    let mut magnitude = 0;
    let mut div : f64 = 1.0;
    let mut dividend = size;
    while dividend > 0
    {
        dividend /= 1000;
        magnitude += 1;
    }
    for _n in 0..(magnitude - 1)
    {
        div *= 1000.0;
    }
    match magnitude
    {
        1 => std::println!("{} bytes", size),
        2 => std::println!("{:.1} kilobytes", size as f64 / div),
        3 => std::println!("{:.1} megabytes", size as f64 / div),
        4 => std::println!("{:.1} gigabytes", size as f64 / div),
        _ => std::println!("what"),
    }
}

fn main()
{
    if let Some(argv1) = std::env::args().nth(1)
    {
        display_size(size_counter(argv1.clone()));
        // std::println!("Size: {}", cur_size);
    }
}

// #[test]
// #[cfg(test)]
// fn nothing_test()
// {
//     "1 megabytes");
// }