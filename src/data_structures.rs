use std::u16;
#[derive(Debug,Clone)]
pub enum MutationType
{
    MisSense,
    InframeInsertion,
    InframeDeletion,
    FrameShift,
    StopGain,
    StopLost
}
#[derive(Debug,Clone)]
pub enum MutatedString
{
    Sequence(String),
    NotSeq 
}

impl PartialEq for MutatedString
{
    fn eq(&self, other:&MutatedString)->bool
    {
        self ==other
    }
}


#[derive(Debug,Clone)]
pub struct MutationInfo
{
    pub ref_aa_position:u16,
    pub mut_aa_position:u16,
    pub ref_aa:String,
    pub mut_aa:MutatedString,
    pub len:u16,
} 

impl MutationInfo
{
    pub fn new(ref_aa_position:u16, mut_aa_position:u16,ref_aa:String,mut_aa:String)->MutationInfo
    {
        MutationInfo
        {
            ref_aa_position:ref_aa_position,
            mut_aa_position:mut_aa_position,
            ref_aa:ref_aa,
            mut_aa:MutatedString::Sequence(mut_aa),
            len:1
        }
    }
}


pub struct Mutation
{
    pub transcrit_name:String,
    
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