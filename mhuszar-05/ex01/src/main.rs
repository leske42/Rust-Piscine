/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/12 14:53:43 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/12 19:13:39 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::thread;
use std::sync::{Arc,Mutex};
use std::io;

struct Logger<W>
{
    buffer: Box<[u8]>,
    writer: W,
}

impl<W> Logger<W>
{
    pub fn new(threshold: usize, writer: W) -> Self
    {
        Logger
        {
            buffer: vec![0u8; threshold].into_boxed_slice(),
            writer,
        }
    }
}

impl<W: io::Write> Logger<W>
{
    pub fn log(&mut self, message: &str) -> io::Result<()>
    {
        let mut ctr = 0;
        let bytes = message.as_bytes();
        while ctr < self.buffer.len() && self.buffer[ctr] != 0
        {
            ctr += 1;
        }
        let max = self.buffer.len() - 1;
        let mut ctr2 = 0;
        if message.len() >= max - ctr
        {
            let hehe = self.flush();
            let _ = self.writer.write_all(bytes);
            let _ = self.writer.write("\n".as_bytes());
            return hehe;
        }
        while ctr2 < bytes.len()
        {
            self.buffer[ctr] = bytes[ctr2];
            ctr += 1;
            ctr2 += 1;
        }
        self.buffer[ctr] = b'\n';
        Ok(())
    }
    pub fn flush(&mut self) -> io::Result<()>
    {
        let mut ctr = 0;
        while ctr < self.buffer.len() && self.buffer[ctr] != 0
        {
            ctr += 1;
        }
        let hehe = self.writer.write_all(&self.buffer[0..ctr]);
        ctr = 0;
        while ctr < self.buffer.len()
        {
            self.buffer[ctr] = 0;
            ctr += 1;
        }
        hehe
    }
}

fn main()
{
    let logger = Logger::new(1024, io::stdout());
    let logger = Arc::new(Mutex::new(logger));
    let mut handles = vec![];

    for id in 0..10 
    {
        let logger = Arc::clone(&logger);
        let handle = thread::spawn(move ||
        {
            for i in 0..10
            {
                // println!("hehe");
                let mut logger = logger.lock().unwrap();
                let _ = logger.log(&format!("hello {} from thread {}!", i, id));
                // let _hehe = logger.flush();
            }
        });
        handles.push(handle);
    }
    for idx in handles
    {
        idx.join().unwrap();
    }
    let mut logger = logger.lock().unwrap();
    let _hehe = logger.flush();
}

// #[cfg(test)]
// #[test]
// fn test_u32()
// {
//     // let array: [u8; 5] = [48, 49, 50, 51, 52];
//     let mut logger = Logger::new(6, io::stdout());
//     let _ = logger.log("123");
//     let _ = logger.log("4");
//     assert_eq!(0, 0);
// }