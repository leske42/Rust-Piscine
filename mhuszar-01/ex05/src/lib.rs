/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 19:10:20 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 19:21:13 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn deduplicate(list: &mut Vec<i32>)
{
    let mut ctr = 0;
    let mut ctr2;
    if list.is_empty()
    {
        return ;
    }
    while ctr < list.len() - 1
    {
        ctr2 = ctr + 1;
        while ctr2 < list.len()
        {
            if list[ctr2] == list[ctr]
            {
                list.remove(ctr2);
                ctr2 = ctr;
            }
            ctr2 += 1;
        }
        ctr += 1;
    }
}

#[test]
#[cfg(test)]
fn test_duplicate()
{
    let mut v = vec![1, 2, 2, 3, 2, 4, 3];
    deduplicate(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}