use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<String, Pulse> },
    Broadcaster,
    Sink,
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    module_type: ModuleType,
    output: Vec<String>,
}

#[derive(Debug)]
struct Machine {
    modules: HashMap<String, Module>,
}

#[derive(Debug)]
struct PassedPulse {
    src: String,
    dst: String,
    pulse: Pulse,
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let input = parts.next().unwrap();
        let output = parts.next().unwrap();
        let module_type = match input.chars().next().unwrap() {
            '%' => ModuleType::FlipFlop { on: false },
            '&' => ModuleType::Conjunction {
                inputs: HashMap::new(),
            },
            _ => ModuleType::Broadcaster,
        };
        let name = match module_type {
            ModuleType::FlipFlop { .. } => input[1..].to_string(),
            ModuleType::Conjunction { .. } => input[1..].to_string(),
            ModuleType::Broadcaster => input.to_string(),
            _ => panic!("Invalid module type"),
        };
        let output = output.split(", ").map(|s| s.to_string()).collect();
        Ok(Module {
            name,
            module_type,
            output,
        })
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules: HashMap<_, _> = s
            .trim()
            .lines()
            .map(|s| {
                let module: Module = s.trim().parse().unwrap();
                (module.name.clone(), module)
            })
            .collect();
        let mut final_modules = modules.clone();
        for (_, module) in modules {
            for output in &module.output {
                let output_module = match final_modules.get_mut(output) {
                    Some(m) => m,
                    None => {
                        final_modules.insert(
                            output.clone(),
                            Module {
                                name: output.clone(),
                                module_type: ModuleType::Sink,
                                output: vec![],
                            },
                        );
                        final_modules.get_mut(output).unwrap()
                    }
                };
                if let ModuleType::Conjunction { inputs } = &mut output_module.module_type {
                    inputs.insert(module.name.clone(), Pulse::Low);
                }
            }
        }
        Ok(Machine {
            modules: final_modules,
        })
    }
}

impl ToString for PassedPulse {
    fn to_string(&self) -> String {
        let pulse = match self.pulse {
            Pulse::High => "high",
            Pulse::Low => "low",
        };
        format!("{} -{}-> {}", self.src, pulse, self.dst)
    }
}

impl Module {
    fn process(&mut self, passed: PassedPulse) -> Vec<PassedPulse> {
        let mut result: Vec<_> = Vec::new();
        match &mut self.module_type {
            ModuleType::FlipFlop { on } => {
                match passed.pulse {
                    Pulse::High => (), // Ignore high pulses
                    Pulse::Low => {
                        let output_pulse = match *on {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        *on = !*on;
                        for output in self.output.iter() {
                            let next = PassedPulse {
                                src: self.name.clone(),
                                dst: output.clone(),
                                pulse: output_pulse.clone(),
                            };
                            result.push(next);
                        }
                    }
                }
            }
            ModuleType::Conjunction { inputs } => {
                inputs.insert(passed.src, passed.pulse);
                let output_pulse = match inputs.values().all(|p| p == &Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                for output in self.output.iter() {
                    let next = PassedPulse {
                        src: self.name.clone(),
                        dst: output.clone(),
                        pulse: output_pulse.clone(),
                    };
                    result.push(next);
                }
            }
            ModuleType::Broadcaster => {
                for output in self.output.iter() {
                    let next = PassedPulse {
                        src: self.name.clone(),
                        dst: output.clone(),
                        pulse: passed.pulse.clone(),
                    };
                    result.push(next);
                }
            }
            ModuleType::Sink => (), // Ignore pulses to sinks
        }
        result
    }
}

fn solve(input: &str) -> usize {
    let mut machine: Machine = input.parse().unwrap();
    //dbg!(&machine);

    let mut high_cout = 0;
    let mut low_count = 0;
    for _ in 0..1000 {
        let mut queue: Vec<PassedPulse> = vec![PassedPulse {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            pulse: Pulse::Low,
        }];
        while queue.len() > 0 {
            let passed = queue.remove(0);
            //println!("{}", passed.to_string());

            match passed.pulse {
                Pulse::High => high_cout += 1,
                Pulse::Low => low_count += 1,
            }

            let module = machine.modules.get_mut(&passed.dst).unwrap();
            let next = module.process(passed);
            queue.extend(next);
        }
    }

    return high_cout * low_count;
}

fn main() {
    let input = include_str!("../../inputs/day20.txt");
    let ans = solve(input);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a";
        assert_eq!(solve(input), 32000000);

        let input = r"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output";
        assert_eq!(solve(input), 11687500);
    }
}
