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
        let char_prob: Vec<HashMap<char, f32>> = (0..5).map(|i| {
            chars.iter().fold(HashMap::new(), |mut map, c| {
                map.insert(*c, contents.iter().filter(|w| w[i] == *c).count() as f32 / contents.len() as f32);
                map
            }) 
        }).collect();

        contents.sort_by_cached_key(|word| {
            -word.iter().enumerate().map(|(i, c)| 
                (char_prob[i].get(&c).unwrap_or(&0.0) * 100000.0) as i32 / word.iter().filter(|e| *e == c).count() as i32
            ).sum::<i32>()
        });

        println!("================================");
        println!("{} mögliche Wörter verbleibend", contents.len());
        println!("Empfehlungen:");

        let mut input = "n".to_string();
        let mut page = 0;
        while input == "n" && page < contents.len() / 16 + 1 {
            println!("Seite {} von {}. [n] show more", page + 1, (contents.len()-1)/16+1);
            for (i,w) in contents.iter().enumerate().skip(page * 16).take(16) {
                println!("{:2}: {}", i+1, w.iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
            }
            print!("Wähle ein wort [1.. | <word>]: ");
            std::io::stdout().flush().unwrap();
            input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_ascii_lowercase().to_string();
            page += 1;
        }
        if input == "n" {
            println!("Keine weiteren Wörter verfügbar");
            print!("Wähle ein wort [1.. | <word>]: ");
            std::io::stdout().flush().unwrap();
            input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_ascii_lowercase().to_string();
        }
        
        println!("input: {}", input);
        let best = if input.len() == 5 {
            input.chars().collect::<Vec<_>>()
        } else {
            let index = input.parse::<usize>().unwrap();
            contents[index-1].clone()
        };

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

        for (i,(c, b)) in input.iter().zip(best.into_iter()).enumerate() {
            match c {
                '1' => contents.retain(|w| !w.contains(&b)),
                '2' => contents.retain(|w| w.contains(&b) && w[i] != b),
                '3' => contents.retain(|w| w[i] == b),
                _ => println!("Ungültige Eingabe. Verhalten nicht definiert"),
            } 
        }
    }
    if contents.len() == 1 {
        println!("Das Wort ist: {}", contents.iter().next().unwrap().iter().fold(String::new(), |mut a, c| {a.push(*c); a}));
    } else {
        println!("Keine Lösung gefunden");
    }
}