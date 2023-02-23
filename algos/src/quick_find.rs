struct UF {
    ids: Vec<usize>,
}

impl UF {
    // O(N). The operations increase depending on the amount of elements in the list
    fn new(n: usize) -> Self {
        let mut ids: Vec<usize> = vec![];
        for i in 0..n {
            ids.push(i);
        }
        Self { ids }
    }

    // O(1) since we dont have to loop the array to find out of something is connected
    fn connected(&self, p: usize, q: usize) -> bool {
        self.ids[p] == self.ids[q]
    }

    // O(N). The operations increase depending on the amount of elements in the list
    fn union(mut self, p: usize, q: usize) -> Self {
        let p_val = self.ids[p];
        let q_val = self.ids[q];
        for i in 0..self.ids.len() {
            if self.ids[i] == p_val {
                self.ids[i] = q_val;
            }
        }
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
        assert!(uf.connected(1, 5));
    }
}
