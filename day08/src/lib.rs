mod input;

type Error = Box<dyn std::error::Error>;
type Memory = Vec<Instruction>;

pub enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64)
}

impl Instruction {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut parts = input.split(" ");
        let op = parts.next().unwrap();
        let arg = Self::parse_arg(parts.next().unwrap())?;

        match op {
            "acc" => Ok(Instruction::ACC(arg)),
            "jmp" => Ok(Instruction::JMP(arg)),
            "nop" => Ok(Instruction::NOP(arg)),
            _ => Err("Invalid Input: Bad Instruction".into())
        }
    }

    fn parse_arg(input: &str) -> Result<i64, Error> {
        let mut chars = input.chars();
        let sign = chars.next().unwrap();
        let num = chars.collect::<String>();
        match sign {
            '-' => Ok(-num.parse::<i64>()?),
            '+' => Ok(num.parse::<i64>()?),
            _ => Err("Invalid Input: Bad Sign".into())
        }
    }
}

pub fn parse_input(input: &str) -> Result<Memory, Error> {
    Ok(
        input.split('\n')
        .map(|line| Instruction::parse(line).unwrap())
        .collect::<Memory>()
    )
}

pub fn boot(memory: &Memory) -> Result<i64, i64> {
    let mem_size = memory.len();
    let mut ipointer: u64 = 0;
    let mut memory_hit = Vec::<u64>::new();
    let mut acc: i64 = 0;

    while !memory_hit.contains(&ipointer) {
        memory_hit.push(ipointer);
        match memory[ipointer as usize] {
            Instruction::NOP(_) => ipointer += 1,
            Instruction::JMP(offset) => ipointer = ((ipointer as i64) + offset) as u64,
            Instruction::ACC(value) => {
                ipointer += 1;
                acc += value;
            }
        }
        if ipointer as usize >= mem_size {return Ok(acc)}
    }
    Err(acc)
}

#[cfg(test)]
mod tests {
    use super::{input::*, parse_input, Instruction, boot};

    #[test]
    fn find_acc_before_first_repeated_instruction() { // Part 1
        let memory = parse_input(INPUT).unwrap();
        let acc = boot(&memory).unwrap_err();
        println!("ACC before the first repeated instruction: {}", acc)
    }

    #[test]
    fn bruteforce_find_corrupted_instruction() { // Part 2
        let mut memory = parse_input(INPUT).unwrap();
        let mut acc:i64 = 0;
        for i in 0..memory.len() {
            match memory[i] {
                Instruction::ACC(_) => continue,
                Instruction::JMP(x) => memory[i] = Instruction::NOP(x),
                Instruction::NOP(x) => memory[i] = Instruction::JMP(x)
            }

            match boot(&memory) {
                Ok(result) => { acc = result; break; },
                Err(_) => ()
            }

            match memory[i] {
                Instruction::ACC(_) => continue,
                Instruction::JMP(x) => memory[i] = Instruction::NOP(x),
                Instruction::NOP(x) => memory[i] = Instruction::JMP(x)
            }
        }
        println!("Booted Successfuly!\nACC: {}", acc);
    }
}