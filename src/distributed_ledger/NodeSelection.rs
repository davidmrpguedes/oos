use std::collections::HashMap;

struct Validator {
    address: String,
    stake: u64,
}

//This defines a function that takes a slice of validators and the number of validators to select and returns a vector of the selected validators.
fn select_validators(validators: &[Validator], num_validators: usize) -> Vec<Validator> {
    let total_stake: u64 = validators.iter().map(|v: &Validator| v.stake).sum(); //Calculates the total stake of all validators by iterating over the validators and summing their stakes using the `iter()` and `sum()` methods.
    let mut validator_probabilities: HashMap<usize, f64> = HashMap::new(); //This creates an empty `HashMap` to store the validator probabilities. The keys are the indices of the validators, and the values are their probabilities.
    //This calculates the probability of each validator being selected as a leader for a given block based on their stake. 
    //The probability is the validator's stake divided by the total stake in the network. 
    //The probabilities are stored in the `validator_probabilities` hash map using the validator index as the key.
    for (i, validator) in validators.iter().enumerate() {
        let probability: f64 = validator.stake as f64 / total_stake as f64;
        validator_probabilities.insert(i, probability);
    } 
    //This selects the validators by choosing a random validator index based on the probabilities stored in the `validator_probabilities` hash map 
    //using the `weighted_random` function. The function returns a vector of the selected validators.
    let mut selected_validators: Vec<Validator> = Vec::new();
    for _ in 0..num_validators {
        let validator_index: usize = weighted_random(&validator_probabilities);
        selected_validators.push(validators[validator_index].clone());
    }
    return selected_validators;
}

//This defines a function that takes a hash map of indices and weights and returns a randomly selected index based on the weights.
fn weighted_random<T>(weights: &HashMap<usize, f64>) -> usize {
    //This creates a new `rand` `ThreadRng` instance, which we will use for generating random numbers.
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut remaining_weight: f64 = 1.0;
    let mut chosen_index: usize = 0;
    //This iterates over the weights and calculates the remaining weight of the remaining indices. 
    //It then generates a random number between 0 and 1 using the `gen()` method of the `ThreadRng` instance. 
    //If the random number is greater than the remaining weight, the current index is selected
    for (i, weight) in weights.iter() {
        remaining_weight -= weight;
        if rng.gen::<f64>() > remaining_weight {
            chosen_index = *i;
            break;
        }
    }
    return chosen_index;
}
