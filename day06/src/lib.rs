mod input;

type Answers = Vec<u32>;

pub fn parse_input(input: &str) -> Vec<Vec<Answers>> {
    input
        .split("\n\n")
        .map(|group_answers| {
            group_answers
                .split('\n')
                .map(|person_answers| person_answers.chars().map(|c| (c as u32) - 97).collect())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{input::INPUT, parse_input};

    #[test]
    fn first_test() { // Part 1
        let total_yeses = parse_input(INPUT)
            .iter()
            .flat_map(|group_answers| {
                group_answers.iter().flatten().fold(
                    vec![false; 26],
                    |mut combined_answers, answer_number| {
                        combined_answers[*answer_number as usize] = true;
                        combined_answers
                    },
                )
            })
            .filter(|b| *b)
            .count();

        println!(
            "Number of questions anyone in a group answered 'yes' to: {}",
            total_yeses
        );
    }

    #[test]
    fn second_test() { // Part 2
        let total_agreed_answers: usize = parse_input(INPUT)
            .iter()
            .map(|group_answers| {
                let (answer_freq, num_people) = group_answers.iter().fold(
                    (vec![0; 26], 0usize),
                    |(mut answer_freq, num_people), answer_numbers| {
                        for number in answer_numbers {
                            answer_freq[*number as usize] += 1;
                        }
                        (answer_freq, num_people + 1)
                    },
                );
                answer_freq.iter().filter(|f| **f == num_people).count()
            })
            .sum();

        println!(
            "Number of questions a whole group answered 'yes' to: {}",
            total_agreed_answers
        )
    }
}
