use std::collections::HashMap;
use enum_iterator::{all, first, Sequence};

#[derive(PartialEq,Eq, Hash, Copy, Clone)]
enum Observations {
    Normal,
    Cold,
    Dizzy
}

#[derive(PartialEq,Eq, Hash, Sequence, Copy, Clone, Debug)]
enum States {
    Healthy,
    Fever
}

struct VEntry {
    probability: f64,
    previous: Option<States>,
}

fn main() {
    let start_probability = HashMap::from([
        (States::Healthy, 0.6),
        (States::Fever, 0.4),
    ]);

    let transition_probability = HashMap::from([
        (States::Healthy, HashMap::from([
            (States::Healthy, 0.7),
            (States::Fever, 0.3),
        ])),
        (States::Fever, HashMap::from([
            (States::Healthy, 0.4),
            (States::Fever, 0.6),
        ]))
    ]);

    let emit_probability = HashMap::from([
        (States::Healthy, HashMap::from([
            (Observations::Normal, 0.5),
            (Observations::Cold, 0.4),
            (Observations::Dizzy, 0.1),
        ])),
        (States::Fever, HashMap::from([
            (Observations::Normal, 0.1),
            (Observations::Cold, 0.3),
            (Observations::Dizzy, 0.6),
        ]))
    ]);

    let obs = vec![Observations::Normal, Observations::Cold, Observations::Dizzy];

    let mut v: Vec<HashMap<States, VEntry>> = Vec::new();

    v.push(HashMap::new());
    for st in all::<States>() {
        v[0].insert(st, VEntry {
            probability: start_probability[&st] * emit_probability[&st][&obs[0]],
            previous: None
        });
    }
    
    for t in 1..obs.len() {
        v.push(HashMap::new());

        for st in all::<States>() {
            let first_state_enum = first::<States>().unwrap();
            let first_state = &v[t-1].get(&first_state_enum);
            let current_transition_probability = &transition_probability[&first_state_enum].get(&st);
            let current_emit_probability = emit_probability[&st][&obs[t]];
            
            let mut max_transition_probablitiy = first_state.unwrap().probability * current_transition_probability.unwrap() * current_emit_probability;
            let mut prev_st_selected = first::<States>().unwrap();
            let mut state_iterator = all::<States>();
            state_iterator.next();

            for prev_st in state_iterator {
                let tr_prob = &v[t-1].get(&prev_st).unwrap().probability * transition_probability[&prev_st].get(&st).unwrap() * emit_probability.get(&st).unwrap().get(&obs[t]).unwrap();
                if tr_prob > max_transition_probablitiy {
                    max_transition_probablitiy = tr_prob;
                    prev_st_selected = prev_st;
                }
            }

            v[t].insert(st, VEntry {
                probability: max_transition_probablitiy,
                previous: Some(prev_st_selected),
            });
        }
    }

    let mut max_probability: f64 = 0.0;
    let mut best_st: States = States::Fever;
    let mut opt: Vec<States> = Vec::new();

    for (st, data) in v.last().unwrap() {
        if max_probability < data.probability {
            max_probability = data.probability;
            best_st = *st;
        }
    }

    opt.push(best_st);
    let mut previous = best_st;

    let a = -1..(v.len() as i32) - 1;

    for t in a.rev() {
        let prev = v[(t+1) as usize].get(&previous).unwrap().previous;

        match prev {
            Some(p) => {
                opt.insert(0, p);
                previous = p
            },
            None => println!("should have had a value"),
        }
    }
    println!("Steps are: {:?} with highest probability of {}", opt, max_probability);
}
