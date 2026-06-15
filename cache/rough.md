- need to implement lru get(restructure), put(delete) -- both in O(1)
- with hashmap: get and put are O(1) but delete using counter is O(n)
- with stack and queue: restructure is O(n) on get
- with linked list: restructure is O(1) on get but get itself is O(n)

- so better is combination of hashmap and linked list -- hashmap for O(1) get and put, linked list for O(1) restructure and delete