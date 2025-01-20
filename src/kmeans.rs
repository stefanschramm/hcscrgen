use std::u32;

pub trait KmeansContext<T> {
    /// Initialize a centroid for a cluster
    fn initialize_centroid(&self, k: usize) -> T;

    /// Determine centroid for cluster by set of elements
    fn determine_centroid(&self, elements: &Vec<&T>) -> T;

    /// Calculate difference between two elements
    ///
    /// 0 means the elements are equal
    fn diff(&self, a: &T, b: &T) -> u32;

    /// Called for each iteration
    ///
    /// If true is returned it means a stop criteria was reached and the main optimization loop should exit.
    fn iteration_callback(&self, i: u32, clusters: &Vec<Cluster<T>>) -> bool;
}

pub struct Assignment {
    pub index: usize,
    diff: u32,
}

pub struct Cluster<T> {
    pub centroid: T,
    pub elements: Vec<Assignment>,
}

struct Diff {
    index: usize,
    diff: u32,
}

pub fn kmeans<T>(
    context: &impl KmeansContext<T>,
    k: usize,
    elements: &Vec<T>,
    max_iterations: u32,
    fill_empty_clusters_every_nth_iteration: u32,
) -> Vec<Cluster<T>> {
    let mut clusters = initialize_clusters(context, k);
    for i in 0..max_iterations {
        reset_cluster_assignments(&mut clusters);
        reassign_elements(context, &mut clusters, &elements);
        recalculate_centroids(context, &mut clusters, &elements);
        if i % fill_empty_clusters_every_nth_iteration == 0 {
            if fill_empty_clusters(context, &mut clusters, &elements) {
                // perfect approximation reached
                return clusters;
            }
        }
        if context.iteration_callback(i, &clusters) {
            return clusters;
        };
    }
    clusters
}

fn initialize_clusters<T>(context: &impl KmeansContext<T>, k: usize) -> Vec<Cluster<T>> {
    let mut clusters: Vec<Cluster<T>> = Vec::with_capacity(k);
    for i in 0..k {
        let c = Cluster {
            centroid: context.initialize_centroid(i),
            elements: Vec::new(),
        };
        clusters.push(c);
    }
    clusters
}

fn reset_cluster_assignments<T>(clusters: &mut Vec<Cluster<T>>) {
    for cluster in clusters {
        cluster.elements.clear();
    }
}

fn reassign_elements<T>(
    context: &impl KmeansContext<T>,
    clusters: &mut Vec<Cluster<T>>,
    elements: &Vec<T>,
) {
    for (index, element) in elements.iter().enumerate() {
        let mut best_cluster_idx: Option<usize> = None;
        let mut best_diff: u32 = u32::MAX;
        for j in 0..clusters.len() {
            let diff = context.diff(&clusters[j].centroid, &element);
            if diff < best_diff {
                best_cluster_idx = Some(j);
                best_diff = diff;
            }
        }
        clusters[best_cluster_idx.expect("No cluster found!?")]
            .elements
            .push(Assignment {
                index,
                diff: best_diff,
            });
    }
}

fn recalculate_centroids<T>(
    context: &impl KmeansContext<T>,
    clusters: &mut Vec<Cluster<T>>,
    elements: &Vec<T>,
) {
    for cluster in clusters {
        if !cluster.elements.is_empty() {
            cluster.centroid = recalculate_centroid(&cluster.elements, &elements, context);
        }
    }
}

fn recalculate_centroid<T>(
    cluster_elements: &Vec<Assignment>,
    elements: &Vec<T>,
    context: &impl KmeansContext<T>,
) -> T {
    let element_refs = cluster_elements
        .iter()
        .map(|assignment| &elements[assignment.index])
        .collect::<Vec<&T>>();

    context.determine_centroid(&element_refs)
}

fn fill_empty_clusters<T>(
    context: &impl KmeansContext<T>,
    clusters: &mut Vec<Cluster<T>>,
    elements: &Vec<T>,
) -> bool {
    let mut max_diffs = get_highest_deviations_by_cluster(&clusters);
    if !max_diffs.is_empty() {
        println!("Max diff: {}", max_diffs[0].diff);
        if max_diffs[0].diff == 0 {
            return true;
        }
    } else {
        println!("No more diffs?!");
    }

    for c in clusters {
        if max_diffs.is_empty() {
            break;
        }
        if !c.elements.is_empty() {
            continue;
        }
        c.elements.push(Assignment {
            index: max_diffs.pop().expect("No more max diffs?!").index,
            diff: 0,
        });
        c.centroid = recalculate_centroid(&c.elements, elements, context);
    }

    false
}

fn get_highest_deviations_by_cluster<T>(clusters: &Vec<Cluster<T>>) -> Vec<Diff> {
    let mut max_diffs: Vec<Diff> = Vec::new();

    for c in clusters {
        if c.elements.len() < 2 {
            continue;
        }
        let max = c
            .elements
            .iter()
            .max_by(|a, b| a.diff.cmp(&b.diff))
            .expect("No max found in non empty list!?");
        max_diffs.push(Diff {
            index: max.index,
            diff: max.diff,
        });
    }

    max_diffs.sort_by(|d1, d2| d2.diff.cmp(&d1.diff));

    max_diffs
}
