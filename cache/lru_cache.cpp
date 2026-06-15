#include <iostream>
#include <unordered_map>

class Node {
public:    
    int data_key;
    int data_val;
    Node* next;
    Node* prev;

    Node(int key, int val){
        data_key = key;
        data_val = val;
        next = nullptr;
        prev = nullptr;
    }
};

class LRUCache {
public:
    int lru_capacity;
    Node* head; //most recent used
    Node* tail; //least recent used
    std::unordered_map<int, Node*> mp;

    LRUCache(int capacity) {
        lru_capacity = capacity;
        head = new Node(-1, -1);
        tail = new Node(-1, -1);
        head->next = tail;
        tail->prev = head;
    }

    int get(int key) {
        if (mp.find(key) != mp.end()) {
            Node* curr = mp[key];

            curr->prev->next = curr->next;
            curr->next->prev = curr->prev;

            curr->next = head->next;
            curr->prev = head;

            head->next->prev = curr;
            head->next = curr;

            return curr->data_val;
        } else {
            return -1;
        }
    }
    
    void put(int key, int value) {            
        if (mp.find(key) != mp.end()) {
            Node* curr = mp[key];
            curr->data_val = value;

            curr->prev->next = curr->next;
            curr->next->prev = curr->prev;

            curr->next = head->next;
            curr->prev = head;

            head->next->prev = curr;
            head->next = curr;

        } else {

            Node* curr = new Node(key, value);
            
            if (mp.size() < lru_capacity) {
                curr->next = head->next;
                curr->prev = head;

                head->next->prev = curr;
                head->next = curr;

            } else {
                Node* to_delete = tail->prev;
                mp.erase(to_delete->data_key);
                tail->prev->prev->next = tail;
                tail->prev = tail->prev->prev;
                
                curr->next = head->next;
                curr->prev = head;

                head->next->prev = curr;
                head->next = curr;
                delete to_delete;
            }
            mp[key] = curr;
        }
    }
};

int main() {

    // // Stack allocation: object lives on stack, access with '.', cleaned up automatically
    // LRUCache cache(2);
    // cache.put(1,1); //[1]
    // cache.put(2,2); //[2,1]
    // std::cout << cache.get(1) << std::endl; //[1,2] --> 1
    // std::cout << cache.get(3) << std::endl; //[1,2] --> -1
    // cache.put(3,3); //[3,1]
    // std::cout << cache.get(2) << std::endl; //[3,1] --> -1

    // Heap allocation: object lives on heap, access with '->', must call delete manually
    LRUCache* cache = new LRUCache(2); //dont miss * here
    cache->put(1,1); //[1]
    cache->put(2,2); //[2,1]
    std::cout << cache->get(1) << std::endl; //[1,2] --> 1
    std::cout << cache->get(3) << std::endl; //[1,2] --> -1
    cache->put(3,3); //[3,1]
    std::cout << cache->get(2) << std::endl; //[3,1] --> -1
}