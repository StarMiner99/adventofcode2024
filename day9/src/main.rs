use std::fs;

#[derive(Clone)]
#[derive(Debug)]
enum Memory {
    Free(u32),
    Used{size: u32, id: usize}
}

#[derive(Debug)]
struct OutMemory{
    size: u32,
    id: usize
}

fn fill_free_space(mut mem_vec: Vec<Memory>) -> Vec<OutMemory>{

    let mut out_vec = vec![];

    let mut end_of_vec = false;
    //let mut i = 0;
    let mut rest_mem: Option<OutMemory> = None;
    'outer: while !end_of_vec {
        let current = mem_vec.remove(0);
        match current {
            Memory::Used{size, id} => {
                out_vec.push(OutMemory{size, id});
            },
            Memory::Free(len) => {
                let mut need_distribution = len;
                // check if we have rest memory to allocate
                if let Some(ref mut rest) = rest_mem {
                    let need = rest.size;
                    
                    if need > need_distribution {
                        rest.size = need - need_distribution;
                        out_vec.push(OutMemory{size: need_distribution, id: rest.id});
                        continue; // continue because we dont have more memory to distribute
                    } else {
                        out_vec.push(OutMemory{size: rest.size, id: rest.id});
                        need_distribution -= rest.size;
                        rest_mem = None;
                    }
                }

                while need_distribution != 0 {
                    // we still need to distribute memory 
                    let mut segment = Memory::Free(0);
                    while let Memory::Free(_) = segment {
                        let new_seg = mem_vec.pop();
                        if let None = new_seg { // we have to get out of the loop as soon as nothing is left
                            break 'outer;
                        }
                        segment = new_seg.unwrap();
                    }

                    // always true
                    if let Memory::Used { size, id } = segment {
                        if size > need_distribution {
                            rest_mem = Some(OutMemory { size: size - need_distribution, id });
                            out_vec.push(OutMemory { size: need_distribution, id });
                            need_distribution = 0;
                        } else {
                            out_vec.push(OutMemory { size, id });
                            need_distribution -= size;
                        }
                    } else {
                        // just in case...
                        panic!();
                    }
                }

            }
        }

        if mem_vec.len() == 0 {
            end_of_vec = true;
        }
    }

    if let Some(rest) = rest_mem {
        out_vec.push(OutMemory{size: rest.size, id: rest.id});
    }

    out_vec
}

fn fill_left(mut mem_vec: Vec<Memory>) -> Vec<Memory>{

    let mut used_ptr = mem_vec.len() - 1;

    while used_ptr > 0{
        let current_seg = mem_vec.remove(used_ptr);

        if let Memory::Used { size: size_target, .. } = current_seg {
            let mut index_insert = used_ptr;
            for (i, seg) in mem_vec[0..used_ptr].iter().enumerate() {
                if let Memory::Free(size) = seg {
                    if *size >= size_target {
                        index_insert = i;
                        break;
                    }
                }
            }
            mem_vec.insert(index_insert, current_seg);

            if index_insert == used_ptr {
                used_ptr -= 1;
            } else {
                let free_change: &mut Memory = mem_vec.get_mut(index_insert + 1).unwrap();
                if let Memory::Free(ref mut size) = free_change {
                    *size -= size_target;
                } else {
                    panic!()
                }

                mem_vec.insert(used_ptr + 1, Memory::Free(size_target));

            }
        } else {
            mem_vec.insert(used_ptr, current_seg);
            used_ptr -= 1;
        }

    }

    mem_vec

}

const INPUT_PATH: &str = "input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    // parse the input and store in form of a vec
    let mut mem_vec = vec![];
    for (i, c) in input.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let c_i: u32 = c.to_string().parse().unwrap();

        let new_mem;
        if i % 2 == 0 {
            new_mem = Memory::Used{size: c_i, id: i / 2};
        } else {
            new_mem = Memory::Free(c_i);
        }
        mem_vec.push(new_mem);
    }

    let out_vec = fill_free_space(mem_vec.clone()); // this code is very ugly, but it works i guess

    // calculate checksum

    let mut chksum = 0;
    let mut i = 0;
    for mem_seg in out_vec {
        for _ in 0..mem_seg.size {
            chksum += i * mem_seg.id;
            i += 1;
        }
    }

    println!("checksum: {}", chksum); // no way this somehow worked first try... (this does not work for the simple example... got very lucky probably)

    // now the code is prettier
    // move everything to the left if possible
    let mem_vec = fill_left(mem_vec);

    let mut chksum = 0;

    let mut i = 0;
    for mem_seg in mem_vec {
        let mem_size;
        let mut mem_id = 0;
        let include_id;
        match mem_seg {
            Memory::Free(size) => {
                mem_size = size;
                include_id = false;
            },
            Memory::Used { size, id } => {
                mem_size = size;
                mem_id = id;
                include_id = true;
            }
        }

        for _ in 0..mem_size {
            if include_id {
                chksum += i * mem_id;
            }
            i += 1;
        }
    }

    println!("checksum 2: {}", chksum);



}
