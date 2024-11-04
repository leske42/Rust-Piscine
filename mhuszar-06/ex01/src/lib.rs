/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/13 13:56:38 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/13 16:48:03 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::slice::from_raw_parts;
// use std::mem::transmute;

#[allow(dead_code)]
type GoldNugget = u16;

#[allow(dead_code)]
type Iron = u32;

#[allow(dead_code)]
type Mercure = u64;

#[allow(dead_code)]
struct PhilosopherStone;

#[allow(dead_code)]
impl PhilosopherStone
{
    fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2]
    {
        let mut arr : [GoldNugget; 2] = [0, 0];
        arr[1] = (iron >> 16) as u16;
        arr[0] = iron as u16;
        arr
    }
    
    fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4]
    {
        let mut arr : [GoldNugget; 4] = [0, 0, 0, 0];
        arr[3] = (mercure >> 48) as u16;
        arr[2] = (mercure >> 32) as u16;
        arr[1] = (mercure >> 16) as u16;
        arr[0] = mercure as u16;
        arr
    }

    fn transmute_gold(self, gold: u16) -> [GoldNugget; 1]
    {
        let mut arr : [GoldNugget; 1] = [0];
        arr[0] = gold;
        arr
    }
}

#[allow(dead_code)]
type Gold = [GoldNugget];

#[allow(dead_code)]
/// # Safety
/// This wasnt even my idea i copied it from the subject
unsafe trait Metal {}

unsafe impl Metal for u64 { }

unsafe impl Metal for u32 { }

unsafe impl Metal for u16 { }

#[allow(dead_code)]
impl PhilosopherStone
{
    fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold
    {
        //SAFETY:
        // this operation could be dangerous if "metal" could be u8. But i did
        // not implement the Metal trait for u8, therefore this can not be an issue
        unsafe
        {
            let hehe = metal as *const M;
            let lolll = hehe as *const u16;
            let hihi = from_raw_parts(lolll, size_of::<M>() / 2);
            hihi
        }
    }
}

#[cfg(test)]
#[test]
fn test_iron()
{
    let iron = 0x12345678;
    assert_eq!(PhilosopherStone.transmute_iron(iron), [0x5678, 0x1234]);
    let mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_mercure(mercure),
        [0xCDEF, 0x89AB, 0x4567, 0x0123],
    );
}

#[cfg(test)]
#[test]
fn test_metal()
{
    let mercure: Mercure = 0x0123456789ABCDEF;
    assert_eq!(
        PhilosopherStone.transmute_metal(&mercure),
        &[0xCDEF, 0x89AB, 0x4567, 0x0123],
    );
}

