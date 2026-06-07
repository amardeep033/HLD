use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ── hash helper ──────────────────────────────────────────────────────────────

fn hash(value: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

// ── ring ─────────────────────────────────────────────────────────────────────

struct ConsistentHashRing {
    // BTreeMap keeps ring positions sorted — gives us cheap clockwise lookup
    ring: BTreeMap<u64, String>, // position -> server name
    virtual_nodes: usize,
}

impl ConsistentHashRing {
    fn new(virtual_nodes: usize) -> Self {
        ConsistentHashRing {
            ring: BTreeMap::new(),
            virtual_nodes,
        }
    }

    // Each physical server gets `virtual_nodes` positions on the ring.
    // e.g. "server-A#0", "server-A#1", "server-A#2", ...
    fn add_server(&mut self, server: &str) {
        for i in 0..self.virtual_nodes {
            let vnode_key = format!("{}#{}", server, i);
            let pos = hash(&vnode_key);
            self.ring.insert(pos, server.to_string());
        }
        println!("+ added '{}' ({} virtual nodes)", server, self.virtual_nodes);
    }

    fn remove_server(&mut self, server: &str) {
        for i in 0..self.virtual_nodes {
            let vnode_key = format!("{}#{}", server, i);
            let pos = hash(&vnode_key);
            self.ring.remove(&pos);
        }
        println!("- removed '{}'", server);
    }

    // Walk clockwise from the key's position; wrap around if we reach the end.
    fn get_server(&self, key: &str) -> Option<&str> {
        if self.ring.is_empty() {
            return None;
        }
        let pos = hash(key);

        // range(pos..) gives all entries with position >= pos (clockwise)
        self.ring
            .range(pos..)
            .next()
            .or_else(|| self.ring.iter().next()) // wrap-around
            .map(|(_, server)| server.as_str())
    }
}

// ── debug helpers ────────────────────────────────────────────────────────────

fn print_ring(ring: &ConsistentHashRing) {
    println!("\n  [ring positions — sorted]");
    for (pos, server) in &ring.ring {
        println!("    {:>20} → {}", pos, server);
    }
}

fn print_key_mappings(ring: &ConsistentHashRing, keys: &[&str]) {
    println!("\n  [key positions → server]");
    for key in keys {
        let key_pos = hash(key);
        let server = ring.get_server(key).unwrap();
        let server_pos = ring
            .ring
            .range(key_pos..)
            .next()
            .or_else(|| ring.ring.iter().next())
            .map(|(pos, _)| *pos)
            .unwrap();
        println!(
            "    '{:<6}' pos={:>20}  →  {} (pos={:>20})",
            key, key_pos, server, server_pos
        );
    }
}

// ── demo ─────────────────────────────────────────────────────────────────────

fn main() {
    let keys = ["alice", "bob", "carol", "dave", "eve", "frank"];

    let mut ring = ConsistentHashRing::new(3);

    // ── initial cluster ──────────────────────────────────────────────────────
    println!("\n=== initial cluster ===");
    ring.add_server("server-A");
    ring.add_server("server-B");
    ring.add_server("server-C");

    print_ring(&ring);
    print_key_mappings(&ring, &keys);

    // ── add a server ─────────────────────────────────────────────────────────
    println!("\n=== add server-D ===");
    ring.add_server("server-D");

    print_ring(&ring);
    print_key_mappings(&ring, &keys);

    // ── remove a server ──────────────────────────────────────────────────────
    println!("\n=== remove server-B ===");
    ring.remove_server("server-B");

    print_ring(&ring);
    print_key_mappings(&ring, &keys);
}