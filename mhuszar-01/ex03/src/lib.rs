/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 15:08:16 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 17:44:46 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32]
{
    if haystack.is_empty() || needle.is_empty()
    {
        return &[];
    }
    let mut counter = 0;
    let mut ctr2 = 0;
    let mut ctr3;
    let mut len = 0;
    let mut idx = 0;
    let mut temp_len = 0;
    let mut temp_idx;
    let stack_max = haystack.len() - 1;
    let needle_max = needle.len() - 1;
    let mut sub_arr;
    let mut valid_array = false;
    while counter <= stack_max
    {
        if needle.contains(&haystack[counter])
        {
            temp_idx = counter;
            ctr3 = counter;
            while ctr3 <= stack_max && needle.contains(&haystack[ctr3])
            {
                temp_len += 1;
                ctr3 += 1;
            }
            sub_arr = &haystack[temp_idx..ctr3];
            while ctr2 <= needle_max && sub_arr.contains(&needle[ctr2])
            {
                ctr2 += 1;
                if ctr2 > needle_max
                {
                    valid_array = true;
                }
            }
            ctr2 = 0;
            if valid_array && temp_len > len
            {
                len = temp_len;
                idx = temp_idx;
                valid_array = false;
            }
            temp_len = 0;
        }
        counter += 1;
    }
    if len > 0
    {
        &haystack[idx..(idx + len)]
    }
    else
    {
        &[]
    }
}

#[test]
#[cfg(test)]
fn test_lifetimes()
{
    let haystack = [1, 2, 3, 2, 1];
    let result;
    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }
    assert_eq!(result, &[2, 3, 2]);
}
#[test]
#[cfg(test)]
fn test_largest_group()
{
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
}