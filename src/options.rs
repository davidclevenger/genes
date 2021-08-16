pub enum TargetType {
    Minimize,
    Maximize
}

pub enum SelectionMethod {
    Equal,
    EqualTournament,
    Weighted,
    WeightedTournament
}
pub enum CrossoverMethod {
    Random,
    Barrier,
    DoubleBarrier
}