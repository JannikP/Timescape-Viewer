use crate::core::{FullStat, IForestIndex, Version};

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    version: Version,
    data: IForestIndex<FullStat>,
    // TODO: Reference to timestamps? e. g. IForestIndex<TimeStamp> or something like that?
    // TODO: Data structure holding delta values for lean updates of the trace data.
}
