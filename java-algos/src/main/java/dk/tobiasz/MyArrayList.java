package dk.tobiasz;

import java.util.Comparator;
import java.util.Random;
import java.util.function.Predicate;

public class MyArrayList<T> {
    private T[] data;
    private int size;

    public MyArrayList() {
        data = (T[]) new Object[1];
        size = 0;
    }

    // Upper O(n)
    // Lower 0(1)
    public void add(T item) {
        if (size == data.length) {
            resize(2 * data.length);
        }
        data[size++] = item;
    }

    // Upper O(n)
    // Lower O(1)
    public boolean delete(T item) {
        int index = find(item);
        if (index == -1) {
            return false;
        }
        for (int i = index; i < size - 1; i++) {
            data[i] = data[i + 1];
        }
        data[--size] = null;
        if (size > 0 && size == data.length / 4) {
            resize(data.length / 2);
        }
        return true;
    }

    // Upper O(n)
    // Lower O(1)
    public boolean search(T item) {
        return find(item) != -1;
    }

    // Upper O(n)
    // Lower O(1)
    private int find(T item) {
        for (int i = 0; i < size; i++) {
            if (data[i].equals(item)) {
                return i;
            }
        }
        return -1;
    }

    // Upper O(n)
    // Lower O(n)
    private void resize(int newSize) {
        T[] newData = (T[]) new Object[newSize];
        for (int i = 0; i < size; i++) {
            newData[i] = data[i];
        }
        data = newData;
    }

    // O(1)
    // O(1)
    public T get(int i) {
        return data[i];
    }

    // O(1)
    // O(1)
    public void set(int i, T item) {
        this.data[i] = item;
    }

    // Upper O(n)
    // Lower O(n)
    public void insert(int i, T item) {
        this.shift(i);
        this.data[i] = item;
    }

    private void shift(int beginIndex) {
        this.resize(data.length + 1);
        this.size++;
        for (int i = 0; i < beginIndex - 1; i++) {
            if (i > 0) {
                this.data[i] = this.data[i + 1];
            }
        }
    }

    // O(1)
    // O(1)
    public int size() {
        return this.size;
    }

    // Upper O(1)
    // Lower O(1)
    public boolean isEmpty() {
        return this.size == 0;
    }

    // Upper O(n)
    // Lower O(n)
    public T[] toArray() {
        T[] arr = (T[]) new Object[size];
        for (int i = 0; i < arr.length; i++) {
            arr[i] = data[i];
        }
        return arr;
    }

    // Upper O(n^2)
    // Lower O(n^2)
    public void sort(Comparator<T> comparator) {
        for (int i = 0; i < this.size; i++) {
            for (int j = 0; j < this.size - 1; j++) {
                T prev = this.data[j];
                int compare = comparator.compare(prev, this.data[j + 1]);
                if (compare > 0) {
                    this.data[j] = this.data[j + 1];
                    this.data[j + 1] = prev;
                }
            }
        }
    }

    // Upper O(n)
    // Lower O(n)
    // NOTICE: This could be O(1) if we simply pointed the tail to be the head
    public void reverse() {
        int count = 0;
        int middle = this.size / 2;
        for (int i = this.size - 1; i > middle; i--) {
            T front = this.data[count];
            this.data[count] = this.data[i];
            this.data[i] = front;
            count++;
        }
        if (!isOdd(this.size)) {
            T front = this.data[count];
            this.data[count] = this.data[middle];
            this.data[middle] = front;
        }
    }

    private boolean isOdd(int n) {
        return n % 2 != 0;
    }

    // Upper O(n)
    // Lower O(n)
    public void shuffle() {
        Random random = new Random();
        for (int i = 0; i < this.size; i++) {
            int j = random.nextInt(this.size - 2);
            T next = this.data[j];
            this.data[j] = this.data[i];
            this.data[i] = next;
        }
    }

    // Upper O(n log(n))
    // Lower O(n) ?? Because we may find the element as the root node
    public T binarySearch(Predicate<T> predicate, Comparator<T> comparator) {
        Node root = this.createBinaryTree(this.data, 0, this.size - 1);
        if (root == null) {
            return null;
        }
        return searchTree(root, predicate, comparator);
    }

    private Node createBinaryTree(T[] arr, int left, int right) {
        if (left > right) {
            return null;
        }
        int middle = left + (right - left) / 2;
        Node node = new Node(arr[middle]);
        node.left = createBinaryTree(arr, left, middle - 1);
        node.right = createBinaryTree(arr, middle + 1, right);
        return node;
    }

    private T searchTree(Node node, Predicate<T> predicate, Comparator<T> comparator) {
        if (predicate.test(node.val)) {
            return node.val;
        }
        if (node.left == null && node.right == null) {
            return null;
        }
        if (node.left == null) {
            return searchTree(node.right, predicate, comparator);
        }
        if (node.right == null) {
            return searchTree(node.left, predicate, comparator);
        }

        int compare = comparator.compare(node.left.val, node.right.val);
        if (compare > 0) {
            return searchTree(node.left, predicate, comparator);
        }
        return searchTree(node.right, predicate, comparator);
    }

    class Node {
        private final T val;
        private Node left;
        private Node right;

        Node(T val) {
            this.val = val;
        }
    }
}