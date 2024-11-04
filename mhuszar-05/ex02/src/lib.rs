/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/12 17:30:40 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/12 21:36:54 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::thread_local;
use std::cell::Cell;
use std::marker::Copy;
use std::clone::Clone;

thread_local!
{
    static LAST_ERROR: Cell<Error> = const { Cell::new(Error::Success) };
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[allow(dead_code)]
enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteFail,
    ReadFail,
}

#[allow(dead_code)]
impl Error {
    
    fn last() -> Self
    {
        LAST_ERROR.get()
    }
    fn make_last(self)
    {
        LAST_ERROR.set(self)
    }
}

#[cfg(test)]
#[test]
fn test_errors()
{
    // let a : Error = Error::IsDirectory;
    // a.last();
    assert_eq!(Error::last(), Error::Success);
    Error::IsDirectory.make_last();
    assert_eq!(Error::last(), Error::IsDirectory);
}