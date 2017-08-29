/*
    Store all nodes within a file.
        -> inside a subdirectory
       (->) If to many nodes exist use a secondary file.


    ( Node generation should be a secondary project, as they are too complex to hand write.)
    A node contains:
        -> name: &str
        -> connecting nodes
            -> distance to them
            -> ways of transportation with times.
       (->) absolute x and y positions.



    It will use multiple algorithms to find the shortest and fastest path using whatever
    transportation the user has requested. (any transportation by default)
-
    Djikstras, A-pointer
        (->) including alternative paths.

    The user will get a summery of the trip,
    The user can request a generated map of traveling between the nodes to the destination.

 */

extern crate pathfinder;


fn main() {
    /*
    Number of nodes = 2^number -1
    // TODO fix so that the number is better indicated.
    */

    match pathfinder::create_network(3, 200) {
        Ok(_) => println!("Created a node network successfully."),
            _ => panic!("TODO: proper error message here."),
    };
}


