package dk.tobiasz;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;

import java.util.ArrayList;
import java.util.Arrays;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class MyArrayListTest {

    @Test
    void myArrayList() throws Exception {
        MyArrayList<Integer> myArray = new MyArrayList<>();
        myArray.add(1);
        myArray.add(2);
        myArray.add(3);
        myArray.add(4);
        myArray.add(5);

        System.out.println("Array after adding items: " + myArray);

        boolean isDeleted = myArray.delete(3);
        if (isDeleted) {
            System.out.println("Deleted item 3 successfully");
        } else {
            System.out.println("Item 3 not found in array");
        }

        boolean isFound = myArray.search(4);
        if (isFound) {
            System.out.println("Found item 4 in array");
        } else {
            System.out.println("Item 4 not found in array");
        }
    }

    @Test
    @DisplayName("insert")
    void insert() throws Exception {
        MyArrayList<Integer> list = new MyArrayList<>();
        list.add(1);
        list.add(5);
        list.insert(1, 3);
        Integer integer = list.get(1);
        assertEquals(3, integer);
    }

    @Test
    @DisplayName("sort")
    void sort() throws Exception {
        MyArrayList<Integer> list = new MyArrayList<>();
        list.add(1);
        list.add(1);
        list.add(5);
        list.add(3);
        list.add(2);
        list.sort((o1, o2) -> {
            return o1 - o2;
        });
        System.out.println(Arrays.toString(list.toArray()));
    }

    @Test
    @DisplayName("reverse")
    void reverse() throws Exception {
        MyArrayList<Integer> list = new MyArrayList<>();
        list.add(1);
        list.add(1);
        list.add(5);
        list.add(3);
        list.add(2);
        list.add(3);
        list.reverse();
    }

    @Test
    @DisplayName("shuffle")
    void shuffle() throws Exception {
        MyArrayList<Integer> list = new MyArrayList<>();
        list.add(1);
        list.add(1);
        list.add(5);
        list.add(3);
        list.add(2);
        list.add(3);
        list.shuffle();
        System.out.println(Arrays.toString(list.toArray()));
    }

}