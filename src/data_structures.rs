use std::{str::FromStr, u16};
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
#[derive(Debug,Clone,PartialEq)]
pub enum MutatedString
{
    Sequence(String),
    EndSequence(String),
    NotSeq 
}


impl FromStr for MutatedString
{
    type Err=(); 

    fn from_str(input_str:&str)->Result<MutatedString, ()>
    {
        if input_str=="*"
        {
            return Ok(MutatedString::NotSeq);
        }
        else if input_str.matches('*').count()!=0 
        {
            return Ok(MutatedString::EndSequence(input_str.to_string()));
        }
        else
        {
            return Ok(MutatedString::Sequence(input_str.to_string()))
        }       
    }
}


#[derive(Debug,Clone,PartialEq)]
pub struct MutationInfo
{
    pub ref_aa_position:u16,
    pub mut_aa_position:u16,
    pub ref_aa:MutatedString,
    pub mut_aa:MutatedString,
    
} 

impl MutationInfo
{
    pub fn new(ref_aa_position:u16, mut_aa_position:u16,ref_aa:String,mut_aa:String)->MutationInfo
    {
        MutationInfo
        {
            ref_aa_position:ref_aa_position-1, // rest the index to be 0-indexed
            mut_aa_position:mut_aa_position-1,
            ref_aa:MutatedString::from_str(&ref_aa).unwrap(),
            mut_aa:MutatedString::from_str(&mut_aa).unwrap(),
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