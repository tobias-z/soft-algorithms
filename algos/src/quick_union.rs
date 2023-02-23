struct UF {
    ids: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> Self {
        let mut ids: Vec<usize> = vec![];
        for i in 0..n {
            ids.push(i);
        }
        Self { ids }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find_root(p) == self.find_root(q)
    }

    fn find_root(&self, n: usize) -> usize {
        let val = self.ids[n];
        if val == n {
            return val;
        }
        self.find_root(val)
    }

    fn union(mut self, p: usize, q: usize) -> Self {
        let p_root = self.find_root(p);
        let q_root = self.find_root(q);
        self.ids[p_root] = q_root;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_determine_connected() {
        let uf = UF::new(8);
        assert!(uf.connected(1, 1));
        assert!(!uf.connected(1, 2));
    }

    #[test]
    fn can_union() {
        let uf = UF::new(8);
        let uf = uf.union(1, 5);
        let uf = uf.union(5, 6);
        let uf = uf.union(2, 5);
        assert!(uf.connected(2, 6));
    }
}
