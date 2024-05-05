pub mod vecn;

/*
use smallvec::SmallVec;

// TODO: Upgrade this to a more specialized approach!
use zwohash::HashMap;

/// This is a reasonably-sized container for few particles, 
/// or a large container for many particles.
type CellContainer = SmallVec<[usize; 4]>;

/// Euclidean neighborhood query accelerator. Uses a hashmap grid.
pub struct QueryAccelerator<const Dims: usize> {
    cells: HashMap<[i32; Dims], CellContainer>,
    neighbors: Vec<[i32; Dims]>,
    radius: f32,
    radius_sq: f32,
}

impl<const Dims: usize> QueryAccelerator {
    /// Construct a new query accelerator
    pub fn new(points: &[VecN], radius: f32) -> Self {
        let mut cells: HashMap<[i32; 3], CellContainer> = HashMap::default();

        for (idx, &point) in points.iter().enumerate() {
            cells.entry(quantize(point, radius)).or_default().push(idx);
        }

        let neighbors = neighborhood::<3>();

        Self {
            cells,
            radius,
            radius_sq: radius * radius,
            neighbors,
        }
    }

    /*
    /// This should result in better cache locality for queries, but may take some time.
    pub fn sort_indices(mut self) -> Self {
        for indices in self.cells.values_mut() {
            indices.sort();
        }
        self
    }
    */

    // Query the neighbors of `queried_idx` in `points`
    pub fn query_neighbors<'s, 'p: 's>(
        &'s self,
        points: &'p [VecN],
        query_idx: usize,
        query_point: VecN,
    ) -> impl Iterator<Item = usize> + 's {
        let origin = quantize(query_point, self.radius);

        self.neighbors
            .iter()
            .filter_map(move |diff| {
                let key = add(origin, *diff);
                self.cells.get(&key).map(|cell_indices| {
                    cell_indices.iter().copied().filter(move |&idx| {
                        let dist = (points[idx] - query_point).length_squared();
                        idx != query_idx && dist <= self.radius_sq
                    })
                })
            })
            .flatten()
    }

    // Approximate neighbors; neighbors considered may be outside of the radius!
    pub fn query_neighbors_fast<'s, 'p: 's>(
        &'s self,
        query_idx: usize,
        query_point: VecN,
    ) -> impl Iterator<Item = usize> + 's {
        let origin = quantize(query_point, self.radius);

        self.neighbors
            .iter()
            .filter_map(move |diff| {
                let key = add(origin, *diff);
                self.cells.get(&key).map(|cell_indices| {
                    cell_indices
                        .iter()
                        .copied()
                        .filter(move |&idx| idx != query_idx)
                })
            })
            .flatten()
    }

    pub fn replace_point(&mut self, idx: usize, prev: VecN, current: VecN) {
        // TODO: Keep points in sorted order and use binary search! Or use hashsets for O(n)?
        // Find this point in our cells and remove it
        let prev_bins = self.cells.get_mut(&quantize(prev, self.radius)).unwrap();
        let prev_idx = prev_bins.iter().position(|v| *v == idx).unwrap();
        prev_bins.remove(prev_idx);

        // Add this point to its new cell
        self.cells
            .entry(quantize(current, self.radius))
            .or_default()
            .push(idx);
    }

    #[allow(unused)]
    pub fn stats(&self, name: &str) {
        println!("{} accel stats: ", name);
        let parts_per_cell =
            self.cells.iter().map(|c| c.1.len()).sum::<usize>() as f32 / self.cells.len() as f32;
        println!("Particles per cell: {}", parts_per_cell);
        let max = self.cells.iter().map(|c| c.1.len()).max().unwrap();
        println!("Max: {}", max);
        println!();
    }

    /*
    pub fn tiles(&self) -> impl Iterator<Item = (&[i32; 3], &Vec<usize>)> {
        self.cells.iter()
    }
    */
}

fn add(mut a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
    a
}

fn quantize(p: VecN, radius: f32) -> [i32; 3] {
    (*p.as_ref()).map(|v| (v / radius).floor() as i32)
}

fn neighborhood<const N: usize>() -> Vec<[i32; N]> {
    combos(-1, 1, 1)
}

fn combos<const Dims: usize>(min: i32, max: i32, step: i32) -> Vec<[i32; Dims]> {
    let mut dims = [min; N];
    let mut combos = vec![];
    loop {
        combos.push(dims);
        if dims == [max; N] {
            break combos;
        }
        for i in 0..dims.len() {
            if dims[i] < max {
                dims[i] += step;
                break;
            } else {
                dims[i] = min;
            }
        }
    }
}

*/
