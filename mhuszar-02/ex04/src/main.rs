/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/09 20:46:24 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/09 23:25:30 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

enum Command {
    Todo(String),   // Command: "TODO"
    Done(usize),    // Command: "DONE"
    Purge,          // Command: "PURGE"
    Quit,           // Command: "QUIT"
}

impl Command {
    fn prompt() -> Self
    {
        let mut hihi;
        let mut transf;
        loop
        {
            hihi = ftkit::read_line();
            transf = hihi.as_str();
            if transf.is_empty()
            {
                return Command::Quit;
            }
            if let Some(tmp) = transf.strip_prefix("TODO ")
            {
                let other = tmp.trim();
                match other
                {
                    "" => continue ,
                    _ =>  return Command::Todo(other.to_string()),
                }
            }
            else if let Some(tmp) = transf.strip_prefix("DONE ")
            {
                let other = tmp.trim();
                match other
                {
                    "" => continue ,
                    _ =>  {
                        if let Result::Ok(res) = other.parse::<usize>() { return Command::Done(res) }
                        else { continue ; }
                    }
                }
            }
            let other = transf.trim();
            match other
            {
                "PURGE" => return Command::Purge,
                "QUIT" => return Command::Quit,
                _ => continue
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self
    {
        TodoList
        {
            dones: Vec::new(),
            todos: Vec::new(),
        }
    }
    fn display(&self)
    {
        let mut ctr = 0;
        while ctr < self.todos.len()
        {
            std::println!("{ctr} [ ] {}", self.todos[ctr]);
            ctr += 1;
        }
        ctr = 0;
        while ctr < self.dones.len()
        {
            std::println!("[x] {}", self.dones[ctr]);
            ctr += 1;
        }
    }
    fn add(&mut self, todo: String)
    {
        self.todos.push(todo);
    }
    fn done(&mut self, index: usize)
    {
        if index < self.todos.len()
        {
            let hehe = self.todos[index].as_str();
            self.dones.push(hehe.to_string()); //mi a fasssz
            self.todos.remove(index);
        }
    }
    fn purge(&mut self)
    {
        self.dones.clear();
    }
}

fn main()
{
    let mut hehe = TodoList::new();
    loop
    {
        std::println!();
        hehe.display();
        std::println!();
        match Command::prompt()
        {
            Command::Todo(x) => TodoList::add(&mut hehe, x),
            Command::Done(y) => TodoList::done(&mut hehe, y),
            Command::Purge => TodoList::purge(&mut hehe),
            Command::Quit => return
        }
    }
}