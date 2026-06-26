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
    pub data: i32,
    pub next: *mut SinglyLinkedListNode,
}

fn removeKthNodeFromEnd(head: *const SinglyLinkedListNode, k: i32) -> *const SinglyLinkedListNode {
    // if k < 0 {
    //     return head;
    // }
    let mut j = 1;
    let mut curr_node = head as *mut SinglyLinkedListNode;
    let mut trailing_node = head as *mut SinglyLinkedListNode;
    let mut removed = false;
    while !curr_node.is_null() {
        if j <= k + 1 {
            j += 1;
            curr_node = unsafe { (*curr_node).next }
        } else {
            j += 1;
            curr_node = unsafe { (*curr_node).next };
            if curr_node.is_null() {
                unsafe {
                    let next_node = (*trailing_node).next;
                    (*trailing_node).next = (*next_node).next;
                }
                removed = true;
                break;
            }
            trailing_node = unsafe { (*trailing_node).next }
        }
    }
    if removed {
        // println!("Removed a node, returning head");
        return head;
    }
    if j == k + 2 {
        return unsafe { (*head).next };
    }
    head
}

// Helper: Build a linked list from a vec
fn build_list(values: &[i32]) -> *mut SinglyLinkedListNode {
    let mut head: *mut SinglyLinkedListNode = std::ptr::null_mut();
    let mut tail: *mut SinglyLinkedListNode = std::ptr::null_mut();

    for &val in values {
        let node = Box::into_raw(Box::new(SinglyLinkedListNode {
            data: val,
            next: std::ptr::null_mut(),
        }));
        if head.is_null() {
            head = node;
            tail = node;
        } else {
            unsafe {
                (*tail).next = node;
            }
            tail = node;
        }
    }
    head
}

// Helper: Convert list to vec for easy assertion
fn list_to_vec(head: *const SinglyLinkedListNode) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head;
    while !curr.is_null() {
        unsafe {
            result.push((*curr).data);
            curr = (*curr).next;
        }
    }
    result
}

// Helper: Free the list
fn free_list(mut head: *mut SinglyLinkedListNode) {
    while !head.is_null() {
        unsafe {
            let next = (*head).next;
            let _ = Box::from_raw(head);
            head = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_node_invalid_k() {
        // [5], k=1 -> 2nd-from-end doesn't exist -> unchanged [5]
        let head = build_list(&[5]);
        let result = removeKthNodeFromEnd(head as *const _, 1);
        assert_eq!(list_to_vec(result), vec![5]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_single_node_remove_last() {
        // [5], k=0 -> remove last -> []
        let head = build_list(&[5]);
        let result = removeKthNodeFromEnd(head as *const _, 0);
        assert_eq!(list_to_vec(result), vec![]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_two_nodes_remove_last() {
        // [1,2], k=0 -> [1]
        let head = build_list(&[1, 2]);
        let result = removeKthNodeFromEnd(head as *const _, 0);
        assert_eq!(list_to_vec(result), vec![1]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_two_nodes_remove_head() {
        // [1,2], k=1 -> remove 2nd-from-end (the head) -> [2]
        let head = build_list(&[1, 2]);
        let result = removeKthNodeFromEnd(head as *const _, 1);
        assert_eq!(list_to_vec(result), vec![2]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_remove_middle() {
        // [1,2,3,4,5], k=1 -> [1,2,3,5]
        let head = build_list(&[1, 2, 3, 4, 5]);
        let result = removeKthNodeFromEnd(head as *const _, 1);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_remove_middle_different() {
        // [1,2,3,4,5], k=2 -> [1,2,4,5]
        let head = build_list(&[1, 2, 3, 4, 5]);
        let result = removeKthNodeFromEnd(head as *const _, 2);
        assert_eq!(list_to_vec(result), vec![1, 2, 4, 5]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_official_example() {
        // [5,6,7,8], k=3 -> remove head -> [6,7,8]
        let head = build_list(&[5, 6, 7, 8]);
        let result = removeKthNodeFromEnd(head as *const _, 3);
        assert_eq!(list_to_vec(result), vec![6, 7, 8]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_remove_last_of_three() {
        // [1,2,3], k=0 -> [1,2]
        let head = build_list(&[1, 2, 3]);
        let result = removeKthNodeFromEnd(head as *const _, 0);
        assert_eq!(list_to_vec(result), vec![1, 2]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_invalid_k_negative() {
        // k < 0 -> unchanged
        let head = build_list(&[1, 2, 3]);
        let result = removeKthNodeFromEnd(head as *const _, -1);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_out_of_bounds_k() {
        // [1,2,3], k=5 -> unchanged
        let head = build_list(&[1, 2, 3]);
        let result = removeKthNodeFromEnd(head as *const _, 5);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
        free_list(result as *mut _);
    }

    #[test]
    fn test_k_equals_length_out_of_bounds() {
        // [1,2,3], k=3 -> out of bounds (max valid k is 2) -> unchanged
        let head = build_list(&[1, 2, 3]);
        let result = removeKthNodeFromEnd(head as *const _, 3);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
        free_list(result as *mut _);
    }
}
