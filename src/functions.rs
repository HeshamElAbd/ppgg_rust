pub mod text_parser
{   
    use super::super::data_structures::MutationInfo; 
    /// The function takes the consequence string and returned a Result enum either containing an Ok or Err type.
    /// # Ok
    /// a vector of strings that has three elements first the type of the mutation,
    /// secnd the transcript id and third the change in the position and the sequence of the mutated amino acids.
    /// # Errors 
    /// incase the provided string does not have six pipe sperators
    ///``` 
    /// let example_csq_string="stop_gained|RABGEF1|ENST00000484547|NMD|+|32Q>32*|66771993C>T".to_string();
    /// match split_csq_string(example_csq_string)
    /// {
    ///      Ok(fields) =>
    ///      {
    ///        for field in fields {println!("{}",field)};
    ///      }
    /// }
    ///```
    pub fn split_csq_string(input_string:String, )->Result<Vec<String>,String>
    {
        let num_match=input_string.matches('|').count();
        if num_match !=6 
        {
            return Err(format!("In correct number of fields, expected 6, recieved {}",num_match));
        }
        else
        {
            let index:Vec<usize>=vec![0,2,5];
            let res: Vec<String>= input_string.split('|').map(|elem| elem.into()).collect::<Vec<String>>(); 
            Ok(index.iter().map(|i| res[*i].clone()).collect::<Vec<String>>())
        }
    }
    /// The function takes the mutation amino acid field, e.g. "32Q>32*" and returned a Result enum either containing an Ok or Err type.
    /// # Ok
    /// a vector of strings that has three elements first the type of the mutation,
    /// secnd the transcript id and third the change in the position and the sequence of the mutated amino acids.
    /// # Errors 
    /// incase the provided string does not have six pipe sperators
    ///```rust 
    /// # let example_csq_string="stop_gained|RABGEF1|ENST00000484547|NMD|+|32Q>32*|66771993C>T".to_string();
    /// # match split_csq_string(example_csq_string)
    /// # {
    /// #     Ok(fields) =>
    /// #     {
    /// #       for field in fields {println!("{}",field)};
    /// #     }
    /// # }
    ///```
    pub fn parse_amino_acid_field(input_string:String)->Result<MutationInfo,String>
    {
        // split the field into two amino acids 
        let parsed_strings=input_string.split('>').collect::<Vec<&str>>();
        if parsed_strings.len()!=2
        {
            return Err(format!("The psrsed string has a length of: {}, expected only two",parsed_strings.len()));
        }
        // get the position and the reference sequence
        let (ref_pos, ref_seq)=match parse_amino_acid_seq_position(&parsed_strings[0])
        {
            Ok((index,sequence))=>(index,sequence), 
            Err(err_msg)=>
            {
                return Err(format!("\n while extracting the sequence and the position of the reference the following error was encounterred {}",err_msg));
            }
        };
        // get the position and the mutated sequence
        let (mut_pos,mut_seq)= match  parse_amino_acid_seq_position(&parsed_strings[1])
        {
            Ok((index,sequence))=>(index,sequence),
            Err(err_msg)=>
            {
                return Err(format!("\n while extracting the sequence and the position of the mutation the following error was encounterred {}",err_msg));
            }
        };
        Ok(MutationInfo::new(ref_pos,mut_pos,ref_seq,mut_seq))
    }
    /// The function takes an input string composite of an aminoacid position concatinated with a stirng object ,e.g 35KTEST and returns 
    /// the amino acid position as u16 int, in this case it 35, and the string containg the mutation, here it is KTEST.
    /// ## Ok
    /// a tuple containg the amino acid position as an int and the sequence as a stirng,
    /// ## Errors 
    /// a string contain the cause of faliure
    /// # Example
    ///``` 
    /// let test_example="35KTEST"
    /// match parse_amino_acid_seq_position(test_example)
    /// {
    ///       Ok((pos,seq))=>println!("The position is: {}, while the sequence is: {}",pos,seq),
    ///       Err(seq)=>_
    /// }
    ///```
    pub fn parse_amino_acid_seq_position(input_seq: &str)->Result<(u16,String),String>
    {
        if input_seq.matches('-').count() !=0
        {
            return Err(format!("Input string: {} is invalid, it contains a '-' sign which is not valid for indexing amino acid positions, also it is not avalid amino acid",input_seq));
        }
        let input_as_vec=input_seq.chars().collect::<Vec<char>>();// split the input string into a vector of chars, for example, 32Q -> 3,2,Q;
        let nums=['0','1','2','3','4','5','6','7','8','9']; // valid numbers 
        let position = match input_as_vec.iter().clone().filter(|c|  nums.contains(&c)).collect::<String>().parse() // extract the numbers from the stream 
        {
            Ok(num)=>num,
            Err(err_msg)=>
            {
                return Err(format!("Parsing the input sequence {}, failed with the following error message {}",input_seq,err_msg ));
            }
        };
        let sequence = input_as_vec.iter().clone().filter(|c| !nums.contains(&c)).collect::<String>();
        if sequence.is_empty()
        {
            return Err(format!("Input string: {} is invalid, could not extract a valid sequence from it", input_seq));
        }
        Ok((position,sequence)) // get a position, sequence tuple 
    }  
} 

#[cfg(test)]
mod test_text_parser
{
    use super::*; // make the content of the file public  
    use super::super::data_structures; 
    #[test]
    /// The function test that the function produce the correct results when given a default correct string 
    /// This test will be passed if the function only produce three strings from the provided string 
    fn test_split_csq_string_one()->Result<(),String>
    {
        let test_string="stop_gained|RABGEF1|ENST00000484547|NMD|+|32Q>32*|66771993C>T".to_string();
        match text_parser::split_csq_string(test_string)
        {
            Ok(test_string)=>
            {
                let res:Vec<String>=vec!["stop_gained","ENST00000484547","32Q>32*"].iter().map(|&x| x.to_string()).collect();
                assert_eq!(res,test_string);
                Ok(())
            },
            Err(error)=>
            {
                Err(format!("Test have failed, the function failed while it should not have failed, error was {}", error))
            }
        }
    }
    #[test]
    fn test_split_csq_string_two()->Result<(),String>
    {
        let test_string_2="5_prime_utr|RABGEF1|ENST00000437078|protein_coding".to_string(); 
        match text_parser::split_csq_string(test_string_2)
        {
            Ok(res)=>
            {
                Err(format!("The function should have returned an Err results, however, it returned {}", res[0]))
            },
            Err(error)=>
            {
                Ok(())
            }
        }
    }
    #[test]
    #[ignore]
    fn test_parse_amino_acid_field_1()
    {
        let mut_string="32Q>32*".to_string();
        let res = text_parser::parse_amino_acid_field(mut_string).expect("Generating the parse_amino_acid failed");
        let mut_info=data_structures::MutationInfo
        {
            ref_aa_position:32,
            mut_aa_position:32,
            ref_aa:"Q".to_string(),
            mut_aa:data_structures::MutatedString::NotSeq,
            len:0,
        };
        assert_eq!(mut_info.ref_aa_position,res.ref_aa_position); 
        assert_eq!(mut_info.mut_aa_position,res.mut_aa_position); 
        assert_eq!(mut_info.ref_aa,res.ref_aa); 
        assert_eq!(mut_info.mut_aa,res.mut_aa); 
        assert_eq!(mut_info.len,res.len); 
    }
    #[test]
    fn test_parse_amino_acid_seq_position()
    {
        let input_seq="32Q".to_string();
        match text_parser::parse_amino_acid_seq_position(&input_seq)
        {
            Ok((pos,seq))=>
            {
                assert_eq!(pos,32u16);
                assert_eq!(seq,"Q".to_string());
            }
            Err(err_msg)=>
            {
                format!("Test failed, the code resturned an error message: {}",err_msg);
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_indepth()
    {
        let amino_acids_chars="ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
        for i in 0..100 // simulate different amino acids senario
        {
            let pos:u16=i;
            for j in 1..24 // simulate postential length
            {
                let random_seq=amino_acids_chars.iter().take(j).collect::<String>(); // get a test sequence
                let mut pos_dev=pos.to_string();
                pos_dev.push_str(&random_seq);
                match text_parser::parse_amino_acid_seq_position(&pos_dev)
                {
                    Ok((res_pos,res_seq))=>
                    {
                        assert_eq!(pos,res_pos);
                        assert_eq!(random_seq,res_seq);
                    },
                    Err(err_msg)=>
                    {
                        println!("Test Case faile with the following error message: {}, input string was: {}",err_msg,pos_dev);
                    }
                }
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_bad_input_1()->Result<(),String>
    {
        let test_case="Test"; // here test case should fail because there is no position  
        match text_parser::parse_amino_acid_seq_position(&test_case)
        {
            Ok((pos,seq))=>
            {
                println!("Test Case failed it returned the following results: {},{}",pos,seq);
                Err(format!("Test Case failed it returned the following results: {},{}",pos,seq))
            },
            Err(err_msg)=>
            {
                println!("Test Case faile with the following error message: input string was: {}",err_msg);
                Ok(())
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_bad_input_2()->Result<(),String>
    {
        let test_case=""; // here test case should fail because there is no position  
        match text_parser::parse_amino_acid_seq_position(&test_case)
        {
            Ok((pos,seq))=>
            {
                println!("Test Case failed it returned the following results: {},{}",pos,seq);
                Err(format!("Test Case failed it returned the following results: {},{}",pos,seq))
            },
            Err(err_msg)=>
            {
                println!("Test Case faile with the following error message: input string was: {}",err_msg);
                Ok(())
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_bad_input_3()->Result<(),String>
    {
        let test_case="223"; // here test case should fail because there is no position  
        match text_parser::parse_amino_acid_seq_position(&test_case)
        {
            Ok((pos,seq))=>
            {
                println!("Test Case failed it returned the following results: {},{}",pos,seq);
                Err(format!("Test Case failed it returned the following results: {},{}",pos,seq))
            },
            Err(err_msg)=>
            {
                println!("Test Case faile with the following error message: input string was: {}",err_msg);
                Ok(())
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_bugged_input()->Result<(),String>
    {
        let test_case="-223QK"; // here test case should fail because there is no position  
        match text_parser::parse_amino_acid_seq_position(&test_case)
        {
            Ok((pos,seq))=>
            {
                println!("Test Case failed it returned the following results: {},{}",pos,seq);
                Err(format!("Test Case failed it returned the following results: {},{}",pos,seq))
            },
            Err(err_msg)=>
            {
                println!("Test Case faile with the following error message: input string was: {}",err_msg);
                Ok(())
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_rare_input_1()
    {
        let input_seq="32*".to_string();
        match text_parser::parse_amino_acid_seq_position(&input_seq)
        {
            Ok((pos,seq))=>
            {
                assert_eq!(pos,32u16);
                assert_eq!(seq,"*".to_string());
            }
            Err(err_msg)=>
            {
                println!("Test failed, the code resturned an error message: {}",err_msg);
            }
        }
    }
    #[test]
    fn test_parse_amino_acid_seq_position_rare_input_2()
    {
        let input_seq="32KMNOPQQQ*".to_string();
        match text_parser::parse_amino_acid_seq_position(&input_seq)
        {
            Ok((pos,seq))=>
            {
                assert_eq!(pos,32u16);
                assert_eq!(seq,"KMNOPQQQ*".to_string());
            }
            Err(err_msg)=>
            {
                println!("Test failed, the code resturned an error message: {}",err_msg);
            }
        }
    }
}
