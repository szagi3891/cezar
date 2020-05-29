
fn main() {
    let offset = 3;

    let alfabet: Vec<char> = vec!(
        'a', 'ą', 'b', 'c', 'ć',
        'd', 'e', 'ę', 'f', 'g',
        'h', 'i', 'j', 'k', 'l',
        'm', 'n', 'ń', 'o', 'ó',
        'p', 'r', 's', 'ś', 't',
        'u', 'w', 'y', 'z', 'ź',
        'ż'
    );

    let alfabetLen: i16 = alfabet.len() as i16;

    let getChar = |index: i16| -> char {
        alfabet.get(index as usize).unwrap().clone()
    };

    let findIndex = |oneChar: char| -> i16 {
        for index in 0..alfabetLen {
            
            let current = getChar(index);

            if current == oneChar {
                return index;
            }
        }

        panic!("katastrofa -> {}", oneChar);
    };

    let encode = |oneChar: char| -> char {
        if oneChar == ' ' {
            return ' ';
        }

        let index = findIndex(oneChar);
        let newIndex = index + offset;

        let newIndex = if newIndex >= alfabetLen {
            newIndex - alfabetLen
        } else {
            newIndex
        };

        getChar(newIndex)
    };

    let decode = |oneChar: char| -> char {
        if oneChar == ' ' {
            return ' ';
        }

        let index = findIndex(oneChar);
        let newIndex = index - offset;

        let newIndex = if newIndex < 0 {
            newIndex + alfabetLen
        } else {
            newIndex
        };

        getChar(newIndex)
    };

    let encodeBlock = |text: &'static str| -> String {
        let mut out: Vec<String> = Vec::new();

        for textChar in text.chars() {
            out.push(format!("{}", encode(textChar)));
        }

        out.as_slice().join("")
    };

    let encodeBlockAndPrint = |text: &'static str| {
        //println!("Jawny tekst : {}", text);
        println!("Zaszyfrowany: {}", encodeBlock(text));
        println!("");
        println!("");
    };

    let decodeBlock = |text: &'static str| -> String {
        let mut out: Vec<String> = Vec::new();

        for textChar in text.chars() {
            out.push(format!("{}", decode(textChar)));
        }

        out.as_slice().join("")
    };

    let decodeBlockAndPrint = |text: &'static str| {
        //println!("Jawny tekst : {}", text);
        println!("Zdekodowany: {}", decodeBlock(text));
        println!("");
        println!("");
    };

    println!("Kodowanie, tablica znaków");
    println!("");

    for charOne in alfabet.iter() {
        println!("{} -> {}", charOne, decode(charOne.clone()));
    }

    println!("");
    println!("");

    encodeBlockAndPrint("idź spać");

    encodeBlockAndPrint("mamy dzisiaj bardzo deszczową pogodę");

    decodeBlockAndPrint("nrkcoęlh");
    // for textChar in textIn.chars() {

    //     let a: char = textChar;
    // }
}
