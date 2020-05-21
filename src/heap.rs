pub struct Heap<'a, T> {
    data: Vec<Option<(f64, &'a T)>>,
    capacity: usize,
}

impl<'a, T> Heap<'a, T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![None],
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - 1
    }

    pub fn insert(&mut self, f: f64, item: &'a T) {
        if self.len() == 0 {
            self.data.push(Some((f, item)));
            return;
        } else if self.len() < self.capacity {
            self.data.push(Some((f, item)));
            self.heapify();
            return;
        }
        match self.get_max() {
            Some(m) => {
                if m > f {
                    self.extract_max();
                    self.data.push(Some((f, item)));
                    self.heapify();
                }
            }
            None => (),
        }
    }

    pub fn get_elements(&self) -> Vec<(f64, &T)> {
        // let mut sorted = self.data[1..].to_vec().clone();
        let mut sorted = Vec::new();
        for i in 1..self.len() + 1 {
            sorted.push(self.data[i].unwrap())
        }
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        sorted
    }

    pub fn get_max(&self) -> Option<f64> {
        if self.len() == 0 {
            None
        } else {
            Some(self.data[1].unwrap().0)
        }
    }

    fn at_idx(&self, idx: usize) -> f64 {
        self.data[idx].unwrap().0
    }

    fn heapify(&mut self) {
        let parent = |x: usize| -> usize { x / 2 as usize };
        let mut l = self.data.len() - 1;
        let mut p = parent(l);
        while p > 0 && self.at_idx(p) < self.at_idx(l) {
            self.data.swap(l, p);
            l = p;
            p = parent(l);
        }
    }
    fn extract_max(&mut self) -> Option<f64> {
        let m = self.get_max();
        if self.data.len() <= 2 {
            self.data.pop();
            return m;
        }

        // send the last element to the top
        if let Some(x) = self.data.pop() {
            self.data[1] = x;
        }
        // now rebalance
        let mut idx = 1;
        let mut child = idx * 2;
        while (child < self.len() && self.at_idx(idx) < self.at_idx(child))
            || (child + 1 < self.len() && self.at_idx(idx) < self.at_idx(child + 1))
        {
            if (child + 1 < self.len()) && (self.at_idx(child + 1) > self.at_idx(child)) {
                child = child + 1;
            }
            self.data.swap(idx, child);
            idx = child;
            child = idx * 2;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut h = Heap::new(10);
        for i in 0..6 {
            h.insert(i as f64, &0);
        }

        assert_eq!(h.len(), 6);
    }

    #[test]
    fn test_get_elements() {
        let mut h = Heap::new(10);
        h.insert(7.8, &0);
        h.insert(98.78, &0);
        h.insert(0.0, &0);
        h.insert(1.0, &0);

        assert_eq!(
            h.get_elements(),
            vec![(0.0, &0), (1.0, &0), (7.8, &0), (98.78, &0)]
        );
    }

    #[test]
    fn test_extract_max() {
        let mut h = Heap::new(10);

        h.insert(42.0, &0);
        assert_eq!(h.len(), 1);
        match h.extract_max() {
            Some(x) => assert_eq!(x, 42.0),
            None => panic!(),
        }
        assert_eq!(h.len(), 0);

        let v = vec![69.42, 34.26, 72.53, 14.69, 29.24, 89.00, 1.72, 94.44, 30.46];
        for i in v {
            h.insert(i, &0);
        }

        assert_eq!(h.len(), 9);
        match h.extract_max() {
            Some(x) => assert_eq!(x, 94.44),
            None => panic!(),
        }
        assert_eq!(h.len(), 8);
    }

    #[test]
    fn test_get_max() {
        let mut h = Heap::new(10);
        let v = vec![
            69.42, 34.26, 72.53, 14.69, 29.24, 89.00, 1.72, 94.44, 30.46, 81.18,
        ];
        for i in v {
            h.insert(i, &0);
        }

        match h.get_max() {
            Some(x) => assert_eq!(x, 94.44),
            None => panic!(),
        }
    }
}
