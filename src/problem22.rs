
/*
 * Complete the 'deleteDuplicates' function below.
 *
 * The function is expected to return an INTEGER_SINGLY_LINKED_LIST.
 * The function accepts INTEGER_SINGLY_LINKED_LIST head as parameter.
 */

 SinglyLinkedListNode {
     data: i32,
     next: *mut SinglyLinkedListNode,
 };

fn deleteDuplicates(head: *const SinglyLinkedListNode) -> *const SinglyLinkedListNode {
    // Write your code here
    let mut i = 0;
    let mut first_pointer = head;
    let mut second_pointer = head;
    while !second_pointer.is_null() {

    }

}
