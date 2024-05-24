use std::{collections::{HashMap, HashSet}, io::Write};

fn main() {  
    let args = std::env::args().collect::<Vec<_>>();

    let mut contents: Vec<_> = std::fs::read_to_string(&args[1]).unwrap()
        .lines()
        .map(|word| word.to_lowercase().chars().collect::<Vec<_>>()) 
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
        
    let size = contents.len();
    contents = contents.into_iter()
        .filter(|word| word.len() == 5)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    if size != contents.len() {
        println!("Die Wörterliste wurde auf {} Wörter gefiltert", contents.len());
        print!("Änderungen speichern? [y/n]: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() == "y" {
            print!("Dateiname: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            contents.sort_unstable();
            std::fs::write(
                input.trim(), 
                contents.iter()
                    .map(| v | 
                        v.iter().fold(String::new(), |mut a, c| {a.push(*c); a})
                    )
                    .collect::<Vec<_>>()
                    .join("\n")
                ).unwrap();
            println!("Datei gespeichert. Name: {}", input.trim());
        }
    }

    println!("Es wurden {} Wörter mit 5 Buchstaben geladen", contents.len());

    loop {
        solve(contents.clone());
        print!("Neues Spiel starten? [y/n]: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.to_lowercase().trim() != "y" {
            break;
        }
    }
    
}

fn solve(mut contents: Vec<Vec<char>>) {
    while contents.len() > 1 {
        let chars: Vec<char> = contents.iter().flatten().collect::<HashSet<_>>().into_iter().cloned().collect();
        let mut char_count = vec![HashMap::new(); 5];
        for i in 0..5 {
            for c in chars.iter() {
                char_count[i].insert(c, contents.iter().filter(|w| w[i] == *c).count());
            }
        }
        let char_prob = char_count.into_iter()
            .map(| line |
                line.iter().map(|(&c,count)| (c, *count as f64 / contents.len() as f64)).collect::<HashMap<_,_>>()
            ).collect::<Vec<_>>();

        contents.sort_by_cached_key(|word| {
            let mut score = 0;
            for (i,c) in word.iter().enumerate() {
                score += (char_prob[i].get(&c).unwrap_or(&0.0) * 10000.0) as i32 / word.iter().filter(|e| *e == c).count() as i32;
            }
            -score
        });

        println!("================================");
        println!("{} mögliche Wörter verbleibend", contents.len());
        println!("Empfehlungen:");

        for (i,w) in contents.iter().enumerate().take(16) {
            println!("{:2}: {}", i+1, w.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
        }

        print!("Wähle ein wort [1-16]: ");
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>().unwrap();

        let best = contents[index-1].clone();

        println!("{} wurde gewählt. Welche Buchstaben waren korrekt? 1 -> Falsch, 2 -> Existiert, 3 -> Korrekt", best.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().chars().take(5).collect::<Vec<_>>();
        input = input.iter().enumerate()
            .map(|(j,c)| match c {
                '1' if input.iter().enumerate().any(|(i,c)| *c != '1' && best[j] == best[i]) => '2',
                c => *c,
            })
            .collect();

        for (i,c) in input.iter().enumerate() {
            match c {
                '1' => contents.retain(|w| !w.contains(&best[i])),
                '2' => contents.retain(|w| w.contains(&best[i]) && w[i] != best[i]),
                '3' => contents.retain(|w| w[i] == best[i]),
                _ => ()
            } 
        }
    }
    println!("Das Wort ist: {}", contents.iter().next().unwrap().iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
}