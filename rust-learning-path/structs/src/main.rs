fn main() {
    let tuple = ("element", 9, true, 4.5, '💩');
    println!("El primer elemento es {}, el segundo {}, el tercero {}, no hay cuarto malo {} y el mejor una {} en una tupla. {}",
        tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, '🤠');


    // Some structs
    // Classic
    struct Student { name: String, level: u8, remote: bool }
    // Tuple
    struct Grades (char, char, char, char, f32);
    // Unit
    //struct Unit;

    let persona1 = Student {name: String::from("Arnulfo Valentierra"), remote: true, level: 2};
    let persona2 = Student {name: String::from("Prono Velasquez"), level: 5, remote: true };

    let mark_1 = Grades('A','💩','🤠','Z', 3.14);
    let mark_2 = Grades('B','P','Y','1', 2.68);


    println!("you are {}, your level is {}, the remote value is {}, and your average resume is {}", persona1.name, persona1.level, persona1.remote, mark_1.4);
    println!("you are {}, your level is {}, the remote value is {}, and your average resume is {}", persona2.name, persona2.level, persona2.remote, mark_2.4);


    #[derive(Debug)]
    struct  KeyPress(String, char);
    #[derive(Debug)]
    struct  MouseClick {x:i64, y:i64}

    #[derive(Debug)]
    enum WebEvent {WeLoad(bool), WEClick(MouseClick), WeKeys(KeyPress)}

    let we_load = WebEvent::WeLoad(true);

    let click = MouseClick { x: 100, y: 250};
    println!("click reading issue x: {} and y: {}",click.x,click.y);
    let we_click = WebEvent::WEClick(click);

    let keys = KeyPress(String::from("Ctrl+"),'A');
    let we_keys = WebEvent::WeKeys(keys);


    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",we_load, we_click, we_keys);
}
