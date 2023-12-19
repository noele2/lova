use std::fs::read_to_string;
use std::collections::HashMap;


//Encode the statement into a vector of arrays 
fn encode_statement(proof_lines: &Vec<Vec<String>>) -> Vec<[i64; 3]>{
   let mut statement_dict = HashMap::from([
       ("!", 1),
      ("&", 2),
      ("|", 3),
      (">",4)]);   

   let symbols = ["!", "&", "|", ">"];
   let mut count:i64 = 5;

   let mut encoded_statements = Vec::new();
   for line in proof_lines{
      let mut statement: [i64; 3] = [0; 3];
      let mut raw_statement = line[0].split(&['!', '&','|','>'][..]).collect::<Vec<_>>();

      for symbol in symbols{
         if line[0].contains(&symbol.to_owned()){
            raw_statement.push(symbol);
         }
      }
      
      for (index, elem) in raw_statement.iter().enumerate() {
         if statement_dict.get(elem) == None {
            statement_dict.insert(elem, count);
            statement[index] = count;
            count += 1;
         } else {
            statement[index] = *statement_dict.get(elem).unwrap_or(&0);      
         }
      }
      encoded_statements.push(statement);

   }

   encoded_statements
}

//Encode Logic steps
fn encode_logic(proof_lines: &Vec<Vec<String>>) -> Vec<i64>{
   let mut statement_dict = HashMap::from([
       ("hypothesis", 1),
      ("addition", 2),
      ("conjunction",3),
      ("simplification", 4),
      ("modusponens", 5),
      ("modustollens", 6),
      ("hypotheticalsyllogism",7),
      ("disjunctivesyllogism", 8)]);   

   let mut encoded_logic: Vec<i64> = Vec::new();
   for line in proof_lines{
      let formatted_line = line[1].replace(&[','][..], "").to_lowercase();
      encoded_logic.push(*statement_dict.get(&formatted_line as &str).unwrap_or(&0));      
   }

   encoded_logic
}


//Encode Necessary Reasoning lines
fn encode_reasoning(proof_lines: &Vec<Vec<String>>, encoded_statements: &Vec<[i64; 3]>) -> Vec<[i64; 6]>{

   let mut encode_reasoning = Vec::new();
   for line in proof_lines{
      let mut reasoning_lines: [i64; 6] = [0; 6];

      if line.len() == 3{
         let raw_reasoning = line[2].split(&[','][..]).collect::<Vec<_>>(); 

         let mut array_position = 0;
         for elem in raw_reasoning{
            let index = elem.parse::<i64>().unwrap()-1;
            let temp_array = encoded_statements[index as usize];

            for val in temp_array{
               reasoning_lines[array_position] = val;
               array_position +=1;
            }
            array_position += (array_position % 3);
         }
      } 
   
      encode_reasoning.push(reasoning_lines);      
   }
   encode_reasoning
}



//Reading the proof into a vector of strings
fn read_proof(path: &str) -> Vec<Vec<String>> {
   let mut result = Vec::new();
   let file = read_to_string(path).unwrap();

   for line in file.lines() {
      let  proof_line = line.replace(&[' '][..], "").split(&['(', ')', '[', ']']).filter(|&r| r != "").map(|r| r.to_string() ).collect::<Vec<_>>();
      result.push(proof_line);
  }

  result
}


fn main() {
   // --snip--
   let proof_lines = read_proof("misc/basic_proof.txt");
   println!("{:?}", proof_lines);

   let encoded_statements = encode_statement(&proof_lines);
   println!("{:?}", encoded_statements);

   let encoded_logic = encode_logic(&proof_lines);
   println!("{:?}", encoded_logic);

   let encoded_reasoning = encode_reasoning(&proof_lines, &encoded_statements);
   println!("{:?}", encoded_reasoning);
   
}