#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(PartialEq, Eq, Debug)]
struct BucketState {
    number: Bucket,
    capacity: u8,
    contents: u8,
}

impl BucketState {
    pub fn new(capacity: u8, number: Bucket) -> Self {
        BucketState {
            number,
            capacity,
            contents: 0,
        }
    }

    pub fn empty(&mut self) {
        self.contents = 0;
    }

    pub fn fill(&mut self) {
        self.contents = self.capacity;
    }

    pub fn remaining_capacity(&self) -> u8 {
        self.capacity - self.contents
    }

    pub fn is_empty(&self) -> bool {
        self.contents == 0
    }

    pub fn is_full(&self) -> bool {
        self.contents == self.capacity
    }

    pub fn has(&self, amount: u8) -> bool {
        self.contents >= amount
    }

    pub fn take(&mut self, amount: u8) {
        self.contents -= amount;
    }

    pub fn add(&mut self, amount: u8) {
        self.contents += amount
    }
}

fn is_satisfied(
    bucket_one: &BucketState,
    bucket_two: &BucketState,
    goal: u8,
) -> Option<(Bucket, u8)> {
    if bucket_one.contents == goal {
        Some((Bucket::One, bucket_two.contents))
    } else if bucket_two.contents == goal {
        Some((Bucket::Two, bucket_one.contents))
    } else {
        None
    }
}

fn get_empty(bucket_one: &BucketState, bucket_two: &BucketState) -> Option<Bucket> {
    if bucket_one.is_empty() {
        Some(Bucket::One)
    } else if bucket_two.is_empty() {
        Some(Bucket::Two)
    } else {
        None
    }
}

fn get_full(bucket_one: &BucketState, bucket_two: &BucketState) -> Option<Bucket> {
    if bucket_one.is_full() {
        Some(Bucket::One)
    } else if bucket_two.is_full() {
        Some(Bucket::Two)
    } else {
        None
    }
}

fn fill(target: Bucket, a: &mut BucketState, b: &mut BucketState) {
    match target {
        Bucket::One => a.fill(),
        Bucket::Two => b.fill(),
    }
}

fn empty(target: Bucket, a: &mut BucketState, b: &mut BucketState) {
    match target {
        Bucket::One => a.empty(),
        Bucket::Two => b.empty(),
    }
}

fn transfer(from: &mut BucketState, to: &mut BucketState) -> bool {
    let amount = std::cmp::min(from.contents, to.remaining_capacity());

    if amount == 0 || !from.has(amount) {
        return false;
    }

    from.take(amount);
    to.add(amount);

    true
}

fn transfer_target(target: Bucket, a: &mut BucketState, b: &mut BucketState) {
    match target {
        Bucket::One => transfer(a, b),
        Bucket::Two => transfer(b, a),
    };
}

fn solve_recursive(
    bucket_one: &mut BucketState,
    bucket_two: &mut BucketState,
    start: Bucket,
    moves: u8,
    goal: u8,
) -> Option<BucketStats> {

    if let Some((bucket, other_contents)) = is_satisfied(bucket_one, bucket_two, goal) {
        return Some(BucketStats {
            moves,
            goal_bucket: bucket,
            other_bucket: other_contents,
        });
    }

    if bucket_one.is_full() && bucket_two.is_full() {
        return None;
    }

    if bucket_one.capacity == goal {
        bucket_one.fill()
    } else if bucket_two.capacity == goal {
        bucket_two.fill()
    } else if let Some(empty) = get_empty(bucket_one, bucket_two) {
        if empty == start {
            fill(empty, bucket_one, bucket_two)
        } else {
            transfer_target(empty, bucket_two, bucket_one);
        }
    } else if let Some(filled) = get_full(bucket_one, bucket_two) {
        if filled == start {
            transfer_target(filled, bucket_one, bucket_two);
        } else {
            empty(filled, bucket_one, bucket_two);
        }
    }

    solve_recursive(bucket_one, bucket_two, start, moves + 1, goal)
}
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut bucket_one = BucketState::new(capacity_1, Bucket::One);
    let mut bucket_two = BucketState::new(capacity_2, Bucket::Two);

    match start_bucket {
        Bucket::One => bucket_one.fill(),
        Bucket::Two => bucket_two.fill(),
    }

    solve_recursive(&mut bucket_one, &mut bucket_two, *start_bucket, 1, goal)
}
