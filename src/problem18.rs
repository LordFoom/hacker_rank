/*
 *
 * One-Pass Removal of k-th Node from End

Given the head of a singly linked list and an integer k, remove the k-th node from the end in one traversal and return the new head. If k is invalid, return the original list.

Example

Input

head = [5, 6, 7, 8]
k = 3

Output

[6, 7, 8]

Explanation

The list has 4 nodes.
The k-th node from the end with k=3 is the 4th node from the end (value 5), which is the head. Removing it yields [6,7,8].


 * */
struct SinglyLinkedListNode {
    data: i32,
    next: *mut SinglyLinkedListNode,
}

fn removeKthNodeFromEnd(head: *const SinglyLinkedListNode, k: i32) -> *const SinglyLinkedListNode {
    let mut i = 0;
    let mut j = 0;

    let mut curr_node = head;
    let mut trailing_node = head;
    while !curr_node.is_null() {
        if j < k {
        } else {
            j += 1;
            curr_node = unsafe {
                (*curr_node).next;
            };
            if (curr_node.is_null()) {
                break;
            }
            trailing_node = unsafe {
                trailing_node.next;
            }
        }
    }
    trailing_node
}
