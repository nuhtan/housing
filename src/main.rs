use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Instant,
};

const PEOPLE: [&'static str; 12] = [
    "alex",
    "anne",
    "calvin",
    "elliot",
    "gabby",
    "gary",
    "joseph",
    "nathaniel",
    "nick",
    "nik",
    "sasha",
    "tim",
];

const PORT: u16 = 8020;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", PORT)).unwrap();
    for stream in listener.incoming() {
        let handle = thread::spawn(move || {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            match line.find("/") {
                Some(start) => {
                    let request = line[start..line.find("HTTP").unwrap() - 1].to_string();
                    if request.len() < 15 || !(&request[0..14] == "/api/conflict?") {
                        let response = format!("HTTP/1.1 200 OK\r\nConnection: Close\r\nContent-Type: text/html\r\n\r\n{}", fs::read_to_string("index.html").expect("Failed to read html file."));
                        stream.write_all(response.as_bytes()).unwrap();
                    } else {
                        let mut conflicts = Vec::new();
                        let mut ending = request[14..].to_string();
                        ending = ending.replace("%", "");
                        while ending.len() > 0 {
                            match ending.find("&_&") {
                                Some(found) => {
                                    let person1 = ending[0..ending.find("&").unwrap()].to_string();
                                    let person2 =
                                        ending[ending.find("&").unwrap() + 1..found].to_string();
                                    conflicts.push(Conflict { person1, person2 });
                                    ending = String::from(&ending[found + 3..]);
                                }
                                None => {
                                    let person1 = ending[0..ending.find("&").unwrap()].to_string();
                                    let person2 =
                                        ending[ending.find("&").unwrap() + 1..].to_string();
                                    conflicts.push(Conflict { person1, person2 });
                                    ending = String::from("");
                                }
                            }
                        }
                        println!("Generation Started");
                        let now = Instant::now();
                        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: Close\r\n\r\n{}", gen_combos(conflicts));
                        let time = now.elapsed().as_millis();
                        println!("Generation + Formatting Complete in {} milliseconds.", time);
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
                None => {}
            }
        });
        handle.join().unwrap();
    }
}

fn gen_combos(conflicts: Vec<Conflict>) -> String {
    let mut floors = Vec::new();
    gen(&mut floors, PEOPLE.clone().to_vec(), conflicts);

    let mut real: Vec<(Floor, Floor, Floor)> = Vec::new();
    let mut counter: u32 = 1;
    let mut data = format!("{{\"configurations\": [");
    println!("Generated {} possible configurations", floors.len());
    println!("Generated configurations now being formatted");
    let now = Instant::now();
    for res in floors {
        let yeet = res.clone();
        data.push_str("{\"first\": [");
        for (index, person) in res.0.populous.iter().enumerate() {
            data.push_str("\"");
            data.push_str(person);
            data.push_str("\"");
            if index != 3 {
                data.push_str(",")
            }
        }
        data.push_str("], \"second\": [");
        for (index, person) in res.1.populous.iter().enumerate() {
            data.push_str("\"");
            data.push_str(person);
            data.push_str("\"");
            if index != 3 {
                data.push_str(",")
            }
        }
        data.push_str("], \"third\": [");
        for (index, person) in res.2.populous.iter().enumerate() {
            data.push_str("\"");
            data.push_str(person);
            data.push_str("\"");
            if index != 3 {
                data.push_str(",")
            }
        }
        data.push_str("], \"option\":");
        data.push_str(&counter.to_string());
        data.push_str("},");
        real.push(yeet);
        counter += 1;
    }
    println!(
        "Formatting took {} milliseconds.",
        now.elapsed().as_millis()
    );
    data.pop();
    data.push_str("]}");
    return data;
}

fn gen(
    floors: &mut Vec<(Floor, Floor, Floor)>,
    remaining: Vec<&'static str>,
    conflicts: Vec<Conflict>,
) {
    let mut first_floors: Vec<Floor> = Vec::new();
    let people = remaining.clone();
    let mut rem_loop = remaining.clone();
    let person1 = remaining.first().unwrap();
    rem_loop.remove(
        rem_loop
            .iter()
            .enumerate()
            .find(|x| x.1 == person1)
            .unwrap()
            .0,
    );
    let rem_sec = rem_loop.clone();
    for person2 in rem_loop {
        let mut rem2 = rem_sec.clone();
        rem2.remove(rem2.iter().enumerate().find(|x| x.1 == &person2).unwrap().0);
        let rem2_loop = rem2.clone();
        for person3 in rem2_loop {
            let mut rem3 = rem2.clone();
            rem3.remove(rem3.iter().enumerate().find(|x| x.1 == &person3).unwrap().0);
            for person4 in rem3 {
                let mut floor = Floor::new(conflicts.clone());
                if floor.try_member(person1) {
                    floor.add_member(person1);
                    if floor.try_member(person2) {
                        floor.add_member(person2);
                        if floor.try_member(person3) {
                            floor.add_member(person3);
                            if floor.try_member(person4) {
                                floor.add_member(person4);
                                first_floors.push(floor);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut trimmed: Vec<Floor> = Vec::new();
    for floor in first_floors {
        if !trimmed.iter().any(|x| x == &floor) {
            trimmed.push(floor);
        }
    }
    for floor1 in trimmed {
        let people1 = people.clone();
        let mut seconds = Vec::new();
        let mut rem_second_gen: Vec<&&str> = people1
            .iter()
            .filter(|x| {
                x != &&floor1.populous[0]
                    && x != &&floor1.populous[1]
                    && x != &&floor1.populous[2]
                    && x != &&floor1.populous[3]
            })
            .collect();
        // Start looping for second floor
        let rem_sec_sec = rem_second_gen.clone();
        let person1 = rem_sec_sec.first().unwrap();
        rem_second_gen.remove(
            rem_second_gen
                .iter()
                .enumerate()
                .find(|x| x.1 == person1)
                .unwrap()
                .0,
        );
        for person2 in rem_second_gen.clone() {
            let mut rem2 = rem_second_gen.clone();
            rem2.remove(rem2.iter().enumerate().find(|x| x.1 == &person2).unwrap().0);
            let rem2_loop = rem2.clone();
            for person3 in rem2_loop {
                let mut rem3 = rem2.clone();
                rem3.remove(rem3.iter().enumerate().find(|x| x.1 == &person3).unwrap().0);
                for person4 in rem3 {
                    let mut floor = Floor::new(conflicts.clone());
                    if floor.try_member(person1) {
                        floor.add_member(person1);
                        if floor.try_member(person2) {
                            floor.add_member(person2);
                            if floor.try_member(person3) {
                                floor.add_member(person3);
                                if floor.try_member(person4) {
                                    floor.add_member(person4);
                                    seconds.push(floor);
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut trimmed_seconds: Vec<Floor> = Vec::new();
        for floor in seconds {
            if !trimmed_seconds.iter().any(|x| x == &floor) {
                trimmed_seconds.push(floor);
            }
        }
        let mut trimmed_thirds: Vec<Floor> = Vec::new();
        for floor2 in trimmed_seconds {
            let people3 = people.clone();
            let mut remaining3: Vec<&&str> = people3
                .iter()
                .filter(|x| {
                    x != &&floor1.populous[0]
                        && x != &&floor1.populous[1]
                        && x != &&floor1.populous[2]
                        && x != &&floor1.populous[3]
                        && x != &&floor2.populous[0]
                        && x != &&floor2.populous[1]
                        && x != &&floor2.populous[2]
                        && x != &&floor2.populous[3]
                })
                .collect();
            let remaining3_clone = remaining3.clone();
            let person1 = remaining3_clone.first().unwrap();
            remaining3.remove(
                remaining3
                    .iter()
                    .enumerate()
                    .find(|x| x.1 == person1)
                    .unwrap()
                    .0,
            );
            for person2 in remaining3.clone() {
                let mut rem2 = remaining3.clone();
                rem2.remove(rem2.iter().enumerate().find(|x| x.1 == &person2).unwrap().0);
                let rem2_loop = rem2.clone();
                for person3 in rem2_loop {
                    let mut rem3 = rem2.clone();
                    rem3.remove(rem3.iter().enumerate().find(|x| x.1 == &person3).unwrap().0);
                    for person4 in rem3 {
                        let mut floor = Floor::new(conflicts.clone());
                        if floor.try_member(person1) {
                            floor.add_member(person1);
                            if floor.try_member(person2) {
                                floor.add_member(person2);
                                if floor.try_member(person3) {
                                    floor.add_member(person3);
                                    if floor.try_member(person4) {
                                        floor.add_member(person4);
                                        // seconds.push(floor);
                                        if !trimmed_thirds.iter().any(|x| x == &floor) {
                                            trimmed_thirds.push(floor.clone());
                                            floors.push((floor1.clone(), floor2.clone(), floor));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Conflict {
    person1: String,
    person2: String,
}

#[derive(Debug, Clone)]
struct Floor {
    population: u8,
    populous: Vec<&'static str>,
    conflicts: Vec<Conflict>,
}

impl Floor {
    fn new(conflicts: Vec<Conflict>) -> Floor {
        return Floor {
            population: 0,
            populous: Vec::new(),
            conflicts,
        };
    }

    fn try_member(&self, mem: &'static str) -> bool {
        let mut mems: Vec<&'static str> = self.populous.clone();
        mems.push(mem);
        return Floor::verify(self, mems);
    }

    fn add_member(&mut self, mem: &'static str) {
        self.populous.push(mem);
        self.population += 1;
    }

    fn verify(&self, members: Vec<&'static str>) -> bool {
        let members1 = members.clone();
        let members2 = members.clone();
        for member1 in members1 {
            for member2 in members2.clone() {
                if member1 != member2 {
                    let con = Conflict {
                        person1: member1.to_string(),
                        person2: member2.to_string(),
                    };
                    for conf in &self.conflicts[..] {
                        if &con == conf {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        for person1 in self.populous.clone() {
            let mut exists = false;
            for person2 in other.populous.clone() {
                if person1 == person2 {
                    exists = true;
                }
            }
            if !exists {
                return false;
            }
        }
        return true;
    }
}
