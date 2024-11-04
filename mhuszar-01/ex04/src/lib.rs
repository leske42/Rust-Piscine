/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 16:51:21 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 18:55:43 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn sort_boxes(boxes: &mut [[u32; 2]])
{
    let mut ctr = 0;
    let mut sub = 1;
    if boxes.is_empty()
    {
        return ;
    }
    while sub < boxes.len()
    {
        while ctr < boxes.len() - sub
        {
            if boxes[ctr][0] <= boxes[ctr + 1][0] && boxes[ctr][1] <= boxes[ctr + 1][1]
            {
                boxes.swap(ctr, ctr + 1);
            }
            ctr += 1;
        }
        ctr = 0;
        sub += 1;
    }
    ctr = 0;
    while ctr < boxes.len() - 1
    {
        if boxes[ctr][0] >= boxes[ctr + 1][0] && boxes[ctr][1] >= boxes[ctr + 1][1]
        {
            ctr += 1;
        }
        else
        {
            panic!("wrong boxes!");
        }
    }
}

// fn main()
// {
//     let mut boxes = [[3, 3], [4, 3], [5, 5], [7, 7], [9, 9]];
//     sort_boxes(&mut boxes);
//     assert_eq!(boxes, [[9, 9], [7, 7], [5, 5], [4, 3], [3, 3]]);
// }

#[test]
#[cfg(test)]
fn test_sort_basic()
{
    let mut boxes = [[3, 3], [4, 3], [5, 5], [7, 7], [9, 9]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[9, 9], [7, 7], [5, 5], [4, 3], [3, 3]]);

    boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);

    let mut boxes2: [[u32; 2]; 0] = [];
    let empty: [[u32; 2]; 0] = [];
    sort_boxes(&mut boxes2);
    assert_eq!(boxes2, empty);
}

#[test]
#[cfg(test)]
#[should_panic]
fn panic_test()
{
    let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 2], [3, 3]];
    sort_boxes(&mut boxes);
}
