/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/09 20:19:35 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/09 20:40:40 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(std::clone::Clone, std::cmp::PartialOrd, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
struct MyType {
    
}

fn main() {
    let instance = MyType::default();

    let other_instance = instance.clone();

    println!("the default value of MyType is {instance:?}");
    println!("the clone of `instance` is {other_instance:#?}");
    assert_eq!(
        instance,
        other_instance,
        "the clone isn't the same :/"
    );
    assert!(
        instance == other_instance,
        "why would the clone be less or greater than the original?",
    );
}