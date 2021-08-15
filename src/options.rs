pub enum TargetType {
    MINIMIZE,
    MAXIMIZE
}

pub enum SelectionMethod {
    Equal,
    EqualTournament,
    Weighted,
    WeightedTournament
}
pub enum CrossoverMethod {
    RANDOM,
    BARRIER,
    DOUBLEBARRIER
}