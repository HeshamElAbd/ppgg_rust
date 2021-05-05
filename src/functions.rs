pub mod text_parser
{
    pub fn split_csq_string(input_string:&String, )->Result<Vec<String>,String>
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
} 

#[cfg(test)]
mod test_text_parser
{
    use super::*; // make the content of the file public  
    
    #[test]
    fn test_split_csq_string_one()->Result<(),String>
    {
        let test_string="stop_gained|RABGEF1|ENST00000484547|NMD|+|32Q>32*|66771993C>T".to_string();
        match text_parser::split_csq_string(&test_string)
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

}