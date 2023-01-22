fn main() {
    // str bir veri tipidir. str değişkenler toplanamaz. &str şeklinde bir tanımlama referans tipe işaret eder
    // str, String değildir!

    // to_owned() -> sahinine göre değiştir demektir.

    let _x; // varsayılan olarak değeri yoktur, fakat bir sonraki adımda 6 değeri atandığı için tipi 'i32' olacaktır
    _x = 6;

    // İlk değeri A atandığı için tipi &str oldu. Burada ayrıca shadowing uygulanmış oldu. Daha önce tanımlanan ve değeri sonradan atanan x değişkeni tekrar tanımlanabildi
    let _x = "A";

    // Burada to_owned, x str olduğu için str tipine çevirdi
    println!("X: {}, {}", _x, _x.to_owned() + "BCD");

    let _x = 11;
    println!("X: {}, {}", _x, _x.to_owned() + 55);

    // Bir stringi, integer tipine çevirmek için parse() komutu kullanılır.
    // parse() metodu normalde bir result döner. Direkt tipe çevirmez
    let _x = "17";
    println!("X(str): {}, X(i32): {}, X+37: {}", _x, _x.parse::<i32>().unwrap(), _x.parse::<i32>().unwrap() + 37);
}