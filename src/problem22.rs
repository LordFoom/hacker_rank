
/*
 * Complete the 'deleteDuplicates' function below.
 *
 * The function is expected to return an INTEGER_SINGLY_LINKED_LIST.
 * The function accepts INTEGER_SINGLY_LINKED_LIST head as parameter.
 */

 struct SinglyLinkedListNode {
     data: i32,
     next: *mut SinglyLinkedListNode,
 }

fn deleteDuplicates(head: *const SinglyLinkedListNode) -> *const SinglyLinkedListNode {
    let mut first_pointer: *mut SinglyLinkedListNode = head as *mut _;
    unsafe{
        while !first_pointer.is_null() && !(*first_pointer).next.is_null() {
            if (* (*first_pointer).next).data  == (*first_pointer).data {
                (*first_pointer).next=(*(*first_pointer).next).next;
                continue;
            }
            if (*first_pointer).next.is_null(){
                continue;
            }
            first_pointer=(*first_pointer).next;
        }
    }
    head

}
