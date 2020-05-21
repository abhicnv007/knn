pub struct Heap {
    data: Vec<f64>,
    capacity: usize,
}

impl Heap {
    pub fn new(capacity: usize) -> Heap {
        Heap {
            data: vec![0.0],
            capacity: capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - 1
    }

    pub fn insert(&mut self, f: f64) {
        if self.len() == 0 {
            self.data.push(f);
            return;
        }
        if self.len() < self.capacity {
            self.data.push(f);
            self.heapify();
            return;
        }
        if self.get_max() > f {
            self.extract_max();
            self.data.push(f);
            self.heapify();
        }
    }

    pub fn get_elements(&self) -> Vec<f64> {
        let mut sorted = self.data[1..].to_vec().clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    }

    pub fn get_max(&self) -> f64 {
        self.data[1]
    }

    fn heapify(&mut self) {
        let parent = |x: usize| -> usize { x / 2 as usize };
        let mut l = self.data.len() - 1;
        let mut p = parent(l);
        while p > 0 && self.data[p] < self.data[l] {
            self.data.swap(l, p);
            l = p;
            p = parent(l);
        }
    }
    fn extract_max(&mut self) -> f64 {
        let m = self.get_max();
        // send the last element to the top
        match self.data.pop() {
            Some(x) => self.data[1] = x,
            None => (),
        }
        // now rebalance
        let mut idx = 1;
        let mut child = idx * 2;
        while (child < self.capacity && self.data[idx] < self.data[child])
            || (child + 1 < self.capacity && self.data[idx] < self.data[child + 1])
        {
            if (child + 1 < self.capacity) && (self.data[child + 1] > self.data[child]) {
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
            h.insert(i as f64);
        }

        assert_eq!(h.len(), 6);
    }

    #[test]
    fn test_get_elements() {
        let mut h = Heap::new(10);
        h.insert(7.8);
        h.insert(98.78);
        h.insert(0.0);
        h.insert(1.0);

        assert_eq!(h.get_elements(), vec![0.0, 1.0, 7.8, 98.78]);
    }

    #[test]
    fn test_extract_max() {
        let mut h = Heap::new(10);
        let v = vec![
            69.42, 34.26, 72.53, 14.69, 29.24, 89.00, 1.72, 94.44, 30.46, 81.18,
        ];
        for i in v {
            h.insert(i);
        }

        assert_eq!(h.len(), 10);
        assert_eq!(h.extract_max(), 94.44);
        assert_eq!(h.len(), 9)
    }
    #[test]
    fn test_get_max() {
        let mut h = Heap::new(10);
        let v = vec![
            69.42, 34.26, 72.53, 14.69, 29.24, 89.00, 1.72, 94.44, 30.46, 81.18,
        ];
        for i in v {
            h.insert(i);
        }

        assert_eq!(h.get_max(), 94.44)
    }
}
