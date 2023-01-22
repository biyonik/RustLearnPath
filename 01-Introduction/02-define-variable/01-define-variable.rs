fn main() {
    // Rust programlama dilinde değişkenler varsayılan immutable(değiştirilemez) olarak tanımlanır
    // Tanımlarken (initialize) anında değer atanır
    // Değişken tanımlama 'let' anahtar sözcüğü ile yapılır
    let num = 5;
    println!("x: {}", x);

    // 'mut' anahtar sözcüğü ile bir değişken mutable(değiştirilebilir) olarak tanımlanabilir
    let mut mutable_number = 11;
    println!("İlk değer: {}", mutable_number);
    mutable_number = 13;
    println!("İlk değer: {}", mutable_number);

}