package dk.tobiasz;

import static org.junit.jupiter.api.Assertions.fail;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class MyLinkedListTest {

    @Test
    @DisplayName("find")
    void find() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        int i = list.find(2);
        System.out.println(i);
    }

    @Test
    @DisplayName("insert")
    void insert() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.insert(2, 10);
        list.printList();
    }

    @Test
    @DisplayName("remove")
    void remove() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.remove(2);
        list.printList();
    }

    @Test
    @DisplayName("contains")
    void contains() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        System.out.println(list.contains(2));
    }

    @Test
    @DisplayName("first and last")
    void firstAndLast() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        System.out.println(list.getFirst());
        System.out.println(list.getLast());
    }

    @Test
    @DisplayName("removeFirst")
    void removeFirst() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.removeFirst();
        list.printList();
    }

    @Test
    @DisplayName("removeLast")
    void removeLast() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.removeLast();
        list.printList();
    }

    @Test
    @DisplayName("addFirst")
    void addFirst() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.addFirst(5);
        list.printList();
    }

    @Test
    @DisplayName("addLast")
    void addLast() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.addLast(17);
        list.printList();
    }

    @Test
    @DisplayName("reverse")
    void reverse() throws Exception {
        MyLinkedList<Integer> list = new MyLinkedList<>();
        list.addLast(1);
        list.addLast(3);
        list.addLast(2);
        list.addLast(1);
        list.addLast(17);
        list.reverse();
        list.printList();

        System.out.println("\n");
        System.out.println(list.getFirst());
        System.out.println(list.getLast());
    }

}