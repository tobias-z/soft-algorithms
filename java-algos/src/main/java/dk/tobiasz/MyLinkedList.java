package dk.tobiasz;

import java.util.Comparator;

public class MyLinkedList<T> {

    Node<T> tail;
    Node<T> head;

    MyLinkedList() {
        head = null;
    }

    void printList() {
        Node<T> temp = head;
        while (temp != null) {
            System.out.print(temp.val + " ");
            temp = temp.next;
        }
    }

    // Upper O(n)
    // Lower O(1)
    public int find(T item) {
        int count = 0;
        Node<T> node = head;
        while (node != null) {
            if (node.val.equals(item)) {
                return count;
            }
            count++;
            node = node.next;
        }
        return -1;
    }

    // Upper O(n)
    // Lower O(1)
    public void insert(int index, T item) {
        int count = 0;
        Node<T> node = head;
        while (count <= index - 1) {
            if (count == index - 1) {
                Node<T> newNode = new Node<>(item);
                Node<T> oldNext = node.next;
                node.next = newNode;
                newNode.next = oldNext;
                newNode.prev = node;
                if (newNode.next.next == null) {
                    tail = newNode.next;
                }
                break;
            } else {
                node = node.next;
                count++;
            }
        }
    }

    // Upper O(n)
    // Lower O(1)
    public void remove(int index) {
        int count = 0;
        Node<T> node = head;
        while (count <= index - 1) {
            if (count == index - 1) {
                node.next = node.next.next;
                if (node.next == null) {
                    tail = node;
                } else {
                    node.next.prev = node;
                }
                break;
            } else {
                node = node.next;
                count++;
            }
        }
    }

    // Upper O(1)
    // Lower O(1)
    public boolean isEmpty() {
        return head == null;
    }

    // Upper O(1)
    // Lower O(1)
    // I'm pretty sure this is fine since there is nothing pointing to the other nodes
    // meaning java will take care of garbage collecting them
    public void clear() {
        head = null;
        tail = null;
    }

    // Upper O(n)
    // Lower O(1)
    public boolean contains(T item) {
        Node<T> node = head;
        while (node != null) {
            if (node.val.equals(item)) {
                return true;
            }
            node = node.next;
        }
        return false;
    }

    // Upper O(1)
    // Lower O(1)
    public T getFirst() {
        return head.val;
    }

    // Upper O(1)
    // Lower O(2)
    public T getLast() {
        return tail.val;
    }

    // Upper O(1)
    // Lower O(1)
    public void removeFirst() {
        head = head.next;
    }

    // Upper O(1)
    // Lower O(1)
    public void removeLast() {
        tail.prev.next = null;
        tail = tail.prev;
    }

    // Upper O(1)
    // Lower O(1)
    public void addFirst(T item) {
        Node<T> prevHead = head;
        Node<T> newHead = new Node<>(item);
        head = newHead;
        newHead.next = prevHead;
        if (prevHead != null) {
            prevHead.prev = newHead;
        }
        if (tail == null) {
            tail = head;
        }
    }

    // Upper O(1)
    // Lower O(1)
    public void addLast(T item) {
        Node<T> prevTail = tail;
        Node<T> newTail = new Node<>(item);
        tail = newTail;
        newTail.prev = prevTail;
        if (prevTail != null) {
            prevTail.next = newTail;
        }
        if (head == null) {
            head = tail;
        }
    }

    // Upper O(n)
    // Lower O(n)
    public void reverse() {
        Node<T> node = head;
        Node<T> last = null;
        while (node != null) {
            Node<T> next = node.next;
            node.next = last;
            node.prev = node;
            last = node;
            node = next;
        }
        Node<T> newTail = head;
        head = last;
        tail = newTail;
    }

    /**
     * Upper O(n)
     * Lower O(1)
     *
     * @param list1 sorted list
     * @param list2 sorted list
     * @return merged list
     */
    public static <T> MyLinkedList<T> mergeSortedLinkedList(
        MyLinkedList<T> list1,
        MyLinkedList<T> list2,
        Comparator<T> comparator
    ) {
        MyLinkedList<T> result = new MyLinkedList<>();
        boolean hasNext = true;
        Node<T> next1 = list1.head;
        Node<T> next2 = list2.head;
        while (hasNext) {
            if (next1 == null && next2 == null) {
                hasNext = false;
                continue;
            }
            if (next1 == null) {
                result.addLast(next2.val);
                next2 = next2.next;
                continue;
            }
            if (next2 == null) {
                result.addLast(next1.val);
                next1 = next1.next;
                continue;
            }
            int compare = comparator.compare(next1.val, next2.val);
            if (compare >= 0) {
                result.addLast(next2.val);
                next2 = next2.next;
            } else {
                result.addLast(next1.val);
                next1 = next1.next;
            }
        }
        return result;
    }

}

class Node<T> {

    T val;
    Node<T> next;
    Node<T> prev;

    Node(T val) {
        this.val = val;
        next = null;
    }
}
