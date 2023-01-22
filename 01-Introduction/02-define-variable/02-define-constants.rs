const CONSTANT_VARIABLE_FROM_OUTSIDE_MAIN_BLOCK: &str = "Bu sabit main bloğu dışında tanımlandı!";

fn main() {
    // Sabit tanımlamak için 'const' anahtar sözcüğü kullanılır
    // 'const' main bloğu dışında tanımlanabilir
    // const, içeriği değiştirilemeyen bir değişken tanımlamak için kullanılır. sabitin değeri bir kez tanımlanır, tipi ve değeri bilinir

    const MAX_POINT: u32 = 1000;
    const NAME: &str = "John Doe";

    println!("CONSTANT_VARIABLE_FROM_OUTSIDE_MAIN_BLOCK: {}", CONSTANT_VARIABLE_FROM_OUTSIDE_MAIN_BLOCK);
    println!("MAX_POINT: {}", MAX_POINT);
    println!("NAME: {}", NAME);
}