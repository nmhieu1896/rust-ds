mod algo;
mod fenwick_tree;
mod hash;
mod heap;
mod linked_list;
mod parser;
mod pointers;
mod smart_pointers;
mod sort;
mod try_async;

fn main() {
    // heap::_run();
    // hash::group_anagrams::_run();
    // hash::four_sum_2::_run();
    // hash::int_to_roman::_run();
    // bit_wise::_run();
    // fenwick_tree::_run();
    // pointers::_run();
    // pointers::mut_pointers::_run();

    // linked_list::singly::_run();
    // linked_list::rc_singly::_run();
    // linked_list::doubly::_run();
    // sort::qsort::_run()

    // binary_tree::_run();

    try_async::main();
}

// use std::{
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
// };

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         handle_connection(stream);
//     }
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     if request_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = "Nothing to see here!";
//         let length = contents.len();

//         let response = format!(
//             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//         );

//         stream.write_all(response.as_bytes()).unwrap();
//     } else {
//         // some other request
//     }
// }
