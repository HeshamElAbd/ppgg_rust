use std::u16;

pub enum MutationType
{
    MisSense,
    InframeInsertion,
    InframeDeletion,
    FrameShift,
    StopGain,
    StopLost
}

pub struct Mutation
{
    pub transcrit_name:String,
    pub ref_aa_position:u16,
    pub mut_aa_position:u16,
    pub ref_aa:String,
    pub mut_aa:String,
    pub len:u16,
    pub mut_type:MutationType
}
/*
impl Mutation
{
    fn new(info_vec:Vec<String>)->Result<Mutation,String>
    {
        if info_vec.len()
        
    }
}

#[cfg(test)]
mod test_mutations
{
    #[test]
    fn test_mutation_()
    {
        let code_str: String::from("");
    }
}*/