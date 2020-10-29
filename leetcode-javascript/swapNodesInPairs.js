/// # 24 Swap Nodes in Pairs
/// Given a linked list, swap every two adjacent nodes and return its head.
/// You may not modify the values in the list's nodes. Only nodes itself may be changed.
///
/// Constraints:
/// - the number of nodes in the list is in the range [0, 100]
/// - 0 <= Node.val <= 100
///
/// ## Example 1
/// Input: head = [1, 2, 3, 4]
/// Output: [2, 1, 4, 3]
///
/// ## Example 2
/// Input: head = []
/// Output: []

/**
 * Definition for singly-linked list.
 */
function ListNode(val, next) {
    this.val = (val===undefined ? 0 : val)
    this.next = (next===undefined ? null : next)
}

// helper function that returns a List of ListNodes using the values in arr, which is an Array of numbers
function buildList(arr) {
    let head = new ListNode();
    let cur_node = head;
    for (n of arr) {
        let node = new ListNode(n);
        cur_node.next = node;
        cur_node = node;
    }
    return head;
}

function hasNext(node) {
    return !!(node && node.next);
}

// prints the value of each node to the console.log
function printList(head) {
    let cur_node = head;
    while (cur_node !== null) {
        console.log(cur_node.val);
        cur_node = cur_node.next;
    }
}

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
const swapPairs = function(head) {
    let cur_node = head;

    while (cur_node !== null) {
        // get next two nodes
        let n1 = hasNext(cur_node) ? cur_node.next : null;
        let n2 = hasNext(n1) ? n1.next : null;

        // swap the nodes
        if (n1 && n2) {
            n1.next = n2.next;
            n2.next = n1;
            cur_node.next = n2;
        }
        // advance cur_node by two nodes
        cur_node = cur_node.next;
        if (hasNext(cur_node)) cur_node = cur_node.next;
    }

    return head;
};

// const head = buildList([1,2,3,4]);
// const swapped = swapPairs(head);
// printList(swapped);

const head = buildList([1,2,3,4,5,6]);
const swapped = swapPairs(head);
printList(swapped);
