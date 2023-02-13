struct UF {
    ids: Vec<usize>,
    id_sizes: Vec<usize>,
}

// can be used for things like perculation
impl UF {
    fn new(n: usize) -> Self {
        let mut ids: Vec<usize> = vec![];
        for i in 0..n {
            ids.push(i);
        }
        Self {
            ids,
            id_sizes: vec![1; n],
        }
    }

    fn connected(self: &mut Self, p: usize, q: usize) -> bool {
        self.find_root(p) == self.find_root(q)
    }

    fn find_root(self: &mut Self, n: usize) -> usize {
        let val = self.ids[n];
        if val == n {
            return val;
        }
        // flatten the tree so that the path becomes even smaller
        // you could also do a complete flattening of the tree
        self.ids[n] = self.ids[self.ids[n]];
        self.find_root(self.ids[n])
    }

    // Ensure that we have smaller trees by always linking the smaller tree to the bigger one
    fn union(mut self: Self, p: usize, q: usize) -> Self {
        let p_root = self.find_root(p);
        let q_root = self.find_root(q);
        if p_root == q_root {
            return self; // they are already linked
        }
        if self.id_sizes[p] >= self.id_sizes[q] {
            self.ids[q_root] = p_root;
            self.id_sizes[p_root] += self.id_sizes[q_root];
        } else {
            self.ids[q_root] = p_root;
            self.id_sizes[q_root] += self.id_sizes[p_root];
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_determine_connected() {
        let mut uf = UF::new(8);
        assert!(uf.connected(1, 1));
        assert!(!uf.connected(1, 2));
    }

    #[test]
    fn can_union() {
        let uf = UF::new(8);
        let uf = uf.union(1, 5);
        let uf = uf.union(5, 6);
        let mut uf = uf.union(2, 5);
        assert!(uf.connected(2, 6));
    }
}
