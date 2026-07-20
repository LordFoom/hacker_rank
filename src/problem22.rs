
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
    let mut is_first_step = true;
    while !second_pointer.is_null() {
        let second_pointer=second_pointer.next;
        if is_first_step {
            is_first_step=false;
            continue;
        }
        let first_pointer=first_pointer.next;
        //we cut out the duplicates by pointing away from them
        while !second_pointer.is_null && first_pointer.data == second_pointer.data {
            second_pointer = second_pointer.next;
        }

    }

}
