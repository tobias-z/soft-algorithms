package dk.tobiasz;

public class MyLinkedList<T> {

    Node tail;
    Node head;

    MyLinkedList() {
        head = null;
    }

    void printList() {
        Node temp = head;
        while (temp != null) {
            System.out.print(temp.val + " ");
            temp = temp.next;
        }
    }

    // Upper O(n)
    // Lower O(1)
    public int find(T item) {
        int count = 0;
        Node node = head;
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
        Node node = head;
        while (count <= index - 1) {
            if (count == index - 1) {
                Node newNode = new Node(item);
                Node oldNext = node.next;
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
        Node node = head;
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
    }

    // Upper O(n)
    // Lower O(1)
    public boolean contains(T item) {
        Node node = head;
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
        Node prevHead = head;
        Node newHead = new Node(item);
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
        Node prevTail = tail;
        Node newTail = new Node(item);
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
        Node node = head;
        Node last = null;
        while (node != null) {
            Node next = node.next;
            node.next = last;
            node.prev = node;
            last = node;
            node = next;
        }
        Node newTail = head;
        head = last;
        tail = newTail;
    }

    class Node {

        T val;
        Node next;
        Node prev;

        Node(T val) {
            this.val = val;
            next = null;
        }
    }
}

