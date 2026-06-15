#include <iostream>
using namespace std;

//class1: linked list class making use of node class
class LinkedList {

struct Node {
    int data;
    Node* next;

    Node(int value) {
        data = value;
        next = nullptr;
    }
};

private:
    // member variable
    Node* head;

public:
    // constructor
    LinkedList() {
        head = nullptr;
    }

    // fn1: Insert at beginning
    void insertFront(int value) {
        Node* newNode = new Node(value);
        newNode->next = head;
        head = newNode;
    }

    // fn2: Insert at end
    void insertBack(int value) {
        Node* newNode = new Node(value);

        if (head == nullptr) {
            head = newNode;
            return;
        }

        Node* temp = head;
        while (temp->next != nullptr) {
            temp = temp->next;
        }

        temp->next = newNode;
    }

    // fn3: Delete first occurrence of value
    void deleteValue(int value) {
        if (head == nullptr) {
            cout << "List is empty\n";
            return;
        }

        if (head->data == value) {
            Node* temp = head;
            head = head->next;
            delete temp;
            return;
        }

        Node* curr = head;

        while (curr->next != nullptr &&
               curr->next->data != value) {
            curr = curr->next;
        }

        if (curr->next == nullptr) {
            cout << "Value not found\n";
            return;
        }

        Node* temp = curr->next;
        curr->next = curr->next->next;
        delete temp;
    }

    // fn4: Search value
    bool search(int value) {
        Node* temp = head;

        while (temp != nullptr) {
            if (temp->data == value)
                return true;

            temp = temp->next;
        }

        return false;
    }

    // fn5: Length of list
    int length() {
        int count = 0;
        Node* temp = head;

        while (temp != nullptr) {
            count++;
            temp = temp->next;
        }

        return count;
    }

    // fn6: Display list
    void display() {
        Node* temp = head;

        while (temp != nullptr) {
            cout << temp->data << " -> ";
            temp = temp->next;
        }

        cout << "NULL\n";
    }

    // Destructor
    ~LinkedList() {
        while (head != nullptr) {
            Node* temp = head;
            head = head->next;
            delete temp;
        }
    }
};

//main function
int main() {
    LinkedList list;

    list.insertBack(10);
    list.insertBack(20);
    list.insertBack(30);

    cout << "Initial List:\n";
    list.display();

    list.insertFront(5);

    cout << "\nAfter inserting 5 at front:\n";
    list.display();

    list.deleteValue(20);

    cout << "\nAfter deleting 20:\n";
    list.display();

    cout << "\nLength: " << list.length() << endl;

    cout << "Search 30: "
         << (list.search(30) ? "Found" : "Not Found")
         << endl;

    return 0;
}