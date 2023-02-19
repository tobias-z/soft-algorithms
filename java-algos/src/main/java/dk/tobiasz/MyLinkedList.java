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

    public void sort(Comparator<T> comparator) {
        NodeContainer<T> nodeContainer = sorted(new NodeContainer<>(head, tail), comparator);
        head = nodeContainer.head();
        tail = nodeContainer.tail();
    }

    private NodeContainer<T> mergeNodes(Node<T> node1, Node<T> node2, Comparator<T> comparator) {
        Node<T> sorted = new Node<>(null);
        Node<T> currentNode = sorted;

        while (node1 != null || node2 != null) {
            if (node1 == null) {
                currentNode.next = node2;
                currentNode.next.prev = currentNode;
                node2 = node2.next;
                currentNode = currentNode.next;
                continue;
            }
            if (node2 == null) {
                currentNode.next = node1;
                currentNode.next.prev = currentNode;
                node1 = node1.next;
                currentNode = currentNode.next;
                continue;
            }
            int compare = comparator.compare(node1.val, node2.val);
            if (compare >= 0) {
                currentNode.next = node2;
                currentNode.next.prev = currentNode;
                node2 = node2.next;
            } else {
                currentNode.next = node1;
                currentNode.next.prev = currentNode;
                node1 = node1.next;
            }
            currentNode = currentNode.next;
        }
        return new NodeContainer<>(sorted.next, currentNode);
    }

    private NodeContainer<T> sorted(NodeContainer<T> nodeContainer, Comparator<T> comparator) {
        Node<T> node = nodeContainer.head();
        if (node == null || node.next == null) {
            return nodeContainer;
        }

        Node<T> leftEnd = node;
        Node<T> rightStart = node;
        Node<T> rightEnd = node;

        while (rightEnd != null && rightEnd.next != null) {
            leftEnd = rightStart;
            rightStart = rightStart.next;
            rightEnd = rightEnd.next.next;
        }

        leftEnd.next = null;

        NodeContainer<T> leftSide = sorted(new NodeContainer<>(node, null), comparator);
        NodeContainer<T> rightSide = sorted(new NodeContainer<>(rightStart, null), comparator);

        return mergeNodes(leftSide.head(), rightSide.head(), comparator);
    }

}

record NodeContainer<T>(Node<T> head, Node<T> tail) {

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
